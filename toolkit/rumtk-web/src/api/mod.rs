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
use crate::utils::APIFunction;
use crate::{APIPath, HTMLResult, SharedAppState, URLParams};
use axum::extract::Multipart;
use rumtk_core::cache::{new_cache, LazyRUMCache, LazyRUMCacheValue};
use rumtk_core::strings::{rumtk_format, RUMString};
use rumtk_core::{rumtk_cache_get, rumtk_cache_push};

pub type APICache = LazyRUMCache<RUMString, APIFunction>;
pub type APIItem<'a> = (&'a str, APIFunction);
pub type UserAPIEndpoints<'a> = Vec<APIItem<'a>>;
pub type APICacheItem = LazyRUMCacheValue<APIFunction>;

static mut API_CACHE: APICache = new_cache();
const DEFAULT_API_HANDLER: APIFunction =
    |path: APIPath, params: URLParams, form: Multipart, state: SharedAppState| -> HTMLResult {
        Err(rumtk_format!(
            "No handler registered for API endpoint => {}",
            path
        ))
    };

pub fn register_api_endpoint(name: &str, api_handler: APIFunction) -> APICacheItem {
    let key = RUMString::from(name);
    let r = rumtk_cache_push!(&raw mut API_CACHE, &key, &api_handler);

    println!(
        "  ➡ Registered api endpoint {} => api function [{:?}]",
        name, &api_handler
    );
    r
}

pub fn get_endpoint(name: &str) -> APICacheItem {
    rumtk_cache_get!(
        &raw mut API_CACHE,
        &RUMString::from(name),
        get_default_api_handler()
    )
}

pub fn get_default_api_handler() -> &'static APIFunction {
    &DEFAULT_API_HANDLER
}

pub fn init_endpoints(user_components: &UserAPIEndpoints) {
    println!("🌩 Registering API Endpoints! 🌩");
    /* Init any user prescribed components */
    for (name, value) in user_components {
        let _ = register_api_endpoint(name, *value);
    }
    println!("🌩 ~~~~~~~~~~~~~~~~~~~~~~ 🌩");
}

#[macro_export]
macro_rules! rumtk_web_register_api {
    ( $key:expr, $fxn:expr ) => {{
        use $crate::api::register_api_endpoint;
        register_api_endpoint($key, $fxn)
    }};
}

#[macro_export]
macro_rules! rumtk_web_get_api_endpoint {
    ( $key:expr ) => {{
        use $crate::api::get_endpoint;
        get_endpoint($key)
    }};
}

#[macro_export]
macro_rules! rumtk_web_init_api_endpoints {
    ( $pages:expr ) => {{
        use $crate::api::init_endpoints;
        init_endpoints($pages)
    }};
}
