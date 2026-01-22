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

///
/// This module provides all the primitives needed to build a multithreaded application.
///
pub mod thread_primitives {
    use crate::cache::{new_cache, LazyRUMCache};
    use std::sync::Arc;
    use tokio::runtime::Runtime as TokioRuntime;
    /**************************** Globals **************************************/
    pub static mut RT_CACHE: TokioRtCache = new_cache();
    /**************************** Helpers ***************************************/
    pub fn init_cache(threads: &usize) -> SafeTokioRuntime {
        let mut builder = tokio::runtime::Builder::new_multi_thread();
        builder.worker_threads(*threads);
        builder.enable_all();
        match builder.build() {
            Ok(handle) => Arc::new(handle),
            Err(e) => panic!(
                "Unable to initialize threading tokio runtime because {}!",
                &e
            ),
        }
    }

    /**************************** Types ***************************************/
    pub type SafeTokioRuntime = Arc<TokioRuntime>;
    pub type TokioRtCache = LazyRUMCache<usize, SafeTokioRuntime>;
}

pub mod threading_manager {
    use crate::cache::LazyRUMCacheValue;
    use crate::core::{RUMResult, RUMVec};
    use crate::strings::rumtk_format;
    use crate::threading::thread_primitives::SafeTokioRuntime;
    use crate::threading::threading_functions::async_sleep;
    use crate::types::{RUMHashMap, RUMID};
    use crate::{rumtk_init_threads, rumtk_resolve_task, threading};
    use std::future::Future;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use tokio::task::JoinHandle;

    const DEFAULT_SLEEP_DURATION: f32 = 0.001f32;
    const DEFAULT_TASK_CAPACITY: usize = 1024;

    pub type TaskItems<T> = RUMVec<T>;
    /// This type aliases a vector of T elements that will be used for passing arguments to the task processor.
    pub type TaskArgs<T> = TaskItems<T>;
    /// Function signature defining the interface of task processing logic.
    pub type SafeTaskArgs<T> = Arc<RwLock<TaskItems<T>>>;
    pub type AsyncTaskHandle<R> = JoinHandle<TaskResult<R>>;
    pub type AsyncTaskHandles<R> = Vec<AsyncTaskHandle<R>>;
    //pub type TaskProcessor<T, R, Fut: Future<Output = TaskResult<R>>> = impl FnOnce(&SafeTaskArgs<T>) -> Fut;
    pub type TaskID = RUMID;

    #[derive(Debug, Clone, Default)]
    pub struct Task<R> {
        pub id: TaskID,
        pub finished: bool,
        pub result: Option<R>,
    }

    pub type SafeTask<R> = Arc<Task<R>>;
    type SafeInternalTask<R> = Arc<RwLock<Task<R>>>;
    pub type TaskTable<R> = RUMHashMap<TaskID, SafeInternalTask<R>>;
    pub type SafeAsyncTaskTable<R> = Arc<RwLock<TaskTable<R>>>;
    pub type TaskBatch = RUMVec<TaskID>;
    /// Type to use to define how task results are expected to be returned.
    pub type TaskResult<R> = RUMResult<SafeTask<R>>;
    pub type TaskResults<R> = TaskItems<TaskResult<R>>;
    pub type TaskRuntime = LazyRUMCacheValue<SafeTokioRuntime>;

