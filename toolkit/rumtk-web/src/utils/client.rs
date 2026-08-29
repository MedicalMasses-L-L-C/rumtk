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
use rumtk_core::base::RUMResult;
use rumtk_core::buffers::RUMBuffer;
use rumtk_core::rumtk_resolve_task;
use rumtk_core::serde::RUMSerJson;
use rumtk_core::strings::rumtk_format;

pub type RUMWebClient = Client;
pub type RUMWebClientResponse = (u16, RUMBuffer);

#[inline]
async fn consume_response(response: Response) -> RUMResult<RUMWebClientResponse> {
    let response_code = response.status().as_u16();
    let url = response.url().to_string();
    let response = response.bytes().await;
    match response {
        Ok(text) => Ok((response_code, RUMBuffer::from(text.as_ref()))),
        Err(e) => Err(rumtk_format!("No response from {} because {}", url, e)),
    }
}

#[inline]
pub async fn rumtk_web_get(url: String) -> RUMResult<RUMWebClientResponse> {
    let client = Client::new();
    let response = match client.get(&url).send().await {
        Ok(r) => r,
        Err(e) => return Err(rumtk_format!("Get request failed for {} because {}", &url, e)),
    };
    consume_response(response).await
}

#[inline]
pub fn rumtk_web_sync_get(url: &str) -> RUMResult<RUMWebClientResponse> {
    let url = url.to_string();
    rumtk_resolve_task!(rumtk_web_get(url))
}

pub async fn rumtk_web_post<T: RUMSerJson + Sync + Send + 'static>(url: String, data: T) -> RUMResult<RUMWebClientResponse> {
    let client = Client::new();
    let response = match client.post(&url).json(&data).send().await {
        Ok(r) => r,
        Err(e) => return Err(rumtk_format!("Post request failed for {} because {}", &url, e)),
    };
    consume_response(response).await
}

pub fn rumtk_web_sync_post<T: RUMSerJson + Sync + Send + 'static>(url: &str, data: T) -> RUMResult<RUMWebClientResponse> {
    let url = url.to_string();
    rumtk_resolve_task!(rumtk_web_post(url, data))
}
