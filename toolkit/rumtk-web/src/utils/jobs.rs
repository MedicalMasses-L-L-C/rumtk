/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D. <lsantos@medicalmasses.com>
 * Copyright (C) 2025  Ethan Dixon
 * Copyright (C) 2025  MedicalMasses L.L.C. <contact@medicalmasses.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::HTMLResult;
use rumtk_core::core::RUMResult;
use rumtk_core::id::id_to_uuid;
use rumtk_core::strings::rumtk_format;
use rumtk_core::threading::threading_manager::{Task, TaskID, TaskManager};
use rumtk_core::types::RUMBuffer;

pub type JobID = TaskID;
pub type JobBuffer = RUMBuffer;

pub type JobResult = RUMResult<Option<HTMLResult>>;
pub type Job = Task<JobResult>;
type JobManager = TaskManager<JobResult>;

static mut TASK_MANAGER: Option<JobManager> = None;

pub fn job_str_id_to_id(id: &str) -> RUMResult<JobID> {
    id_to_uuid(id)
}

pub fn init_job_manager(workers: &usize) -> RUMResult<()> {
    let manager = TaskManager::<JobResult>::new(workers)?;
    unsafe {
        TASK_MANAGER = Some(manager);
    }
    Ok(())
}

pub fn get_manager() -> RUMResult<&'static mut JobManager> {
    unsafe {
        match TASK_MANAGER.as_mut() {
            Some(m) => Ok(m),
            None => return Err(rumtk_format!("TaskManager is not initialized")),
        }
    }
}

#[macro_export]
macro_rules! rumtk_web_init_job_manager {
    ( $workers:expr ) => {{
        use $crate::jobs::init_job_manager;
        init_job_manager($workers)
    }};
}

#[macro_export]
macro_rules! rumtk_web_get_job_manager {
    (  ) => {{
        use $crate::jobs::get_manager;
        get_manager()
    }};
}

#[macro_export]
macro_rules! rumtk_web_generate_job_id {
    ( $id:expr ) => {{
        use $crate::jobs::job_str_id_to_id;
        job_str_id_to_id($id)
    }};
}