    ///
    /// Manages asynchronous tasks submitted as micro jobs from synchronous code. This type essentially
    /// gives the multithreading, asynchronous superpowers to synchronous logic.
    ///
    /// ## Example Usage
    ///
    /// ```
    /// use std::sync::{Arc};
    /// use tokio::sync::RwLock as AsyncRwLock;
    /// use rumtk_core::core::RUMResult;
    /// use rumtk_core::strings::RUMString;
    /// use rumtk_core::threading::threading_manager::{SafeTaskArgs, TaskItems, TaskManager};
    /// use rumtk_core::{rumtk_create_task, };
    ///
    /// let expected = vec![
    ///     RUMString::from("Hello"),
    ///     RUMString::from("World!"),
    ///     RUMString::from("Overcast"),
    ///     RUMString::from("and"),
    ///     RUMString::from("Sad"),
    ///  ];
    ///
    /// type TestResult = RUMResult<Vec<RUMString>>;
    /// let mut queue: TaskManager<TestResult> = TaskManager::new(&5).unwrap();
    ///
    /// let locked_args = AsyncRwLock::new(expected.clone());
    /// let task_args = SafeTaskArgs::<RUMString>::new(locked_args);
    /// let processor = rumtk_create_task!(
    ///     async |args: &SafeTaskArgs<RUMString>| -> TestResult {
    ///         let owned_args = Arc::clone(args);
    ///         let locked_args = owned_args.read().await;
    ///         let mut results = TaskItems::<RUMString>::with_capacity(locked_args.len());
    ///
    ///         for arg in locked_args.iter() {
    ///             results.push(RUMString::new(arg));
    ///         }
    ///
    ///         Ok(results)
    ///     },
    ///     task_args
    /// );
    ///
    /// queue.add_task::<_>(processor);
    /// let results = queue.wait();
    ///
    /// let mut result_data = Vec::<RUMString>::with_capacity(5);
    /// for r in results {
    ///     for v in r.unwrap().result.clone().unwrap().iter() {
    ///         for value in v.iter() {
    ///             result_data.push(value.clone());
    ///         }
    ///     }
    ///  }
    ///
    /// assert_eq!(result_data, expected, "Results do not match expected!");
    ///
    /// ```
    ///
    #[derive(Debug, Clone, Default)]
    pub struct TaskManager<R> {
        tasks: SafeAsyncTaskTable<R>,
        workers: usize,
    }

