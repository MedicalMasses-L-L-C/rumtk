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
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use rumtk_core::strings::{RUMString, RUMStringConversions, ToCompactString};

pub type HTMLResponse = Response<Body>;

#[derive(Debug, Clone)]
pub struct HTMLBody(Html<String>);
#[derive(Debug, Clone)]
pub struct RedirectBody(Redirect);

#[derive(Default, Debug)]
pub enum RUMWebRedirect {
    Redirect(RUMString),
    RedirectTemporary(RUMString),
    RedirectPermanent(RUMString),
    #[default]
    None,
}

#[derive(Default, Debug, PartialEq, Clone)]
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
            RUMWebResponse::GetResponse(res) => res.to_string(),
            _ => RUMString::default(),
        }
    }

    pub fn get_url(&self) -> RUMString {
        match self {
            RUMWebResponse::RedirectResponse(res) => res.location(),
            RUMWebResponse::RedirectTemporaryResponse(res) => res.location(),
            RUMWebResponse::RedirectPermanentResponse(res) => res.location(),
            _ => RUMString::default(),
        }
    }

    pub fn get_code(&self) -> StatusCode {
        match self {
            RUMWebResponse::RedirectResponse(res) => res.status_code(),
            RUMWebResponse::RedirectTemporaryResponse(res) => res.status_code(),
            RUMWebResponse::RedirectPermanentResponse(res) => res.status_code(),
            _ => StatusCode::OK,
        }
    }

    pub fn into_html_result(self) -> HTMLResult {
        Ok(self)
    }

    pub fn into_get_response(data: &str) -> Self {
        RUMWebResponse::GetResponse(HTMLBody::from(String::from(data)))
    }
}

impl IntoResponse for RUMWebResponse {
    fn into_response(self) -> HTMLResponse {
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

impl HTMLBody {
    pub fn from(body: String) -> Self {
        Self(Html(body))
    }

    pub fn into_response(self) -> HTMLResponse {
        self.0.into_response()
    }

    pub fn to_rumstring(&self) -> RUMString {
        self.0 .0.to_string()
    }
}

impl PartialEq for HTMLBody {
    fn eq(&self, other: &Self) -> bool {
        self.0 .0 == other.0 .0
    }
}

impl RedirectBody {
    pub fn to(url: &RUMString) -> Self {
        RedirectBody(Redirect::to(url.as_str()))
    }

    pub fn temporary(url: &RUMString) -> Self {
        RedirectBody(Redirect::temporary(url.as_str()))
    }

    pub fn permanent(url: &RUMString) -> Self {
        RedirectBody(Redirect::permanent(url.as_str()))
    }

    pub fn location(&self) -> RUMString {
        self.0.location().to_string()
    }

    pub fn status_code(&self) -> StatusCode {
        self.0.status_code()
    }

    pub fn into_response(self) -> HTMLResponse {
        self.0.into_response()
    }
}

impl PartialEq for RedirectBody {
    fn eq(&self, other: &Self) -> bool {
        self.0.location() == other.0.location() && self.0.status_code() == other.0.status_code()
    }
}
