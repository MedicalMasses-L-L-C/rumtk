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
use crate::utils::ComponentFunction;
use rumtk_core::cache::{new_cache, LazyRUMCache};
use rumtk_core::strings::RUMString;
use rumtk_core::{rumtk_cache_get, rumtk_cache_push};

//AppShell
pub mod app_body;
pub mod app_head;
pub mod app_shell;

// Components
mod contact_button;
mod contact_card;
mod content_viewer;
pub mod div;
mod footer;
pub mod form;
mod formatted_label;
mod header;
mod info_card;
mod label;
mod list;
mod logo;
mod main;
mod navlink;
mod portrait_card;
mod script;
mod socials;
mod spacer;
mod text_card;
mod title;
mod select;
mod loader;
mod job_loader;
mod container;

pub type ComponentCache = LazyRUMCache<RUMString, ComponentFunction>;
pub type UserComponentItem<'a> = (&'a str, ComponentFunction);
pub type UserComponents<'a> = Vec<UserComponentItem<'a>>;
pub type UserComponentCacheItem = ComponentFunction;

static mut COMPONENT_CACHE: ComponentCache = new_cache();
static DEFAULT_COMPONENT: ComponentFunction = div::div;

pub fn register_component(name: &str, component_fxn: ComponentFunction) {
    let key = RUMString::from(name);
    let _ = rumtk_cache_push!(&raw mut COMPONENT_CACHE, &key, component_fxn);
}

pub fn get_component(name: &str) -> Option<UserComponentCacheItem> {
    rumtk_cache_get!(
        &raw mut COMPONENT_CACHE,
        &RUMString::from(name)
    )
}

pub fn init_components(user_components: Option<UserComponents>) {
    /* Register the default library components */
    register_component("logo", logo::logo);
    register_component("info_card", info_card::info_card);
    register_component("portrait_card", portrait_card::portrait_card);
    register_component("title", title::title);
    register_component("footer", footer::footer);
    register_component("main", main::main);
    register_component("header", header::header);
    register_component("contact_card", contact_card::contact_card);
    register_component("contact_button", contact_button::contact_button);
    register_component("socials", socials::socials);
    register_component("list", list::list);
    register_component("navlink", navlink::navlink);
    register_component("label", label::label);
    register_component("formatted_label", formatted_label::formatted_label);
    register_component("text_card", text_card::text_card);
    register_component("form", form::form::form);
    register_component("spacer", spacer::spacer);
    register_component("script", script::script);
    register_component("loader", loader::loader);
    register_component("job_loader", job_loader::job_loader);
    register_component("content_viewer", content_viewer::content_viewer);
    register_component("container", container::container);
    register_component("select", select::select);
    register_component("div", div::div);

    /* Init any user prescribed components */
    for itm in user_components.unwrap_or_default() {
        let (name, value) = itm;
        register_component(name, value);
    }
}

#[macro_export]
macro_rules! rumtk_web_register_component {
    ( $key:expr, $fxn:expr ) => {{
        use $crate::components::register_component;
        register_component($key, $fxn)
    }};
}

#[macro_export]
macro_rules! rumtk_web_get_component {
    ( $key:expr ) => {{
        use $crate::components::get_component;
        get_component($key)
    }};
}

#[macro_export]
macro_rules! rumtk_web_init_components {
    ( $components:expr ) => {{
        use $crate::components::init_components;
        init_components($components)
    }};
}