    impl<R> TaskManager<R>
    where
        R: Sync + Send + Clone + 'static,
    {
        ///
        /// This method creates a [`TaskQueue`] instance using sensible defaults.
        ///
        /// The `threads` field is computed from the number of cores present in system.
        ///
        pub fn default() -> RUMResult<TaskManager<R>> {
            Self::new(&threading::threading_functions::get_default_system_thread_count())
        }

        ///
        /// Creates an instance of [`ThreadedTaskQueue<T, R>`] in the form of [`SafeThreadedTaskQueue<T, R>`].
        /// Expects you to provide the count of threads to spawn and the microtask queue size
        /// allocated by each thread.
        ///
        /// This method calls [`Self::with_capacity()`] for the actual object creation.
        /// The main queue capacity is pre-allocated to [`DEFAULT_QUEUE_CAPACITY`].
        ///
        pub fn new(worker_num: &usize) -> RUMResult<TaskManager<R>> {
            let tasks = SafeAsyncTaskTable::<R>::new(RwLock::new(TaskTable::with_capacity(
                DEFAULT_TASK_CAPACITY,
            )));
            Ok(TaskManager::<R> {
                tasks,
                workers: worker_num.to_owned(),
            })
        }

        ///
        /// Add a task to the processing queue. The idea is that you can queue a processor function
        /// and list of args that will be picked up by one of the threads for processing.
        ///
        /// This is the async counterpart
        ///
        pub async fn add_task_async<F>(&mut self, task: F) -> TaskID
        where
            F: Future<Output = R> + Send + Sync + 'static,
            F::Output: Send + Sized + 'static,
        {
            let id = TaskID::new_v4();
            let mut safe_task = Arc::new(RwLock::new(Task::<R> {
                id: id.clone(),
                finished: false,
                result: None,
            }));
            self.tasks
                .write()
                .await
                .insert(id.clone(), safe_task.clone());

            let task_wrapper = async move || {
                // Run the task
                let result = task.await;

                // Cleanup task
                safe_task.write().await.result = Some(result);
                safe_task.write().await.finished = true;
            };

            tokio::spawn(task_wrapper());

            id
        }

        ///
        /// See [add_task](Self::add_task)
        ///
        pub fn add_task<F>(&mut self, task: F) -> TaskID
        where
            F: Future<Output = R> + Send + Sync + 'static,
            F::Output: Send + Sized + 'static,
        {
            let rt = rumtk_init_threads!(&self.workers);
            rumtk_resolve_task!(rt, self.add_task_async(task))
        }

        ///
        /// See [wait_async](Self::wait_async)
        ///
        pub fn wait(&mut self) -> TaskResults<R> {
            let rt = rumtk_init_threads!(&self.workers);
            rumtk_resolve_task!(rt, self.wait_async())
        }

        ///
        /// See [wait_on_batch_async](Self::wait_on_batch_async)
        ///
        pub fn wait_on_batch(&mut self, tasks: &TaskBatch) -> TaskResults<R> {
            let rt = rumtk_init_threads!(&self.workers);
            rumtk_resolve_task!(rt, self.wait_on_batch_async(&tasks))
        }

        ///
        /// See [wait_on_async](Self::wait_on_async)
        ///
        pub fn wait_on(&mut self, task_id: &TaskID) -> TaskResult<R> {
            let rt = rumtk_init_threads!(&self.workers);
            rumtk_resolve_task!(rt, self.wait_on_async(&task_id))
        }

        ///
        /// This method waits until a queued task with [TaskID](TaskID) has been processed from the main queue.
        ///
        /// We poll the status of the task every [DEFAULT_SLEEP_DURATION](DEFAULT_SLEEP_DURATION) ms.
        ///
        /// Upon completion,
        ///
        /// 2. Return the result ([TaskResults<R>](TaskResults)).
        ///
        /// This operation consumes the task.
        ///
        /// ### Note:
        /// ```text
        ///     Results returned here are not guaranteed to be in the same order as the order in which
        ///     the tasks were queued for work. You will need to pass a type as T that automatically
        ///     tracks its own id or has a way for you to resort results.
        /// ```
        pub async fn wait_on_async(&mut self, task_id: &TaskID) -> TaskResult<R> {
            let task = match self.tasks.write().await.remove(task_id) {
                Some(task) => task.clone(),
                None => return Err(rumtk_format!("No task with id {}", task_id)),
            };

            while !task.read().await.finished {
                async_sleep(DEFAULT_SLEEP_DURATION).await;
            }

            let x = Ok(Arc::new(task.write().await.clone()));
            x
        }

        ///
        /// This method waits until a set of queued tasks with [TaskID](TaskID) has been processed from the main queue.
        ///
        /// We poll the status of the task every [DEFAULT_SLEEP_DURATION](DEFAULT_SLEEP_DURATION) ms.
        ///
        /// Upon completion,
        ///
        /// 1. We collect the results generated (if any).
        /// 2. Return the list of results ([TaskResults<R>](TaskResults)).
        ///
        /// ### Note:
        /// ```text
        ///     Results returned here are not guaranteed to be in the same order as the order in which
        ///     the tasks were queued for work. You will need to pass a type as T that automatically
        ///     tracks its own id or has a way for you to resort results.
        /// ```
        pub async fn wait_on_batch_async(&mut self, tasks: &TaskBatch) -> TaskResults<R> {
            let mut results = TaskResults::<R>::default();
            for task in tasks {
                results.push(self.wait_on_async(task).await);
            }
            results
        }

        ///
        /// This method waits until all queued tasks have been processed from the main queue.
        ///
        /// We poll the status of the main queue every [DEFAULT_SLEEP_DURATION](DEFAULT_SLEEP_DURATION) ms.
        ///
        /// Upon completion,
        ///
        /// 1. We collect the results generated (if any).
        /// 2. We reset the main task and result internal queue states.
        /// 3. Return the list of results ([TaskResults<R>](TaskResults)).
        ///
        /// This operation consumes all the tasks.
        ///
        /// ### Note:
        /// ```text
        ///     Results returned here are not guaranteed to be in the same order as the order in which
        ///     the tasks were queued for work. You will need to pass a type as T that automatically
        ///     tracks its own id or has a way for you to resort results.
        /// ```
        pub async fn wait_async(&mut self) -> TaskResults<R> {
            let task_batch = self.tasks.read().await.keys().cloned().collect::<Vec<_>>();
            self.wait_on_batch_async(&task_batch).await
        }

        ///
        /// Check if all work has been completed from the task queue.
        ///
        /// ## Examples
        ///
        /// ### Sync Usage
        ///
        ///```
        /// use rumtk_core::threading::threading_manager::TaskManager;
        ///
        /// let manager = TaskManager::<usize>::new(&4).unwrap();
        ///
        /// let all_done = manager.is_all_completed();
        ///
        /// assert_eq!(all_done, true, "Empty TaskManager reports tasks are not completed!");
        ///
        /// ```
        ///
        pub fn is_all_completed(&self) -> bool {
            let rt = rumtk_init_threads!(&self.workers);
            rumtk_resolve_task!(rt, TaskManager::<R>::is_all_completed_async(self))
        }

        pub async fn is_all_completed_async(&self) -> bool {
            for (_, task) in self.tasks.read().await.iter() {
                if !task.read().await.finished {
                    return false;
                }
            }

            true
        }

        ///
        /// Alias for [wait](TaskManager::wait).
        ///
        fn gather(&mut self) -> TaskResults<R> {
            self.wait()
        }
    }
}

