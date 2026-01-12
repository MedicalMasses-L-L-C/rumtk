/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D.
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
use crate::utils::defaults::{DEFAULT_LOCAL_LISTENING_ADDRESS, DEFAULT_OUTBOUND_LISTENING_ADDRESS};
use crate::utils::matcher::*;
use crate::{rumtk_web_fetch, rumtk_web_load_conf};
use axum::routing::get;
use axum::Router;
use clap::Parser;
use rumtk_core::strings::RUMString;
use tower_http::compression::{CompressionLayer, DefaultPredicate};
use tower_http::services::ServeDir;
use tracing::error;

///
/// RUMTK WebApp CLI Args
///
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    ///
    /// Website title to use internally. It can be omitted if defined in the app.json config file
    /// bundled with your app.
    ///
    #[arg(short, long)]
    title: RUMString,
    ///
    /// Website description string. It can be omitted if defined in the app.json config file
    /// bundled with your app.
    ///
    #[arg(short, long)]
    description: RUMString,
    ///
    /// Is the interface meant to be bound to the loopback address and remain hidden from the
    /// outside world.
    ///
    /// It follows the format ```IPv4:port``` and it is a string.
    ///
    /// If a NIC IP is defined via `--ip`, that value will override this flag.
    ///
    #[arg(short, long, default_value = DEFAULT_LOCAL_LISTENING_ADDRESS)]
    ip: str,
}

pub async fn run_app(args: &Args) {
    let state = rumtk_web_load_conf!();
    let comression_layer: CompressionLayer = CompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true)
        .compress_when(DefaultPredicate::new());
    let app = Router::new()
        /* Robots.txt */
        .route("/robots.txt", get(rumtk_web_fetch!(default_robots_matcher)))
        /* Components */
        .route(
            "/component/{*name}",
            get(rumtk_web_fetch!(default_component_matcher)),
        )
        /* Pages */
        .route("/", get(rumtk_web_fetch!(default_page_matcher)))
        .route("/{*page}", get(rumtk_web_fetch!(default_page_matcher)))
        /* Services */
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state)
        .layer(comression_layer);

    let listener = tokio::net::TcpListener::bind(&args.ip)
        .await
        .expect("There was an issue biding the listener.");
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("There was an issue with the server.");
}

#[macro_export]
macro_rules! rumtk_web_run_app {
    (  ) => {{
        let args = Args::parse();
        run_app(args).await;
    }};
}
