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
use crate::components::form::props::InputProps;
use crate::rumtk_web_render_html;
use crate::utils::types::HTMLResult;
use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(
    source = "
        <{{element}} {{props.to_rumstring()}} class='{{css_class}}'>{{data}}</{{element}}>
    ",
    ext = "html"
)]
pub struct FormElement<'a> {
    element: &'a str,
    data: &'a str,
    props: InputProps<'a>,
    css_class: &'a str,
}

pub fn form_element(element: &str, data: &str, props: InputProps, css_class: &str) -> HTMLResult {
    rumtk_web_render_html!(FormElement {
        element,
        data,
        props,
        css_class
    })
}