///
/// This module contains a few helper.
///
/// For example, you can find a function for determining number of threads available in system.
/// The sleep family of functions are also here.
///
pub mod threading_functions {
    use num_cpus;
    use std::thread::{available_parallelism, sleep as std_sleep};
    use std::time::Duration;
    use tokio::time::sleep as tokio_sleep;

    pub const NANOS_PER_SEC: u64 = 1000000000;
    pub const MILLIS_PER_SEC: u64 = 1000;
    pub const MICROS_PER_SEC: u64 = 1000000;

    pub fn get_default_system_thread_count() -> usize {
        let cpus: usize = num_cpus::get();
        let parallelism = match available_parallelism() {
            Ok(n) => n.get(),
            Err(_) => 0,
        };

        if parallelism >= cpus {
            parallelism
        } else {
            cpus
        }
    }

    pub fn sleep(s: f32) {
        let ns = s * NANOS_PER_SEC as f32;
        let rounded_ns = ns.round() as u64;
        let duration = Duration::from_nanos(rounded_ns);
        std_sleep(duration);
    }

    pub async fn async_sleep(s: f32) {
        let ns = s * NANOS_PER_SEC as f32;
        let rounded_ns = ns.round() as u64;
        let duration = Duration::from_nanos(rounded_ns);
        tokio_sleep(duration).await;
    }
}

///
/// Main API for interacting with the threading back end. Remember, we use tokio as our executor.
/// This means that by default, all jobs sent to the thread pool have to be async in nature.
/// These macros make handling of these jobs at the sync/async boundary more convenient.
///
pub mod threading_macros {
    use crate::threading::thread_primitives;
    use crate::threading::threading_manager::SafeTaskArgs;

