/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2024  Luis M. Santos, M.D.
 * Copyright (C) 2025  MedicalMasses L.L.C.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
 */

///
/// This module provides the basic types necessary to be able to handle connections and message
/// transmission in both synchronous and asynchronous contexts.
///
/// The types here should simplify implementation of higher level layers and protocols.
///
pub mod tcp {
    use crate::core::{RUMResult, RUMVec};
    use crate::strings::{rumtk_format, RUMString};
    pub use crate::threading::thread_primitives::*;
    use crate::threading::threading_manager::SafeTaskArgs;
    use crate::types::RUMOrderedMap;
    use crate::{
        rumtk_async_sleep, rumtk_create_task, rumtk_create_task_args, rumtk_init_threads,
        rumtk_resolve_task, rumtk_spawn_task, rumtk_wait_on_task,
    };
    use ahash::HashMapExt;
    use compact_str::ToCompactString;
    use std::collections::VecDeque;
    use std::sync::Arc;
    pub use tokio::net::{TcpListener, TcpStream};

    const MESSAGE_BUFFER_SIZE: usize = 1024;

    /// Convenience constant to localhost
    pub const LOCALHOST: &str = "127.0.0.1";
    /// Convenience constant for the `0.0.0.0` address. This is to be used in contexts in which you do not have any interface preference.
    pub const ANYHOST: &str = "0.0.0.0";
    pub const NET_SLEEP_TIMEOUT: f32 = 0.000001;
    pub const NET_RETRIES: usize = 100;

    pub type RUMNetMessage = RUMVec<u8>;
    pub type RUMNetResult<R> = RUMResult<R>;
    pub type ReceivedRUMNetMessage = (RUMString, RUMNetMessage);
    type RUMNetPartialMessage = (RUMNetMessage, bool);
    pub type ConnectionInfo = (RUMString, u16);

    ///
    /// This structs encapsulates the [tokio::net::TcpStream] instance that will be our adapter
    /// for connecting and sending messages to a peer or server.
    ///
    #[derive(Debug)]
    pub struct RUMClient {
        socket: TcpStream,
        disconnected: bool,
    }

    impl RUMClient {
        ///
        /// Connect to peer and construct the client.
        ///
        pub async fn connect(ip: &str, port: u16) -> RUMResult<RUMClient> {
            let addr = rumtk_format!("{}:{}", ip, port);
            match TcpStream::connect(addr.as_str()).await {
                Ok(socket) => Ok(RUMClient {
                    socket,
                    disconnected: false,
                }),
                Err(e) => Err(rumtk_format!(
                    "Unable to connect to {} because {}",
                    &addr.as_str(),
                    &e
                )),
            }
        }

        ///
        /// If a connection was already pre-established elsewhere, construct our client with the
        /// connected socket.
        ///
        pub async fn accept(socket: TcpStream) -> RUMResult<RUMClient> {
            Ok(RUMClient {
                socket,
                disconnected: false,
            })
        }

        ///
        /// Send message to server.
        ///
        pub async fn send(&mut self, msg: &RUMNetMessage) -> RUMResult<()> {
            if self.is_disconnected() {
                return Err(rumtk_format!(
                    "{} disconnected!",
                    &self.socket.peer_addr().unwrap().to_compact_string()
                ));
            }

            match self.socket.write_all(msg.as_slice()).await {
                Ok(_) => Ok(()),
                Err(e) => {
                    self.disconnect();
                    Err(rumtk_format!(
                        "Unable to send message to {} because {}",
                        &self.socket.local_addr().unwrap().to_compact_string(),
                        &e
                    ))
                }
            }
        }

        ///
        /// Receive message from server. This method will make calls to [RUMClient::recv_some]
        /// indefinitely until we have the full message or stop receiving any data.
        ///
        pub async fn recv(&mut self) -> RUMResult<RUMNetMessage> {
            let mut msg = RUMNetMessage::new();

            if self.is_disconnected() {
                return Err(rumtk_format!(
                    "{} disconnected!",
                    &self.socket.peer_addr().unwrap().to_compact_string()
                ));
            }

            loop {
                let mut fragment = self.recv_some().await?;
                msg.append(&mut fragment.0);
                if fragment.1 == false {
                    break;
                }
            }

            Ok(msg)
        }

