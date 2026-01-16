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
pub mod index;

use crate::utils::PageFunction;
use rumtk_core::cache::{new_cache, LazyRUMCache, LazyRUMCacheValue};
use rumtk_core::strings::RUMString;
use rumtk_core::{rumtk_cache_get, rumtk_cache_push};

pub type PageCache = LazyRUMCache<RUMString, PageFunction>;
pub type PageItem<'a> = (&'a str, PageFunction);
pub type UserPages<'a> = Vec<PageItem<'a>>;
pub type PageCacheItem = LazyRUMCacheValue<PageFunction>;

static mut PAGE_CACHE: PageCache = new_cache();
static DEFAULT_PAGE: PageFunction = index::index;

pub fn register_page(name: &str, component_fxn: PageFunction) {
    let key = RUMString::from(name);
    let _ = rumtk_cache_push!(&raw mut PAGE_CACHE, &key, &component_fxn);
}

pub fn get_page(name: &str) -> PageCacheItem {
    rumtk_cache_get!(&raw mut PAGE_CACHE, &RUMString::from(name), &DEFAULT_PAGE)
}

pub fn init_pages(user_components: &UserPages) {
    /* Init any user prescribed components */
    for itm in user_components {
        let (name, value) = itm;
        register_page(name, *value);
    }
}

#[macro_export]
macro_rules! rumtk_web_register_page {
    ( $key:expr, $fxn:expr ) => {{
        use $crate::pages::register_page;
        register_page($key, $fxn)
    }};
}

#[macro_export]
macro_rules! rumtk_web_get_page {
    ( $key:expr ) => {{
        use $crate::pages::get_page;
        get_page($key)
    }};
}

#[macro_export]
macro_rules! rumtk_web_init_pages {
    ( $pages:expr ) => {{
        use $crate::pages::init_pages;
        init_pages($pages)
    }};
}

#[macro_export]
macro_rules! rumtk_web_collect_page {
    ( $page:expr, $app_state:expr ) => {{
        use $crate::rumtk_web_get_page;

        let page = rumtk_web_get_page!(&$page);

        page($app_state.clone())
    }};
}