///
/// THis macro allows you to check if a background job has completed.
///
/// If the job has completed, return the result which is of type [JobResult].
///
/// If the job is still going, force render a drop in loader component set to retry the check. This
/// loader gets passed the calling element name (`$element_name`) so that it can render the results
/// as it sees fit.
///
/// ## Example
///
/// ### Loader Render
/// ```
/// use rumtk_core::{rumtk_async_sleep, rumtk_new_lock};
/// use rumtk_core::strings::{RUMString, ToCompactString};
/// use rumtk_web::utils::testdata::{JOB_LOADER_TEST_PATTERN};
/// use rumtk_web::defaults::{PARAMS_ID, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM, DEFAULT_NO_TEXT};
/// use rumtk_web::utils::jobs::{JobResult};
/// use rumtk_web::{HTMLResult, SharedAppState, URLParams, URLPath, AppState, RUMWebResponse, RUMWebData};
/// use rumtk_web::{rumtk_web_init_job_manager, rumtk_web_get_job_manager, rumtk_web_check_on_job, rumtk_web_get_text_item, rumtk_web_post_process_html, rumtk_web_init_components};
///
/// let workers: usize = 5;
/// rumtk_web_init_job_manager!(&workers);
/// rumtk_web_init_components!(
///     Some(vec![
///         ("my_element", my_element)
///     ])
/// );
///
/// async fn basic_processor() -> JobResult {
///     rumtk_async_sleep!(100).await;
///     Ok(JobResultType::TEXT(RUMString::new("Hello World")))
/// }
///
/// fn my_element(_path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
///     let job_id = rumtk_web_get_text_item!(params, PARAMS_ID, DEFAULT_NO_TEXT);
///     let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);
///
///     let job_result = rumtk_web_check_on_job!("my_element", job_id, state);
///
///     let job_data = match job_result {
///         JobResultType::TEXT(t) => t,
///         _ => RUMString::new("")
///     };
///
///     rumtk_web_post_process_html!(job_data)
/// }
///
/// let app_state = rumtk_new_lock!(AppState::default());
/// let mut params = RUMWebData::new();
/// let job_id = rumtk_web_get_job_manager!().unwrap().spawn_task(basic_processor()).unwrap();
/// params.insert(RUMString::from(PARAMS_ID), job_id.to_compact_string());
/// let rendered = my_element(&[], &params, app_state.clone()).unwrap().to_rumstring();
///
/// assert!(rendered.as_str().contains(JOB_LOADER_TEST_PATTERN), "Element did not render loader!");
///
/// ```
///
/// ### Component Render
/// ```
/// use rumtk_core::{rumtk_sleep, rumtk_new_lock};
/// use rumtk_core::strings::{RUMString, ToCompactString};
/// use rumtk_web::utils::testdata::{JOB_LOADER_TEST_PATTERN};
/// use rumtk_web::defaults::{PARAMS_ID, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM, DEFAULT_NO_TEXT};
/// use rumtk_web::utils::jobs::{JobResult, JobResultType};
/// use rumtk_web::{HTMLResult, SharedAppState, URLParams, URLPath, AppState, RUMWebResponse, RUMWebData};
/// use rumtk_web::{rumtk_web_init_job_manager, rumtk_web_get_job_manager, rumtk_web_check_on_job, rumtk_web_get_text_item, rumtk_web_post_process_html, rumtk_web_init_components};
///
/// const HELLO_STR: &str = "Hello World";
///
/// let workers: usize = 5;
/// rumtk_web_init_job_manager!(&workers);
/// rumtk_web_init_components!(
///     Some(vec![
///         ("my_element", my_element)
///     ])
/// );
///
/// async fn basic_processor() -> JobResult {
///     Ok(JobResultType::TEXT(RUMString::new(HELLO_STR)))
/// }
///
/// fn my_element(_path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
///     let job_id = rumtk_web_get_text_item!(params, PARAMS_ID, DEFAULT_NO_TEXT);
///     let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);
///
///     let job_result = rumtk_web_check_on_job!("my_element", job_id, state);
///
///     assert!(job_result.is_text(), "Job did not return the expected results! => {:?}", job_result);
///
///     let job_data = match job_result {
///         JobResultType::TEXT(t) => t,
///         _ => RUMString::new("")
///     };
///
///     assert!(job_data.as_str().contains(HELLO_STR), "Job data is missing expected string! Expected {}, Got {}", HELLO_STR, &job_data);
///
///     rumtk_web_post_process_html!(job_data)
/// }
///
/// let app_state = rumtk_new_lock!(AppState::default());
/// let mut params = RUMWebData::new();
/// let job_id = rumtk_web_get_job_manager!().unwrap().spawn_task(basic_processor()).unwrap();
/// params.insert(RUMString::from(PARAMS_ID), job_id.to_compact_string());
///
/// rumtk_sleep!(1);
/// let rendered = my_element(&[], &params, app_state.clone()).unwrap().to_rumstring();
///
/// assert!(rendered.is_empty(), "Element results survived the rendering process's filtering!");
///
/// ```
///
#[macro_export]
macro_rules! rumtk_web_check_on_job {
    ( $element_name:expr, $job_id:expr, $state:expr ) => {{
        use $crate::defaults::{DEFAULT_TEXT_ITEM};
        rumtk_web_check_on_job!($element_name, $job_id, DEFAULT_TEXT_ITEM, $state)
    }};
    ( $element_name:expr, $job_id:expr, $css_class:expr, $state:expr ) => {{
        use rumtk_core::id::id_to_uuid;
        use $crate::defaults::{PARAMS_CSS_CLASS, PARAMS_ELEMENT, PARAMS_ID};
        use $crate::{rumtk_web_get_job_manager, rumtk_web_render_component};

        let id = id_to_uuid($job_id)?;
        let job_finished = rumtk_web_get_job_manager!()?.is_finished(&id);
        let result = match job_finished {
            true => &rumtk_web_get_job_manager!()?.wait_on(&id)?.result,
            false => {
                return rumtk_web_render_component!(
                    "job_loader",
                    [
                        (PARAMS_ID, $job_id),
                        (PARAMS_ELEMENT, $element_name),
                        (PARAMS_CSS_CLASS, $css_class),
                    ], $state
                );
            },
        };

        match result {
            Some(r) => r.clone()?,
            None => None,
        }
    }};
}
