/*
 *     rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 *     This toolkit aims to be reliable, simple, performant, and standards compliant.
 *     Copyright (C) 2026  Luis M. Santos, M.D. <lsantos@medicalmasses.com>
 *     Copyright (C) 2026  MedicalMasses L.L.C. <contact@medicalmasses.com>
 *
 *     This program is free software: you can redistribute it and/or modify
 *     it under the terms of the GNU General Public License as published by
 *     the Free Software Foundation, either version 3 of the License, or
 *     (at your option) any later version.
 *
 *     This program is distributed in the hope that it will be useful,
 *     but WITHOUT ANY WARRANTY; without even the implied warranty of
 *     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *     GNU General Public License for more details.
 *
 *     You should have received a copy of the GNU General Public License
 *     along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

pub use reqwest::{Client, Response, Request, Error as RequestError};
use rumtk_core::rumtk_resolve_task;

#[inline]
pub async fn rumtk_web_get(url: &str) -> Result<Response, RequestError> {
    let client = Client::new();
    client.get(url).send().await
}

#[inline]
pub fn rumtk_web_sync_get(url: &str) -> Result<Response, RequestError> {
    rumtk_resolve_task!(async move || {
        rumtk_web_get(url).await
    })
}

pub async fn rumtk_web_post<T>(url: &str, data: &T) -> Result<Response, RequestError> {
    let client = Client::new();
    client.post(url).json(data).send().await
}

pub fn rumtk_web_sync_post<T>(url: &str, data: &T) -> Result<Response, RequestError> {
    rumtk_resolve_task!(async move || {
        rumtk_web_post(url, data).await
    })
}
