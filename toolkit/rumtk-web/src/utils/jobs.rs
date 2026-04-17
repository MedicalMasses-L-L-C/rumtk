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
use rumtk_core::core::RUMResult;
use rumtk_core::id::id_to_uuid;
use rumtk_core::strings::rumtk_format;
use rumtk_core::strings::RUMString;
use rumtk_core::threading::threading_manager::{Task, TaskID, TaskManager};
use rumtk_core::types::RUMBuffer;

pub type JobID = TaskID;
pub type JobBuffer = RUMBuffer;

#[derive(Default, Debug, Clone)]
pub enum JobResultType<T = RUMString> {
    File(JobBuffer),
    JSON(RUMString),
    TEXT(RUMString),
    RustType(T),
    #[default]
    NONE,
}

pub type JobResult = RUMResult<JobResultType>;
pub type Job = Task<JobResult>;
type JobManager = TaskManager<JobResult>;

static mut TASK_MANAGER: Option<JobManager> = None;

pub fn job_str_id_to_id(id: &str) -> JobID {
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
