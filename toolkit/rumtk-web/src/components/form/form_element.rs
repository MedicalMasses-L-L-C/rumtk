/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2026  Luis M. Santos, M.D. <lsantos@medicalmasses.com>
 * Copyright (C) 2026  MedicalMasses L.L.C. <contact@medicalmasses.com>
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
use crate::components::form::form_node::FormNode;
use crate::components::form::input_element::input_element;
use crate::components::form::props::InputProps;
use crate::components::form::select_element::select_element;
use crate::defaults::{ELEMENT_INPUT, ELEMENT_LABEL, ELEMENT_SELECT, ELEMENT_TEXTAREA};
use crate::ComponentResult;
use rumtk_core::strings::rumtk_format;

pub fn form_element(element: &str, data: &str, props: InputProps, css_class: &str) -> ComponentResult<FormNode> {
    match element {
        ELEMENT_INPUT | ELEMENT_LABEL | ELEMENT_TEXTAREA => input_element(element, data, props, css_class),
        ELEMENT_SELECT => select_element(element, data, props, css_class),
        _ => Err(rumtk_format!("Element of type {} not supported!", element))
    }
}