        async fn recv_some(&mut self) -> RUMResult<RUMNetPartialMessage> {
            let mut buf: [u8; MESSAGE_BUFFER_SIZE] = [0; MESSAGE_BUFFER_SIZE];
            match self.socket.try_read(&mut buf) {
                Ok(n) => match n {
                    0 => {
                        self.disconnect();
                        Err(rumtk_format!(
                            "Received 0 bytes from {}! It might have disconnected!",
                            &self.socket.peer_addr().unwrap().to_compact_string()
                        ))
                    }
                    MESSAGE_BUFFER_SIZE => Ok((RUMNetMessage::from(buf), true)),
                    _ => Ok((RUMNetMessage::from(buf[0..n].to_vec()), false)),
                },
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    Ok((RUMNetMessage::new(), false))
                }
                Err(e) => {
                    self.disconnect();
                    Err(rumtk_format!(
                        "Error receiving message from {} because {}",
                        &self.socket.peer_addr().unwrap().to_compact_string(),
                        &e
                    ))
                }
            }
        }

        /// Returns the peer address:port as a string.
        pub async fn get_address(&self, local: bool) -> Option<RUMString> {
            match local {
                true => match self.socket.local_addr() {
                    Ok(addr) => Some(addr.to_compact_string()),
                    Err(_) => None,
                },
                false => match self.socket.peer_addr() {
                    Ok(addr) => Some(addr.to_compact_string()),
                    Err(_) => None,
                },
            }
        }

        pub fn is_disconnected(&self) -> bool {
            self.disconnected
        }

