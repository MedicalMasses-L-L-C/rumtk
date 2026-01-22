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
use crate::components::app_shell::app_shell;
use crate::utils::defaults::DEFAULT_ROBOT_TXT;
use crate::utils::types::SharedAppState;
use crate::utils::{HTMLResult, RUMString};
use crate::{rumtk_web_get_component, RUMWebData};
use axum::response::Html;

pub async fn default_robots_matcher(
    _path: Vec<RUMString>,
    _params: RUMWebData,
    _state: SharedAppState,
) -> HTMLResult {
    Ok(Html::<String>::from(String::from(DEFAULT_ROBOT_TXT)))
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

pub async fn default_component_matcher(
    path: Vec<RUMString>,
    params: RUMWebData,
    state: SharedAppState,
) -> HTMLResult {
    let path_components = match path.first() {
        Some(x) => x.split('/').collect::<Vec<&str>>(),
        None => Vec::new(),
    };

    let component = match path_components.first() {
        Some(component) => component,
        None => return Err(RUMString::from("Missing component name to fetch!")),
    };

    let component = rumtk_web_get_component!(component);

    component(&path_components[1..], &params, state)
}

#[macro_export]
macro_rules! rumtk_web_fetch {
    ( $matcher:expr ) => {{
        use axum::extract::{Path, Query, State};
        use axum::response::Html;
        use $crate::utils::types::{RouterAppState, RouterComponents, RouterParams};

        async |Path(path_params): RouterComponents,
               Query(params): RouterParams,
               State(state): RouterAppState|
               -> Html<String> {
            match $matcher(path_params, params, state).await {
                Ok(res) => res,
                Err(e) => {
                    error!("{}", e);
                    return Html(String::default());
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! rumtk_web_post {
    ( $matcher:expr ) => {{
        use axum::extract::{Multipart, Path, Query, State};
        use axum::response::Html;
        use $crate::utils::types::{RouterAppState, RouterComponents, RouterForm, RouterParams};

        async |Path(path_params): RouterComponents,
               Query(mut params): RouterParams,
               State(state): RouterAppState,
               mut Multipart: RouterForm|
               -> Html<String> {
            match $matcher(path_params, params, state).await {
                Ok(res) => res,
                Err(e) => {
                    error!("{}", e);
                    return Html(String::default());
                }
            }
        }
    }};
}
