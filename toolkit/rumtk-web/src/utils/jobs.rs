/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D.
 * Copyright (C) 2025  Nick Stephenson
 * Copyright (C) 2025  Ethan Dixon
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
use rumtk_core::core::RUMResult;
use rumtk_core::id::id_to_uuid;
use rumtk_core::threading::threading_manager::{Task, TaskID, TaskManager};
use rumtk_core::types::RUMBuffer;

pub type JobID = TaskID;
pub type JobBuffer = RUMBuffer;
pub type Job = Task<JobBuffer>;
type JobManager = TaskManager<JobBuffer>;

static mut TASK_MANAGER: Option<JobManager> = None;

pub fn job_str_id_to_id(id: &str) -> JobID {
    id_to_uuid(id)
}

pub fn init_job_manager(workers: &usize) -> RUMResult<()> {
    let manager = TaskManager::<JobBuffer>::new(workers)?;
    unsafe {
        TASK_MANAGER = Some(manager);
    }
    Ok(())
}

pub fn get_manager() -> &'static mut JobManager {
    let mut manager = unsafe { TASK_MANAGER.as_mut().unwrap() };
    manager
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
