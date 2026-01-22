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
use std::ops::Deref;

pub type PageCache = LazyRUMCache<RUMString, PageFunction>;
pub type PageItem<'a> = (&'a str, PageFunction);
pub type UserPages<'a> = Vec<PageItem<'a>>;
pub type PageCacheItem = LazyRUMCacheValue<PageFunction>;

static mut PAGE_CACHE: PageCache = new_cache();
static mut DEFAULT_PAGE: PageFunction = index::index;
pub const DEFAULT_PAGE_NAME: &str = "index";

pub fn register_page(name: &str, component_fxn: PageFunction) -> PageCacheItem {
    let key = RUMString::from(name);
    let r = rumtk_cache_push!(&raw mut PAGE_CACHE, &key, &component_fxn);
    unsafe {
        DEFAULT_PAGE = component_fxn;
    }
    println!(
        "  ➡ Registered page {} => page function [{:?}]",
        name, &component_fxn
    );
    r
}

pub fn get_page(name: &str) -> PageCacheItem {
    rumtk_cache_get!(
        &raw mut PAGE_CACHE,
        &RUMString::from(name),
        get_default_page()
    )
}

pub fn get_default_page() -> &'static PageFunction {
    unsafe { &DEFAULT_PAGE }
}

pub fn init_pages(user_components: &UserPages) {
    println!("🗎 Registering user pages! 🗎");
    /* Init any user prescribed components */
    for itm in user_components {
        let (name, value) = itm;
        register_page(name, *value);
    }
    println!("🗎 ~~~~~~~~~~~~~~~~~~~~~~ 🗎");
}

#[macro_export]
macro_rules! rumtk_web_register_page {
    ( $key:expr, $fxn:expr ) => {{
        use $crate::pages::register_page;
        register_page($key, $fxn)
    }};
}

///
/// Helper function for retrieving pages registered in the global cache using a string key!
///
/// ## Example
///
/// ### With Named Page
///
/// ```
/// use rumtk_core::strings::rumtk_format;
/// use rumtk_web::utils::{SharedAppState, RenderedPageComponents};
/// use rumtk_web::{rumtk_web_render_component, rumtk_web_register_page, rumtk_web_get_page};
///
/// pub fn index(app_state: SharedAppState) -> RenderedPageComponents {
///     let title_welcome = rumtk_web_render_component!("title", [("type", "welcome")], app_state.clone());
///
///     vec![
///         title_welcome,
///     ]
/// }
///
/// let r = rumtk_format!("{:?}", &rumtk_web_register_page!("index", index));
/// let p = rumtk_format!("{:?}", &rumtk_web_get_page!("index"));
///
///  assert_eq!(&r, &p, "{}", rumtk_format!("The registered page does not match the retrieved page!\nGot: {:?}\nExpected: {:?}", &r, &p));
///
/// ```
///
/// ### With Default Page
///
/// ```
/// use rumtk_core::strings::rumtk_format;
/// use rumtk_web::utils::{SharedAppState, RenderedPageComponents};
/// use rumtk_web::{rumtk_web_render_component, rumtk_web_register_page, rumtk_web_get_page, rumtk_web_get_default_page};
///
/// pub fn index(app_state: SharedAppState) -> RenderedPageComponents {
///     let title_welcome = rumtk_web_render_component!("title", [("type", "welcome")], app_state.clone());
///
///     vec![
///         title_welcome,
///     ]
/// }
///
/// let default = rumtk_format!("{:?}", rumtk_web_get_default_page!());
/// let r = rumtk_format!("{:?}", &rumtk_web_register_page!("index", index));
/// let p = rumtk_format!("{:?}", &rumtk_web_get_page!(""));
///
///  assert_ne!(&default, &p, "{}", rumtk_format!("The default page matches the retrieved page!\nGot: {:?}\nExpected: {:?}", &r, &p));
///  assert_eq!(&r, &p, "{}", rumtk_format!("The registered page does not match the retrieved page!\nGot: {:?}\nExpected: {:?}", &r, &p));
///
/// ```
///
#[macro_export]
macro_rules! rumtk_web_get_page {
    ( $key:expr ) => {{
        use $crate::pages::get_page;
        get_page($key)
    }};
}

///
/// Returns the default page function that can be used for rendering that page
///
#[macro_export]
macro_rules! rumtk_web_get_default_page {
    ( ) => {{
        use $crate::pages::get_default_page;
        get_default_page()
    }};
}

///
/// Registers a set of pages provided by the user.
///
/// ## Example
///
///```
/// use std::ops::Deref;
/// use rumtk_core::strings::rumtk_format;
/// use rumtk_web::utils::{SharedAppState, RenderedPageComponents};
/// use rumtk_web::{rumtk_web_render_component, rumtk_web_init_pages, rumtk_web_get_page};
///
/// fn my_page(app_state: SharedAppState) -> RenderedPageComponents {
///     let title_welcome = rumtk_web_render_component!("title", [("type", "welcome")], app_state.clone());
///
///     vec![
///         title_welcome,
///     ]
/// }
///
/// let my_page_name = "my_page";
///
/// rumtk_web_init_pages!(&vec![(my_page_name, my_page)]);
///```
///
///
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