    ///
    /// First, let's make sure we have *tokio* initialized at least once. The runtime created here
    /// will be saved to the global context so the next call to this macro will simply grab a
    /// reference to the previously initialized runtime.
    ///
    /// Passing nothing will default to initializing a runtime using the default number of threads
    /// for this system. This is typically equivalent to number of cores/threads for your CPU.
    ///
    /// Passing `threads` number will yield a runtime that allocates that many threads.
    ///
    ///
    /// ## Examples
    ///
    /// ```
    ///     use rumtk_core::{rumtk_init_threads, rumtk_resolve_task, rumtk_create_task_args, rumtk_create_task, rumtk_spawn_task};
    ///     use rumtk_core::core::RUMResult;
    ///     use rumtk_core::threading::threading_manager::SafeTaskArgs;
    ///
    ///     async fn test(args: &SafeTaskArgs<i32>) -> RUMResult<Vec<i32>> {
    ///         let mut result = Vec::<i32>::new();
    ///         for arg in args.read().await.iter() {
    ///             result.push(*arg);
    ///         }
    ///         Ok(result)
    ///     }
    ///
    ///     let rt = rumtk_init_threads!();                                      // Creates runtime instance
    ///     let args = rumtk_create_task_args!(1);                               // Creates a vector of i32s
    ///     let task = rumtk_create_task!(test, args);                           // Creates a standard task which consists of a function or closure accepting a Vec<T>
    ///     let result = rumtk_resolve_task!(&rt, rumtk_spawn_task!(&rt, task)); // Spawn's task and waits for it to conclude.
    /// ```
    ///
    /// ```
    ///     use rumtk_core::{rumtk_init_threads, rumtk_resolve_task, rumtk_create_task_args, rumtk_create_task, rumtk_spawn_task};
    ///     use rumtk_core::core::RUMResult;
    ///     use rumtk_core::threading::threading_manager::SafeTaskArgs;
    ///
    ///     async fn test(args: &SafeTaskArgs<i32>) -> RUMResult<Vec<i32>> {
    ///         let mut result = Vec::<i32>::new();
    ///         for arg in args.read().await.iter() {
    ///             result.push(*arg);
    ///         }
    ///         Ok(result)
    ///     }
    ///
    ///     let thread_count: usize = 10;
    ///     let rt = rumtk_init_threads!(&thread_count);
    ///     let args = rumtk_create_task_args!(1);
    ///     let task = rumtk_create_task!(test, args);
    ///     let result = rumtk_resolve_task!(&rt, rumtk_spawn_task!(&rt, task));
    /// ```
    #[macro_export]
    macro_rules! rumtk_init_threads {
        ( ) => {{
            use $crate::rumtk_cache_fetch;
            use $crate::threading::thread_primitives::{init_cache, RT_CACHE};
            use $crate::threading::threading_functions::get_default_system_thread_count;
            let rt = rumtk_cache_fetch!(
                &mut RT_CACHE,
                &get_default_system_thread_count(),
                init_cache
            );
            rt
        }};
        ( $threads:expr ) => {{
            use $crate::rumtk_cache_fetch;
            use $crate::threading::thread_primitives::{init_cache, RT_CACHE};
            let rt = rumtk_cache_fetch!(&raw mut RT_CACHE, $threads, init_cache);
            rt
        }};
    }

    ///
    /// Puts task onto the runtime queue.
    ///
    /// The parameters to this macro are a reference to the runtime (`rt`) and a future (`func`).
    ///
    /// The return is a [thread_primitives::JoinHandle<T>] instance. If the task was a standard
    /// framework task, you will get [thread_primitives::AsyncTaskHandle] instead.
    ///
    #[macro_export]
    macro_rules! rumtk_spawn_task {
        ( $func:expr ) => {{
            let rt = rumtk_init_threads!();
            rt.spawn($func)
        }};
        ( $rt:expr, $func:expr ) => {{
            $rt.spawn($func)
        }};
    }

    ///
    /// Using the initialized runtime, wait for the future to resolve in a thread blocking manner!
    ///
    /// If you pass a reference to the runtime (`rt`) and an async closure (`func`), we await the
    /// async closure without passing any arguments.
    ///
    /// You can pass a third argument to this macro in the form of any number of arguments (`arg_item`).
    /// In such a case, we pass those arguments to the call on the async closure and await on results.
    ///
    #[macro_export]
    macro_rules! rumtk_wait_on_task {
        ( $rt:expr, $func:expr ) => {{
            $rt.block_on(async move {
                $func().await
            })
        }};
        ( $rt:expr, $func:expr, $($arg_items:expr),+ ) => {{
            $rt.block_on(async move {
                $func($($arg_items),+).await
            })
        }};
    }

