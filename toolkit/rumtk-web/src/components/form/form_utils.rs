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
use rumtk_core::cache::{new_cache, LazyRUMCache};
use rumtk_core::strings::RUMString;
use rumtk_core::{rumtk_cache_fetch, rumtk_cache_push};

use crate::components::form::form_element::form_element;
use crate::components::form::props::InputProps;
use crate::rumtk_web_render_component;
use crate::utils::HTMLResult;

pub type FormElements = Vec<RUMString>;
pub type FormCache = LazyRUMCache<RUMString, FormElements>;
pub type FormElementBuilder =
    fn(element: &str, data: &str, props: InputProps, css: &str) -> RUMString;
pub type FormBuilderFunction = fn(builder: FormElementBuilder) -> FormElements;

static mut form_cache: FormCache = new_cache();

fn new_form_entry(_name: &RUMString) -> FormElements {
    vec![]
}

fn build_form_element(element: &str, data: &str, props: InputProps, css: &str) -> RUMString {
    rumtk_web_render_component!(|| -> HTMLResult { form_element(element, data, props, css) })
}

pub fn register_form_elements(name: &str, element_builder: FormBuilderFunction) -> &FormElements {
    let key = RUMString::from(name);
    rumtk_cache_fetch!(&mut form_cache, &key, new_form_entry);
    let data = element_builder(build_form_element);
    rumtk_cache_push!(&mut form_cache, &key, &data)
}

pub fn get_form(name: &str) -> &FormElements {
    rumtk_cache_fetch!(&mut form_cache, &RUMString::from(name), new_form_entry)
}
