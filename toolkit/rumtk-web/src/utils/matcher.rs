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
use crate::components::app_shell::app_shell;
use crate::components::div::div;
use crate::utils::defaults::DEFAULT_ROBOT_TXT;
use crate::utils::form_data::compile_form_data;
use crate::utils::types::SharedAppState;
use crate::utils::{HTMLResult, RUMString};
use crate::{rumtk_web_get_api_endpoint, rumtk_web_get_component, rumtk_web_render_component, RUMWebData, RUMWebResponse, RouterForm};
use axum::body::Body;
use axum::http::Response;
use axum::response::{Html, IntoResponse};
use rumtk_core::strings::rumtk_format;

pub async fn default_robots_matcher(
    _path: Vec<RUMString>,
    _params: RUMWebData,
    _state: SharedAppState,
) -> HTMLResult {
    RUMWebResponse::into_get_response(DEFAULT_ROBOT_TXT).into_html_result()
}

pub async fn default_page_matcher(
    path: Vec<RUMString>,
    params: RUMWebData,
    state: SharedAppState,
) -> HTMLResult {
    let path_components = match path.first() {
        Some(x) => x.split('/').collect::<Vec<&str>>(),
        None => Vec::new(),
    };

    // Do not minify the page. we saved 0.3KB but transfer went from 5ms to 45ms
    app_shell(&path_components, &params, state)
}

pub async fn default_api_matcher(
    path: RUMString,
    params: RUMWebData,
    mut form: RouterForm,
    state: SharedAppState,
) -> HTMLResult {
    let form_data = compile_form_data(&mut form).await?;
    let api_endpoint = match rumtk_web_get_api_endpoint!(&path) {
        Some(endpoint) => endpoint,
        None => return Err(rumtk_format!("Requested endpoint is not registered!"))
    };
    api_endpoint(path, params, form_data, state)
}

pub async fn default_component_matcher(
    path: Vec<RUMString>,
    params: RUMWebData,
    state: SharedAppState,
) -> HTMLResult {
    let path_components = match path.first() {
        Some(x) => x.split('/').collect::<Vec<&str>>(),
        None => Vec::new(),
    };

    let component = match path_components.last() {
        Some(component) => component,
        None => return Err(RUMString::from("Missing component name to fetch!")),
    };

    rumtk_web_render_component!(component, &[""], params, state)
}

pub fn match_maker(match_response: HTMLResult) -> Response<Body> {
    match match_response {
        Ok(res) => res.into_response(),
        Err(e) => {
            println!("Error: {}", e);
            Html(String::default()).into_response()
        },
    }
}

#[macro_export]
macro_rules! rumtk_web_fetch {
    ( $matcher:expr ) => {{
        use axum::extract::{Path, Query, State};
        use axum::response::{Html, Response};
        use $crate::matcher::match_maker;
        use $crate::utils::types::{RouterAppState, RouterComponents, RouterForm, RouterParams};

        async |Path(path_params): RouterComponents,
               Query(params): RouterParams,
               State(state): RouterAppState|
               -> Response {
            let r = $matcher(path_params, params, state).await;
            match_maker(r)
        }
    }};
}

#[macro_export]
macro_rules! rumtk_web_api_process {
    ( $matcher:expr ) => {{
        use axum::extract::{Multipart, Path, Query, State};
        use axum::response::{Html, Response};
        use $crate::matcher::match_maker;
        use $crate::utils::types::{RouterAPIPath, RouterAppState, RouterForm, RouterParams};

        async |Path(path_params): RouterAPIPath,
               Query(params): RouterParams,
               State(state): RouterAppState,
               mut form: RouterForm|
               -> Response {
            let r = $matcher(path_params, params, form, state).await;
            match_maker(r)
        }
    }};
}