    ///
    /// This macro awaits a future.
    ///
    /// The arguments are a reference to the runtime (`rt) and a future.
    ///
    /// If there is a result, you will get the result of the future.
    ///
    /// ## Examples
    ///
    /// ```
    ///     use rumtk_core::{rumtk_init_threads, rumtk_resolve_task, rumtk_create_task_args, rumtk_create_task, rumtk_spawn_task};
    ///     use rumtk_core::core::RUMResult;
    ///     use rumtk_core::threading::threading_manager::SafeTaskArgs;
    ///
    ///     async fn test(args: &SafeTaskArgs<i32>) -> RUMResult<Vec<i32>> {
    ///         let mut result = Vec::<i32>::new();
    ///         for arg in args.read().await.iter() {
    ///             result.push(*arg);
    ///         }
    ///         Ok(result)
    ///     }
    ///
    ///     let rt = rumtk_init_threads!();
    ///     let args = rumtk_create_task_args!(1);
    ///     let task = rumtk_create_task!(test, args);
    ///     let result = rumtk_resolve_task!(&rt, rumtk_spawn_task!(&rt, task));
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_resolve_task {
        ( $rt:expr, $future:expr ) => {{
            use $crate::strings::rumtk_format;
            // Fun tidbit, the expression rumtk_resolve_task!(&rt, rumtk_spawn_task!(&rt, task)), where
            // rt is the tokio runtime yields async move { { &rt.spawn(task) } }. However, the whole thing
            // is technically moved into the async closure and captured so things like mutex guards
            // technically go out of the outer scope. As a result that expression fails to compile even
            // though the intent is for rumtk_spawn_task to resolve first and its result get moved
            // into the async closure. To ensure that happens regardless of given expression, we do
            // a variable assignment below to force the "future" macro expressions to resolve before
            // moving into the closure. DO NOT REMOVE OR "SIMPLIFY" THE let future = $future LINE!!!
            let future = $future;
            $rt.block_on(async move { future.await })
        }};
    }

    ///
    /// This macro creates an async body that calls the async closure and awaits it.
    ///
    /// ## Example
    ///
    /// ```
    /// use std::sync::{Arc, RwLock};
    /// use tokio::sync::RwLock as AsyncRwLock;
    /// use rumtk_core::strings::RUMString;
    /// use rumtk_core::threading::threading_manager::{SafeTaskArgs, TaskItems};
    ///
    /// pub type SafeTaskArgs2<T> = Arc<RwLock<TaskItems<T>>>;
    /// let expected = vec![
    ///     RUMString::from("Hello"),
    ///     RUMString::from("World!"),
    ///     RUMString::from("Overcast"),
    ///     RUMString::from("and"),
    ///     RUMString::from("Sad"),
    ///  ];
    /// let locked_args = AsyncRwLock::new(expected.clone());
    /// let task_args = SafeTaskArgs::<RUMString>::new(locked_args);
    ///
    ///
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_create_task {
        ( $func:expr ) => {{
            async move {
                let f = $func;
                f().await
            }
        }};
        ( $func:expr, $args:expr ) => {{
            let f = $func;
            async move { f(&$args).await }
        }};
    }

    ///
    /// Creates an instance of [SafeTaskArgs] with the arguments passed.
    ///
    /// ## Note
    ///
    /// All arguments must be of the same type
    ///
    #[macro_export]
    macro_rules! rumtk_create_task_args {
        ( ) => {{
            use $crate::threading::threading_manager::{TaskArgs, SafeTaskArgs, TaskItems};
            use tokio::sync::RwLock;
            SafeTaskArgs::new(RwLock::new(vec![]))
        }};
        ( $($args:expr),+ ) => {{
            use $crate::threading::threading_manager::{SafeTaskArgs};
            use tokio::sync::RwLock;
            SafeTaskArgs::new(RwLock::new(vec![$($args),+]))
        }};
    }