        pub fn disconnect(&mut self) {
            self.disconnected = true;
        }
    }

    /// List of clients that you can interact with.
    pub type ClientList = Vec<RUMNetClient>;
    /// List of client IDs that you can interact with.
    pub type ClientIDList = Vec<RUMString>;
    pub type RUMNetQueue<T> = VecDeque<T>;
    pub type RUMNetClient = Arc<AsyncRwLock<RUMClient>>;
    pub type RUMNetClients = Arc<AsyncRwLock<RUMOrderedMap<RUMString, RUMNetClient>>>;
    type SafeClientIDList = Arc<AsyncMutex<ClientIDList>>;
    pub type RUMNetClientMessageQueue<T> = RUMOrderedMap<RUMString, RUMNetQueue<T>>;
    pub type RUMNetMessageQueue<T> = Arc<AsyncRwLock<RUMNetClientMessageQueue<T>>>;
    pub type SafeListener = Arc<AsyncMutex<TcpListener>>;
    pub type SafeServer = Arc<AsyncRwLock<RUMServer>>;

    async fn lock_client_ex(client: &'_ RUMNetClient) -> AsyncRwLockWriteGuard<'_, RUMClient> {
        let locked = client.write().await;
        locked
    }

    async fn lock_client(client: &'_ RUMNetClient) -> AsyncRwLockReadGuard<'_, RUMClient> {
        let locked = client.read().await;
        locked
    }

    ///
    /// Enum used for selecting which clients to iterate through.
    /// Pass [SOCKET_READINESS_TYPE::NONE] to ignore filtering by readiness type.
    ///
    pub enum SOCKET_READINESS_TYPE {
        NONE,
        READ_READY,
        WRITE_READY,
        READWRITE_READY,
    }

    ///
    /// This is the Server primitive that listens for incoming connections and manages "low-level"
    /// messages.
    ///
    /// This struct tracks accepting new clients via [RUMServer::handle_accept], incoming messages
    /// via [RUMServer::handle_receive] and message dispatchs via [RUMServer::handle_send].
    ///
    /// All key methods are async and shall be run exclusively in the async context. We provide a
    /// set of tools that allow you to interact with this struct from sync code. One such tool is
    /// [RUMServerHandle].
    ///
    /// The [RUMServer::run] method orchestrate a series of steps that allows starting server
    /// management. The result is that the server will check for connections and messages
    /// autonomously. You want to call this method in a non blocking manner from the sync context,
    /// so that the server can handle the transactions in the background
    ///
    pub struct RUMServer {
        address: RUMString,
        clients: RUMNetClients,
    }

    impl RUMServer {
        ///
        /// Constructs a server and binds the `port` on interface denoted by `ip`. The server
        /// management is not started until you invoke [Self::run].
        ///
        pub async fn new(ip: &str, port: u16) -> RUMResult<RUMServer> {
            let mut address = rumtk_format!("{}:{}", ip, port);
            let tcp_listener_handle = match TcpListener::bind(address.as_str()).await {
                Ok(listener) => {
                    address = rumtk_format!("{}:{}", ip, listener.local_addr().unwrap().port());
                    listener
                }
                Err(e) => {
                    return Err(rumtk_format!(
                        "Unable to bind to {} because {}",
                        &address.as_str(),
                        &e
                    ))
                }
            };

            let client_list = RUMOrderedMap::<RUMString, RUMNetClient>::new();
            let clients = RUMNetClients::new(AsyncRwLock::new(client_list));
            let tcp_listener = Arc::new(AsyncMutex::new(tcp_listener_handle));

            //TODO: In the future, see if it is necessary to pass a oneshot channel and gracefully handle closure.
            //for now, it is ok to leak the handle and let process termination kill any future connections.
            tokio::spawn(Self::handle_accept(tcp_listener, clients.clone()));

            Ok(RUMServer { address, clients })
        }

        ///
        /// Contains basic logic for listening for incoming connections.
        ///
        pub async fn handle_accept(listener: SafeListener, clients: RUMNetClients) {
            #[allow(clippy::never_loop)]
            loop {
                match Self::_handle_accept(&listener, &clients).await {
                    Ok(_) => {}
                    Err(_) => {
                        //TODO: Log error accepting client...
                    }
                }
            }
        }

        pub async fn _handle_accept(
            listener: &SafeListener,
            clients: &RUMNetClients,
        ) -> RUMResult<()> {
            match listener.lock().await.accept().await {
                Ok((socket, _)) => {
                    let client = RUMClient::accept(socket).await?;
                    let client_id = match client.get_address(false).await {
                        Some(client_id) => client_id,
                        None => return Err(rumtk_format!("Accepted client returned no peer address. This should not be happening!"))
                    };
                    clients
                        .write()
                        .await
                        .insert(client_id, RUMNetClient::new(AsyncRwLock::new(client)));
                    Ok(())
                }
                Err(e) => Err(rumtk_format!(
                    "Error accepting incoming client! Error: {}",
                    e
                )),
            }
        }

        pub async fn receive(
            &self,
            client_id: &RUMString,
            blocking: bool,
        ) -> RUMResult<RUMNetMessage> {
            let client = self.get_client(client_id).await?;
            loop {
                let data = lock_client_ex(&client).await.recv().await?;

                if data.is_empty() && blocking {
                    continue;
                }

                return Ok(data);
            }
        }

        pub async fn send(&self, client_id: &RUMString, msg: &RUMNetMessage) -> RUMResult<()> {
            let client = self.get_client(client_id).await?;
            let mut err = RUMString::default();

            for _ in 0..NET_RETRIES {
                match lock_client_ex(&client).await.send(msg).await {
                    Ok(_) => return Ok(()),
                    Err(e) => {
                        err = e;
                        rumtk_async_sleep!(NET_SLEEP_TIMEOUT).await;
                        continue;
                    }
                }
            }

            Err(rumtk_format!(
                "Failed to send message after reaching retry limit of {}s because => {}",
                NET_RETRIES as f32 * NET_SLEEP_TIMEOUT,
                err
            ))
        }

        pub async fn disconnect(client: &RUMNetClient) {
            lock_client_ex(client).await.disconnect()
        }

        pub async fn get_client(&self, client: &RUMString) -> RUMResult<RUMNetClient> {
            match self.clients.read().await.get(client) {
                Some(client) => Ok(client.clone()),
                _ => Err(rumtk_format!("Client {} not found!", client)),
            }
        }

        ///
        /// Return client id list.
        ///
        pub async fn get_client_ids(&self) -> ClientIDList {
            self.clients
                .read()
                .await
                .keys()
                .cloned()
                .collect::<Vec<_>>()
        }

        pub async fn get_client_id(client: &RUMNetClient) -> RUMString {
            lock_client(client)
                .await
                .get_address(false)
                .await
                .expect("No address found! Malformed client")
        }

        ///
        /// Return list of clients.
        ///
        pub async fn get_clients(&self) -> ClientList {
            let ids = self.get_client_ids().await;
            let mut clients = ClientList::with_capacity(ids.len());
            for client_id in ids {
                clients.push(
                    self.clients
                        .read()
                        .await
                        .get(client_id.as_str())
                        .unwrap()
                        .clone(),
                );
            }
            clients
        }

        ///
        /// Get the Address:Port info for this socket.
        ///
        pub async fn get_address_info(&self) -> Option<RUMString> {
            Some(self.address.clone())
        }
    }

    ///
    /// Handle struct containing a reference to the global Tokio runtime and an instance of
    /// [RUMNetClient](RUMNetClient). This handle allows sync codebases to interact with the async primitives built
    /// on top of Tokio. Specifically, this handle allows wrapping of the async connect, send, and
    /// receive methods implemented in [RUMClient](RUMClient).
    ///
    pub struct RUMClientHandle {
        client: RUMNetClient,
    }

    type ClientSendArgs<'a> = (RUMNetClient, RUMNetMessage);
    type ClientReceiveArgs = RUMNetClient;

    impl RUMClientHandle {
        pub fn connect(ip: &str, port: u16) -> RUMResult<RUMClientHandle> {
            RUMClientHandle::new(ip, port)
        }

        pub fn new(ip: &str, port: u16) -> RUMResult<RUMClientHandle> {
            let con: ConnectionInfo = (RUMString::from(ip), port);
            let args = rumtk_create_task_args!(con);
            let client = rumtk_wait_on_task!(RUMClientHandle::new_helper, args)?;
            Ok(RUMClientHandle {
                client: RUMNetClient::new(AsyncRwLock::new(client?)),
            })
        }

        ///
        /// Queues a message send via the tokio runtime.
        ///
        pub fn send(&mut self, msg: RUMNetMessage) -> RUMResult<()> {
            let mut client_ref = Arc::clone(&self.client);
            let args = rumtk_create_task_args!((client_ref, msg));
            rumtk_wait_on_task!(RUMClientHandle::send_helper, args.clone())?
        }

        ///
        /// Checks if there are any messages received by the [RUMClient] via the tokio runtime.
        ///
        pub fn receive(&mut self) -> RUMResult<RUMNetMessage> {
            let client_ref = Arc::clone(&self.client);
            let args = rumtk_create_task_args!(client_ref);
            rumtk_wait_on_task!(RUMClientHandle::receive_helper, args.clone())?
        }

        /// Returns the peer address:port as a string.
        pub fn get_address(&self) -> Option<RUMString> {
            let client_ref = Arc::clone(&self.client);
            let args = rumtk_create_task_args!(client_ref);
            rumtk_wait_on_task!(RUMClientHandle::get_address_helper, args.clone())
                .unwrap_or_default()
        }

        async fn send_helper(args: SafeTaskArgs<ClientSendArgs<'_>>) -> RUMResult<()> {
            let lock_future = args.read();
            let locked_args = lock_future.await;
            let (client_lock_ref, msg) = locked_args.get(0).unwrap();
            let mut client_ref = Arc::clone(client_lock_ref);
            let mut client = client_ref.write().await;
            client.send(msg).await
        }

        async fn receive_helper(args: SafeTaskArgs<ClientReceiveArgs>) -> RUMResult<RUMNetMessage> {
            let lock_future = args.read();
            let locked_args = lock_future.await;
            let mut client_ref = locked_args.get(0).unwrap();
            let mut client = client_ref.write().await;
            client.recv().await
        }

        async fn new_helper(args: SafeTaskArgs<ConnectionInfo>) -> RUMNetResult<RUMClient> {
            let lock_future = args.read().await;
            let (ip, port) = match lock_future.get(0) {
                Some((ip, port)) => (ip, port),
                None => {
                    return Err(rumtk_format!(
                        "No IP address or port provided for connection!"
                    ))
                }
            };
            Ok(RUMClient::connect(ip, *port).await?)
        }
        async fn get_address_helper(args: SafeTaskArgs<ClientReceiveArgs>) -> Option<RUMString> {
            let locked_args = args.read().await;
            let client_ref = locked_args.get(0).unwrap();
            let mut client = client_ref.read().await;
            client.get_address(true).await
        }
    }

    ///
    /// Handle struct containing a reference to the global Tokio runtime and an instance of
    /// [SafeServer](SafeServer). This handle allows sync codebases to interact with the async primitives built
    /// on top of Tokio. Specifically, this handle allows wrapping of the async bind, send,
    /// receive, and start methods implemented in [RUMServer](RUMServer). In addition, this handle allows
    /// spinning a server in a fully non-blocking manner. Meaning, you can call start, which will
    /// immediately return after queueing the task in the tokio queue. You can then query the server
    /// for incoming data or submit your own data while the server is operating in the background.
    /// The server can be handling incoming data at the "same" time you are trying to queue your
    /// own message.
    ///
    pub struct RUMServerHandle {
        server: SafeServer,
    }

    type ServerSendArgs = (SafeServer, RUMString, RUMNetMessage);
    type ServerReceiveArgs = (SafeServer, RUMString);
    type ServerSelfArgs = SafeServer;

    impl RUMServerHandle {
        ///
        /// Constructs a [RUMServerHandle](RUMServerHandle) using the detected number of parallel units/threads on
        /// this machine. This method automatically binds to IP 0.0.0.0. Meaning, your server may
        /// become visible to the outside world.
        ///
        pub fn default(port: u16) -> RUMResult<RUMServerHandle> {
            RUMServerHandle::new(ANYHOST, port)
        }

        ///
        /// Constructs a [RUMServerHandle](RUMServerHandle) using the detected number of parallel units/threads on
        /// this machine. This method automatically binds to **localhost**. Meaning, your server
        /// remains private in your machine.
        ///
        pub fn default_local(port: u16) -> RUMResult<RUMServerHandle> {
            RUMServerHandle::new(LOCALHOST, port)
        }

        ///
        /// General purpose constructor for [RUMServerHandle](RUMServerHandle). It takes an ip and port and binds it.
        /// You can also control how many threads are spawned under the hood for this server handle.
        ///
        pub fn new(ip: &str, port: u16) -> RUMResult<RUMServerHandle> {
            let con: ConnectionInfo = (RUMString::from(ip), port);
            let args = rumtk_create_task_args!(con);
            let task_result = rumtk_wait_on_task!(RUMServerHandle::new_helper, &args)?;
            let server = task_result;
            Ok(RUMServerHandle {
                server: Arc::new(AsyncRwLock::new(server?)),
            })
        }

        ///
        /// Sync API method for queueing a message to send a client on the server.
        ///
        pub fn send(&mut self, client_id: &RUMString, msg: &RUMNetMessage) -> RUMResult<()> {
            let args = rumtk_create_task_args!((
                Arc::clone(&mut self.server),
                client_id.clone(),
                msg.clone()
            ));
            let task = rumtk_create_task!(RUMServerHandle::send_helper, args);
            match rumtk_resolve_task!(rumtk_spawn_task!(task)) {
                Ok(_) => Ok(()),
                Err(e) => Err(rumtk_format!("Failed to gc client because => {}", e)),
            }
        }

        //TODO: refactor the net items to look into the task result's result

        ///
        /// Sync API method for obtaining a single message from the server's incoming queue.
        /// Returns the next available [RUMNetMessage]
        ///
        pub fn receive(
            &mut self,
            client_id: &RUMString,
            blocking: bool,
        ) -> RUMResult<RUMNetMessage> {
            let args = rumtk_create_task_args!((Arc::clone(&self.server), client_id.clone()));
            rumtk_resolve_task!(RUMServerHandle::receive_helper(&args, blocking))?
        }

        ///
        /// Sync API method for obtaining the client list of the server.
        ///
        pub fn get_clients(&self) -> ClientList {
            let args = rumtk_create_task_args!((Arc::clone(&self.server)));
            rumtk_resolve_task!(RUMServerHandle::get_clients_helper(&args)).unwrap_or_default()
        }

        ///
        /// Sync API method for obtaining the client list of the server.
        ///
        pub fn get_client_ids(&self) -> ClientIDList {
            let args = rumtk_create_task_args!((Arc::clone(&self.server)));
            rumtk_resolve_task!(RUMServerHandle::get_client_ids_helper(&args)).unwrap_or_default()
        }

        ///
        /// Get the Address:Port info for this socket.
        ///
        pub fn get_address_info(&self) -> Option<RUMString> {
            let args = rumtk_create_task_args!(Arc::clone(&self.server));
            rumtk_resolve_task!(RUMServerHandle::get_address_helper(&args)).unwrap_or_default()
        }

        async fn send_helper(args: &SafeTaskArgs<ServerSendArgs>) -> RUMResult<()> {
            let owned_args = Arc::clone(args).clone();
            let locked_args = owned_args.read().await;
            let (server_ref, client_id, msg) = locked_args.get(0).unwrap();
            let result = server_ref.write().await.send(client_id, &msg).await?;
            Ok(result)
        }

        async fn receive_helper(
            args: &SafeTaskArgs<ServerReceiveArgs>,
            blocking: bool,
        ) -> RUMResult<RUMNetMessage> {
            let owned_args = Arc::clone(args).clone();
            let locked_args = owned_args.read().await;
            let (server_ref, client_id) = locked_args.get(0).unwrap();
            let msg = server_ref.write().await.receive(client_id, blocking).await;
            msg
        }

        async fn new_helper(args: &SafeTaskArgs<ConnectionInfo>) -> RUMNetResult<RUMServer> {
            let owned_args = Arc::clone(args);
            let lock_future = owned_args.read();
            let locked_args = lock_future.await;
            let (ip, port) = match locked_args.get(0) {
                Some((ip, port)) => (ip, port),
                None => {
                    return Err(rumtk_format!(
                        "No IP address or port provided for connection!"
                    ))
                }
            };
            Ok(RUMServer::new(ip, *port).await?)
        }

        async fn get_client_ids_helper(args: &SafeTaskArgs<ServerSelfArgs>) -> ClientIDList {
            let owned_args = Arc::clone(args).clone();
            let lock_future = owned_args.read();
            let locked_args = lock_future.await;
            let server_ref = locked_args.get(0).unwrap();
            let ids = server_ref.read().await.get_client_ids().await;
            ids
        }

        async fn get_clients_helper(args: &SafeTaskArgs<ServerSelfArgs>) -> ClientList {
            let owned_args = Arc::clone(args).clone();
            let lock_future = owned_args.read();
            let locked_args = lock_future.await;
            let server_ref = locked_args.get(0).unwrap();
            let clients = server_ref.read().await.get_clients().await;
            clients
        }

        async fn get_address_helper(args: &SafeTaskArgs<ServerSelfArgs>) -> Option<RUMString> {
            let owned_args = Arc::clone(args).clone();
            let locked_args = owned_args.read().await;
            let server_ref = locked_args.get(0).unwrap();
            let address = server_ref.read().await.get_address_info().await;
            address
        }
    }
}

pub mod tcp_helpers {
    use crate::net::tcp::ConnectionInfo;
    use crate::strings::RUMStringConversions;

    pub fn to_ip_port(address_str: &str) -> ConnectionInfo {
        let mut components = address_str.split(':');
        (
            components.next().unwrap_or_default().to_rumstring(),
            components
                .next()
                .unwrap_or("0")
                .parse::<u16>()
                .unwrap_or_default(),
        )
    }
}

///
/// This module provides the preferred API for interacting and simplifying work with the [tcp]
/// module's primitives.
///
/// The API here is defined in the form of macros!
///
pub mod tcp_macros {
    ///
    /// Macro for creating a server instance.
    ///
    /// If a `port` is passed, we return the default configured [tcp::RUMServerHandle] instance
    /// exposed to the world on all interfaces.
    ///
    /// If an `ip` and `port` is passed, we create an instance of [tcp::RUMServerHandle] bound
    /// to that ip/port combo using the default number of threads on the system which should match
    /// roughly to the number of cores/threads.
    ///
    /// Alternatively, you can pass the `ip`, `port`, and `threads`. In such a case, the constructed
    /// [tcp::RUMServerHandle] will use only the number of threads requested.
    ///
    #[macro_export]
    macro_rules! rumtk_create_server {
        ( $port:expr ) => {{
            use $crate::net::tcp::RUMServerHandle;
            RUMServerHandle::default($port)
        }};
        ( $ip:expr, $port:expr ) => {{
            use $crate::net::tcp::RUMServerHandle;
            RUMServerHandle::new($ip, $port)
        }};
    }

    ///
    /// Macro for starting the server. When a server is created, it does not start accepting clients
    /// right away. You need to call this macro to do that or call [tcp::RUMServerHandle::start]
    /// directly.
    ///
    /// The only argument that we expect is the `blocking` argument. If `blocking` is requested,
    /// calling this macro will block the calling thread. By default, we start the server in
    /// non-blocking mode so that you can do other actions in the calling thread like queueing
    /// messages.
    ///
    #[macro_export]
    macro_rules! rumtk_start_server {
        ( $server:expr ) => {{
            $server.start(false)
        }};
        ( $server:expr, $blocking:expr ) => {{
            $server.start($blocking)
        }};
    }

    ///
    /// This macro is a convenience macro that allows you to establish a connection to an endpoint.
    /// It creates and instance of [tcp::RUMClientHandle].
    ///
    /// If you only pass the `port`, we will connect to a server in *localhost* listening at that
    /// port.
    ///
    /// If you pass both `ip` and `port`, we will connect to a server listening at that ip/port
    /// combo.
    ///
    #[macro_export]
    macro_rules! rumtk_connect {
        ( $port:expr ) => {{
            use $crate::net::tcp::{RUMClientHandle, LOCALHOST};
            RUMClientHandle::connect(LOCALHOST, $port)
        }};
        ( $ip:expr, $port:expr ) => {{
            use $crate::net::tcp::RUMClientHandle;
            RUMClientHandle::connect($ip, $port)
        }};
    }

    ///
    /// Convenience macro for obtaining the ip and port off a string with format `ip:port`.
    ///
    /// # Example Usage
    ///
    /// ```
    /// use rumtk_core::{rumtk_create_server, rumtk_get_ip_port};
    ///
    /// let server = rumtk_create_server!(0).unwrap();
    /// let ip_addr_info = server.get_address_info().unwrap();
    /// let (ip, port) = rumtk_get_ip_port!(&ip_addr_info);
    /// assert!(port > 0, "Expected non-zero port!");
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_get_ip_port {
        ( $address_str:expr ) => {{
            use $crate::net::tcp_helpers::to_ip_port;
            to_ip_port(&$address_str)
        }};
    }
}
