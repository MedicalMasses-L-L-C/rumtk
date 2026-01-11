use crate::components::{COMPONENTS, app_shell::app_shell};
use crate::utils::defaults::DEFAULT_ROBOT_TXT;
use crate::utils::types::{SharedAppConf, TextMap};
use crate::utils::{HTMLResult, MMString};
use axum::response::Html;
use std::collections::HashMap;

pub async fn default_robots_matcher(
    path: Vec<MMString>,
    params: HashMap<MMString, MMString>,
    state: SharedAppConf,
) -> HTMLResult {
    Ok(Html::<MMString>::from(MMString::from(DEFAULT_ROBOT_TXT)))
}

pub async fn default_page_matcher(
    path: Vec<MMString>,
    params: HashMap<MMString, MMString>,
    state: SharedAppConf,
) -> HTMLResult {
    let path_components = match path.first() {
        Some(x) => x.split('/').collect::<Vec<&str>>(),
        None => Vec::new(),
    };

    app_shell(&path_components, &params, state.clone())
}

pub async fn default_component_matcher(
    path: Vec<MMString>,
    params: HashMap<MMString, MMString>,
    state: SharedAppConf,
) -> HTMLResult {
    let path_components = match path.first() {
        Some(x) => x.split('/').collect::<Vec<&str>>(),
        None => Vec::new(),
    };

    let component = match path_components.first() {
        Some(component) => component,
        None => return Err(MMString::from("Missing component name to fetch!")),
    };

    let component = COMPONENTS.get(component);

    match component {
        Some(cf) => cf(&path_components[1..], &params, state.clone()),
        _ => Ok(Html("<div></div>".to_string())),
    }
}