    ///
    /// Convenience macro for packaging the task components and launching the task in one line.
    ///
    /// One of the advantages is that you can generate a new `tokio` runtime by specifying the
    /// number of threads at the end. This is optional. Meaning, we will default to the system's
    /// number of threads if that value is not specified.
    ///
    /// Between the `func` parameter and the optional `threads` parameter, you can specify a
    /// variable number of arguments to pass to the task. each argument must be of the same type.
    /// If you wish to pass different arguments with different types, please define an abstract type
    /// whose underlying structure is a tuple of items and pass that instead.
    ///
    /// ## Examples
    ///
    /// ### With Default Thread Count
    /// ```
    ///     use rumtk_core::{rumtk_exec_task};
    ///     use rumtk_core::core::RUMResult;
    ///     use rumtk_core::threading::threading_manager::SafeTaskArgs;
    ///
    ///     async fn test(args: &SafeTaskArgs<i32>) -> RUMResult<Vec<i32>> {
    ///         let mut result = Vec::<i32>::new();
    ///         for arg in args.read().await.iter() {
    ///             result.push(*arg);
    ///         }
    ///         Ok(result)
    ///     }
    ///
    ///     let result = rumtk_exec_task!(test, vec![5]);
    ///     assert_eq!(&result.clone().unwrap(), &vec![5], "Results mismatch");
    ///     assert_ne!(&result.clone().unwrap(), &vec![5, 10], "Results do not mismatch as expected!");
    /// ```
    ///
    /// ### With Custom Thread Count
    /// ```
    ///     use rumtk_core::{rumtk_exec_task};
    ///     use rumtk_core::core::RUMResult;
    ///     use rumtk_core::threading::threading_manager::SafeTaskArgs;
    ///
    ///     async fn test(args: &SafeTaskArgs<i32>) -> RUMResult<Vec<i32>> {
    ///         let mut result = Vec::<i32>::new();
    ///         for arg in args.read().await.iter() {
    ///             result.push(*arg);
    ///         }
    ///         Ok(result)
    ///     }
    ///
    ///     let result = rumtk_exec_task!(test, vec![5], 5);
    ///     assert_eq!(&result.clone().unwrap(), &vec![5], "Results mismatch");
    ///     assert_ne!(&result.clone().unwrap(), &vec![5, 10], "Results do not mismatch as expected!");
    /// ```
    ///
    /// ### With Async Function Body
    /// ```
    ///     use rumtk_core::{rumtk_exec_task};
    ///     use rumtk_core::core::RUMResult;
    ///     use rumtk_core::threading::threading_manager::SafeTaskArgs;
    ///
    ///     let result = rumtk_exec_task!(
    ///     async move |args: &SafeTaskArgs<i32>| -> RUMResult<Vec<i32>> {
    ///         let mut result = Vec::<i32>::new();
    ///         for arg in args.read().await.iter() {
    ///             result.push(*arg);
    ///         }
    ///         Ok(result)
    ///     },
    ///     vec![5]);
    ///     assert_eq!(&result.clone().unwrap(), &vec![5], "Results mismatch");
    ///     assert_ne!(&result.clone().unwrap(), &vec![5, 10], "Results do not mismatch as expected!");
    /// ```
    ///
    /// ### With Async Function Body and No Args
    /// ```
    ///     use rumtk_core::{rumtk_exec_task};
    ///     use rumtk_core::core::RUMResult;
    ///     use rumtk_core::threading::threading_manager::SafeTaskArgs;
    ///
    ///     let result = rumtk_exec_task!(
    ///     async || -> RUMResult<Vec<i32>> {
    ///         let mut result = Vec::<i32>::new();
    ///         Ok(result)
    ///     });
    ///     let empty = Vec::<i32>::new();
    ///     assert_eq!(&result.clone().unwrap(), &empty, "Results mismatch");
    ///     assert_ne!(&result.clone().unwrap(), &vec![5, 10], "Results do not mismatch as expected!");
    /// ```
    ///
    /// ## Equivalent To
    ///
    /// ```
    ///     use rumtk_core::{rumtk_init_threads, rumtk_resolve_task, rumtk_create_task_args, rumtk_create_task, rumtk_spawn_task};
    ///     use rumtk_core::core::RUMResult;
    ///     use rumtk_core::threading::threading_manager::SafeTaskArgs;
    ///
    ///     async fn test(args: &SafeTaskArgs<i32>) -> RUMResult<Vec<i32>> {
    ///         let mut result = Vec::<i32>::new();
    ///         for arg in args.read().await.iter() {
    ///             result.push(*arg);
    ///         }
    ///         Ok(result)
    ///     }
    ///
    ///     let rt = rumtk_init_threads!();
    ///     let args = rumtk_create_task_args!(1);
    ///     let task = rumtk_create_task!(test, args);
    ///     let result = rumtk_resolve_task!(&rt, rumtk_spawn_task!(&rt, task));
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_exec_task {
        ($func:expr ) => {{
            use tokio::sync::RwLock;
            use $crate::{
                rumtk_create_task, rumtk_create_task_args, rumtk_init_threads, rumtk_resolve_task,
            };
            let rt = rumtk_init_threads!();
            let task = rumtk_create_task!($func);
            rumtk_resolve_task!(&rt, task)
        }};
        ($func:expr, $args:expr ) => {{
            use tokio::sync::RwLock;
            use $crate::{
                rumtk_create_task, rumtk_create_task_args, rumtk_init_threads, rumtk_resolve_task,
            };
            let rt = rumtk_init_threads!();
            let args = SafeTaskArgs::new(RwLock::new($args));
            let task = rumtk_create_task!($func, args);
            rumtk_resolve_task!(&rt, task)
        }};
        ($func:expr, $args:expr , $threads:expr ) => {{
            use tokio::sync::RwLock;
            use $crate::{
                rumtk_create_task, rumtk_create_task_args, rumtk_init_threads, rumtk_resolve_task,
            };
            let rt = rumtk_init_threads!(&$threads);
            let args = SafeTaskArgs::new(RwLock::new($args));
            let task = rumtk_create_task!($func, args);
            rumtk_resolve_task!(&rt, task)
        }};
    }

