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
pub mod index;

use crate::utils::PageFunction;
use rumtk_core::cache::{new_cache, LazyRUMCache};
use rumtk_core::strings::RUMString;
use rumtk_core::{rumtk_cache_get, rumtk_cache_push};
use std::ops::Deref;

pub type PageCache = LazyRUMCache<RUMString, PageFunction>;
pub type PageItem<'a> = (&'a str, PageFunction);
pub type UserPages<'a> = Vec<PageItem<'a>>;
pub type PageCacheItem = PageFunction;

static mut PAGE_CACHE: PageCache = new_cache();
static mut DEFAULT_PAGE: PageFunction = index::index;
pub const DEFAULT_PAGE_NAME: &str = "index";

pub fn register_page(name: &str, component_fxn: PageFunction) {
    let key = RUMString::from(name);
    rumtk_cache_push!(&raw mut PAGE_CACHE, &key, component_fxn);

    if key == DEFAULT_PAGE_NAME {
        unsafe {
            DEFAULT_PAGE = component_fxn;
        }
    }
    println!(
        "  ➡ Registered page {} => page function [{:?}]",
        name, &component_fxn
    );
}

pub fn get_page(name: &str) -> PageCacheItem {
    match rumtk_cache_get!(
        &raw mut PAGE_CACHE,
        &RUMString::from(name)
    ) {
        Some(page) => page,
        None => {
            unsafe {DEFAULT_PAGE}
        }
    }
}

pub fn get_default_page() -> &'static PageFunction {
    unsafe { &DEFAULT_PAGE }
}

pub fn init_pages(user_components: Option<UserPages>) {
    println!("🗎 Registering user pages! 🗎");
    /* Init any user prescribed components */
    for (name, value) in user_components.unwrap_or_default() {
        register_page(name, value);
    }
    println!(
        "  ➡ Default page => page function [{:?}]",
        &get_default_page()
    );
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
/// use rumtk_web::defaults::{PARAMS_TYPE};
/// use rumtk_web::utils::{SharedAppState, RenderedPageComponentsResult};
/// use rumtk_web::{rumtk_web_render_component, rumtk_web_register_page, rumtk_web_get_page};
///
/// pub fn index(app_state: SharedAppState) -> RenderedPageComponentsResult {
///     let title_welcome = rumtk_web_render_component!("title", [(PARAMS_TYPE, "welcome")], app_state)?.to_string();
///
///     Ok(vec![
///         title_welcome,
///     ])
/// }
///
/// rumtk_web_register_page!("index", index);
/// let r = rumtk_format!("{:?}", &rumtk_web_get_page!("index"));
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
/// use rumtk_web::defaults::{PARAMS_TYPE};
/// use rumtk_web::utils::{SharedAppState, RenderedPageComponentsResult};
/// use rumtk_web::{rumtk_web_render_component, rumtk_web_register_page, rumtk_web_get_page, rumtk_web_get_default_page};
///
/// pub fn index(app_state: SharedAppState) -> RenderedPageComponentsResult {
///     let title_welcome = rumtk_web_render_component!("title", [(PARAMS_TYPE, "welcome")], app_state)?.to_string();
///
///     Ok(vec![
///         title_welcome,
///     ])
/// }
///
/// let default = rumtk_format!("{:?}", rumtk_web_get_default_page!());
/// rumtk_web_register_page!("index", index);
/// let r = rumtk_format!("{:?}", &rumtk_web_get_page!("index"));
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
/// use rumtk_web::defaults::{PARAMS_TYPE};
/// use rumtk_web::utils::{SharedAppState, RenderedPageComponentsResult};
/// use rumtk_web::{rumtk_web_render_component, rumtk_web_init_pages, rumtk_web_get_page};
///
/// use rumtk_web::pages::UserPages;
///
///  fn my_page(app_state: SharedAppState) -> RenderedPageComponentsResult {
///     let title_welcome = rumtk_web_render_component!("title", [(PARAMS_TYPE, "welcome")], app_state)?.to_string();
///
///     Ok(vec![
///         title_welcome,
///     ])
/// }
///
/// let my_page_name = "my_page";
///
/// let pages: UserPages = vec![(my_page_name, my_page)];
/// rumtk_web_init_pages!(Some(pages));
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
