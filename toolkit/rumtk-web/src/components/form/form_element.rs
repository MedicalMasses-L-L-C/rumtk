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
use crate::components::form::props::InputProps;
use crate::utils::types::HTMLResult;
use crate::{rumtk_web_render_html, RUMWebTemplate};

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        <{{element}} {{props|safe}} class='{{css_class}}'>{{data}}</{{element}}>
    ",
    ext = "html"
)]
pub struct FormElement<'a> {
    element: &'a str,
    data: &'a str,
    props: &'a str,
    css_class: &'a str,
}

pub fn form_element(element: &str, data: &str, props: InputProps, css_class: &str) -> HTMLResult {
    rumtk_web_render_html!(FormElement {
        element,
        data,
        props: &props.to_rumstring().replace("\\\\", "\\"),
        css_class
    })
}
