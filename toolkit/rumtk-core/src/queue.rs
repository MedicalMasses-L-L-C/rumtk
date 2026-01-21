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
pub mod queue {
    use crate::core::RUMResult;
    use crate::strings::RUMString;
    use crate::threading::thread_primitives::*;
    use crate::threading::threading_functions::async_sleep;
    use crate::{rumtk_init_threads, rumtk_resolve_task, rumtk_spawn_task, threading};
    use std::future::Future;
    use std::sync::Arc;

    pub const DEFAULT_SLEEP_DURATION: f32 = 5f32;
    pub const DEFAULT_QUEUE_CAPACITY: usize = 1024;
    pub const DEFAULT_MICROTASK_QUEUE_CAPACITY: usize = 5;

    pub struct TaskQueue<R> {
        task_handles: AsyncTaskHandles<R>,
        tasks: TaskTable<R>,
        runtime: SafeTokioRuntime,
    }

    impl<R> TaskQueue<R>
    where
        R: Sync + Send + Clone + 'static,
    {
        ///
        /// This method creates a [`TaskQueue`] instance using sensible defaults.
        ///
        /// The `threads` field is computed from the number of cores present in system.
        ///
        pub fn default() -> RUMResult<TaskQueue<R>> {
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
        pub fn new(worker_num: &usize) -> RUMResult<TaskQueue<R>> {
            let tasks = TaskTable::<R>::with_capacity(DEFAULT_QUEUE_CAPACITY);
            let task_handles = AsyncTaskHandles::with_capacity(DEFAULT_QUEUE_CAPACITY);
            let runtime = rumtk_init_threads!(&worker_num);
            Ok(TaskQueue {
                tasks,
                task_handles,
                runtime: runtime.clone(),
            })
        }

        ///
        /// Add a task to the processing queue. The idea is that you can queue a processor function
        /// and list of args that will be picked up by one of the threads for processing.
        ///
        pub fn add_task<F>(&mut self, task: F) -> TaskID
        where
            F: Future<Output = TaskResult<R>> + Send + Sync + 'static,
            F::Output: Send + Sized + 'static,
        {
            let id = TaskID::new_v4();
            let job_handle = rumtk_spawn_task!(&self.runtime, task);
            let task = Task {
                id: id.clone(),
                finished: false,
                result: Err(RUMString::default()),
                job: Arc::new(&job_handle),
            };
            self.task_handles.push(job_handle);
            self.tasks.insert(id.clone(), task);
            id
        }

        ///
        /// This method waits until all queued tasks have been processed from the main queue.
        ///
        /// We poll the status of the main queue every [`DEFAULT_SLEEP_DURATION`] ms.
        ///
        /// Upon completion,
        ///
        /// 1. We collect the results generated (if any).
        /// 2. We reset the main task and result internal queue states.
        /// 3. Return the list of results ([`TaskResults<R>`]).
        ///
        /// ### Note:
        /// ```text
        ///     Results returned here are not guaranteed to be in the same order as the order in which
        ///     the tasks were queued for work. You will need to pass a type as T that automatically
        ///     tracks its own id or has a way for you to resort results.
        /// ```
        pub fn wait(&mut self) -> TaskResults<R> {
            while !self.is_completed() {
                sleep(DEFAULT_SLEEP_DURATION);
            }

            let results = self.gather();
            self.reset();
            results
        }

        pub async fn wait_for(&self, task_id: &TaskID) {
            match self.tasks.get(task_id) {
                Some(task) => {
                    while !task.finished {
                        async_sleep(DEFAULT_SLEEP_DURATION).await;
                    }
                }
                None => {}
            }
        }

        pub async fn wait_for_batch(&self, tasks: &TaskBatch) {
            for task in tasks {
                self.wait_for(task).await;
            }
        }

        ///
        /// Check if all work has been completed from the task queue.
        ///
        /// This implementation is branchless.
        ///
        pub fn is_completed(&self) -> bool {
            let mut accumulator: usize = 0;

            if self.tasks.is_empty() {
                return false;
            }

            for task in self.tasks.iter() {
                accumulator += task.is_finished() as usize;
            }
            (accumulator / self.tasks.len()) > 0
        }

        ///
        /// Reset task queue and results queue states.
        ///
        pub fn reset(&mut self) {
            self.tasks.clear();
        }

        fn gather(&mut self) -> TaskResults<R> {
            let mut result_queue = TaskResults::<R>::with_capacity(self.tasks.len());
            for i in 0..self.tasks.len() {
                let task = self.tasks.pop().unwrap();
                result_queue.push(rumtk_resolve_task!(&self.runtime, task).unwrap());
            }
            result_queue
        }
    }
}

pub mod queue_macros {
    #[macro_export]
    macro_rules! rumtk_new_task_queue {
        ( $worker_num:expr ) => {{
            use $crate::queue::queue::TaskQueue;
            TaskQueue::new($worker_num);
        }};
    }
}
