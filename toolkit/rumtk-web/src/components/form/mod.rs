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
pub mod form;
pub mod form_element;
pub mod form_utils;
pub mod props;

///
/// This is an API macro for defining a form that can be used to render it later in your web pages.
///
#[macro_export]
macro_rules! rumtk_web_add_form {
    ( $name:expr, $build_fxn:expr ) => {{
        use $crate::components::form::form_utils::register_form_elements;

        register_form_elements($name, $build_fxn)
    }};
}

///
/// This is an API macro to get the list of rendered elements that will be fed into the form shell
/// to render your form in your web page.
///
#[macro_export]
macro_rules! rumtk_web_get_form {
    ( $name:expr ) => {{
        use $crate::components::form::form_utils::get_form;

        get_form($name)
    }};
}
