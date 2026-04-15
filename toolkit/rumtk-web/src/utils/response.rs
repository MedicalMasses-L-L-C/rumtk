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

/* Responses */
use axum::body::Body;
use axum::response::{Html, IntoResponse, Redirect, Response};
use rumtk_core::strings::{RUMString, RUMStringConversions};

pub type HTMLBody = Html<String>;
pub type RedirectBody = Redirect;

#[derive(Default, Debug)]
pub enum RUMWebRedirect {
    Redirect(RUMString),
    RedirectTemporary(RUMString),
    RedirectPermanent(RUMString),
    #[default]
    None,
}

#[derive(Default, Debug)]
pub enum RUMWebResponse {
    GetResponse(HTMLBody),
    RedirectResponse(RedirectBody),
    RedirectTemporaryResponse(RedirectBody),
    RedirectPermanentResponse(RedirectBody),
    #[default]
    None,
}

pub type HTMLResult = Result<RUMWebResponse, RUMString>;

/* Implementations */
impl RUMWebResponse {
    pub fn is_redirect(&self) -> bool {
        match self {
            RUMWebResponse::RedirectResponse(_) => true,
            RUMWebResponse::RedirectTemporaryResponse(_) => true,
            RUMWebResponse::RedirectPermanentResponse(_) => true,
            _ => false,
        }
    }

    pub fn to_rumstring(&self) -> RUMString {
        match self {
            RUMWebResponse::GetResponse(res) => res.0.to_rumstring(),
            _ => RUMString::default(),
        }
    }

    pub fn into_html_result(self) -> HTMLResult {
        Ok(self)
    }
}

impl IntoResponse for RUMWebResponse {
    fn into_response(self) -> Response<Body> {
        match self {
            RUMWebResponse::GetResponse(r) => r.into_response(),
            RUMWebResponse::RedirectResponse(r) => r.into_response(),
            RUMWebResponse::RedirectTemporaryResponse(r) => r.into_response(),
            RUMWebResponse::RedirectPermanentResponse(r) => r.into_response(),
            RUMWebResponse::None => Html(String::default()).into_response(),
        }
    }
}

impl RUMWebRedirect {
    pub fn into_web_response(self, default: Option<String>) -> RUMWebResponse {
        match self {
            RUMWebRedirect::Redirect(url) => {
                RUMWebResponse::RedirectResponse(RedirectBody::to(&url))
            }
            RUMWebRedirect::RedirectTemporary(url) => {
                RUMWebResponse::RedirectTemporaryResponse(RedirectBody::temporary(&url))
            }
            RUMWebRedirect::RedirectPermanent(url) => {
                RUMWebResponse::RedirectPermanentResponse(RedirectBody::permanent(&url))
            }
            RUMWebRedirect::None => {
                RUMWebResponse::GetResponse(HTMLBody::from(default.unwrap_or(String::default())))
            }
        }
    }
}
