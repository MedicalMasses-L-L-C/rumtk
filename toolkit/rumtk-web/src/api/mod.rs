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
use crate::utils::APIFunction;
use crate::{APIPath, FormData, HTMLResult, RUMWebData, SharedAppState};
use rumtk_core::cache::{new_cache, LazyRUMCache};
use rumtk_core::strings::{rumtk_format, RUMString};
use rumtk_core::{rumtk_cache_get, rumtk_cache_push};

pub type APICache = LazyRUMCache<RUMString, APIFunction>;
pub type APIItem<'a> = (&'a str, APIFunction);
pub type UserAPIEndpoints<'a> = Vec<APIItem<'a>>;
pub type APICacheItem = APIFunction;

static mut API_CACHE: APICache = new_cache();
const DEFAULT_API_HANDLER: APIFunction =
    |path: APIPath, params: RUMWebData, form: FormData, state: SharedAppState| -> HTMLResult {
        Err(rumtk_format!(
            "No handler registered for API endpoint => {}",
            path
        ))
    };

pub fn register_api_endpoint(name: &str, api_handler: APIFunction) -> APICacheItem {
    let key = RUMString::from(name);
    rumtk_cache_push!(&raw mut API_CACHE, &key, api_handler);

    println!(
        "  ➡ Registered api endpoint {} => api function [{:?}]",
        name, &api_handler
    );
    api_handler
}

pub fn get_endpoint(name: &str) -> Option<APICacheItem> {
    rumtk_cache_get!(
        &raw mut API_CACHE,
        &RUMString::from(name)
    )
}

pub fn get_default_api_handler() -> &'static APIFunction {
    &DEFAULT_API_HANDLER
}

pub fn init_endpoints(user_components: Option<UserAPIEndpoints>) {
    println!("🌩 Registering API Endpoints! 🌩");
    /* Init any user prescribed components */
    for (name, value) in user_components.unwrap_or_default() {
        let _ = register_api_endpoint(name, value);
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
