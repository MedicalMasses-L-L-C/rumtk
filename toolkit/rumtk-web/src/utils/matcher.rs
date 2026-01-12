/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D.
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
use crate::rumtk_web_get_component;
use crate::utils::defaults::DEFAULT_ROBOT_TXT;
use crate::utils::types::SharedAppConf;
use crate::utils::{HTMLResult, RUMString};
use axum::response::Html;
use std::collections::HashMap;

pub async fn default_robots_matcher(
    path: Vec<RUMString>,
    params: HashMap<RUMString, RUMString>,
    state: SharedAppConf,
) -> HTMLResult {
    Ok(Html::<String>::from(String::from(DEFAULT_ROBOT_TXT)))
}

pub async fn default_page_matcher(
    path: Vec<RUMString>,
    params: HashMap<RUMString, RUMString>,
    state: SharedAppConf,
) -> HTMLResult {
    let path_components = match path.first() {
        Some(x) => x.split('/').collect::<Vec<&str>>(),
        None => Vec::new(),
    };

    app_shell(&path_components, &params, state.clone())
}

pub async fn default_component_matcher(
    path: Vec<RUMString>,
    params: HashMap<RUMString, RUMString>,
    state: SharedAppConf,
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

    match component {
        Some(cf) => cf(&path_components[1..], &params, state.clone()),
        _ => Ok(Html("<div></div>".to_string())),
    }
}