    ///
    /// Sleep a duration of time in a sync context, so no await can be call on the result.
    ///
    /// You can pass any value that can be cast to f32.
    ///
    /// The precision is up to nanoseconds and it is depicted by the number of decimal places.
    ///
    /// ## Examples
    ///
    /// ```
    ///     use rumtk_core::rumtk_sleep;
    ///     rumtk_sleep!(1);           // Sleeps for 1 second.
    ///     rumtk_sleep!(0.001);       // Sleeps for 1 millisecond
    ///     rumtk_sleep!(0.000001);    // Sleeps for 1 microsecond
    ///     rumtk_sleep!(0.000000001); // Sleeps for 1 nanosecond
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_sleep {
        ( $dur:expr) => {{
            use $crate::threading::threading_functions::sleep;
            sleep($dur as f32)
        }};
    }

    ///
    /// Sleep for some duration of time in an async context. Meaning, we can be awaited.
    ///
    /// You can pass any value that can be cast to f32.
    ///
    /// The precision is up to nanoseconds and it is depicted by the number of decimal places.
    ///
    /// ## Examples
    ///
    /// ```
    ///     use rumtk_core::{rumtk_async_sleep, rumtk_exec_task};
    ///     use rumtk_core::core::RUMResult;
    ///     rumtk_exec_task!( async || -> RUMResult<()> {
    ///             rumtk_async_sleep!(1).await;           // Sleeps for 1 second.
    ///             rumtk_async_sleep!(0.001).await;       // Sleeps for 1 millisecond
    ///             rumtk_async_sleep!(0.000001).await;    // Sleeps for 1 microsecond
    ///             rumtk_async_sleep!(0.000000001).await; // Sleeps for 1 nanosecond
    ///             Ok(())
    ///         }
    ///     );
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_async_sleep {
        ( $dur:expr) => {{
            use $crate::threading::threading_functions::async_sleep;
            async_sleep($dur as f32)
        }};
    }

    ///
    ///
    ///
    #[macro_export]
    macro_rules! rumtk_new_task_queue {
        ( $worker_num:expr ) => {{
            use $crate::threading::threading_manager::TaskManager;
            TaskManager::new($worker_num);
        }};
    }
}
