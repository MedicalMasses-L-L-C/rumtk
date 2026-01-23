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
use crate::{Headers, ResponseBody};
use axum::body::Body;
pub use axum::http::{header, header::HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};

pub struct RUMWebResponse {
    pub status: Option<StatusCode>,
    pub headers: Option<Headers>,
    pub body: Option<ResponseBody>,
}

impl IntoResponse for RUMWebResponse {
    fn into_response(mut self) -> Response<Body> {
        let mut response = Response::builder()
            .status(self.status.unwrap_or_else(|| StatusCode::OK))
            .body(self.body.unwrap_or_else(|| ResponseBody::default()))
            .unwrap_or_default();

        let headers = self.headers.unwrap_or_default();
        let r_headers = response.headers_mut();

        for (k, v) in headers.iter() {
            r_headers.insert(k, v.parse().unwrap());
        }

        response
    }
}
