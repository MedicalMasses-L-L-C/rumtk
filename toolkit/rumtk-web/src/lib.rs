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
pub mod api;
pub mod components;
pub mod css;
pub mod pages;
pub mod static_components;
pub mod utils;

pub use app::*;
pub use utils::*;

///
/// Add utils unit tests here to ensure internal functions work.
///
#[cfg(test)]
mod tests {
    use crate::testdata::{
        create_test_form, TESTDATA_EXPECTED_FORMDATA, TESTDATA_FORMDATA_REQUEST,
    };

    #[test]
    fn test_run_app() {}

    #[test]
    fn test_compile_form() {
        let expected_form = TESTDATA_EXPECTED_FORMDATA();
        let form_data = create_test_form(TESTDATA_FORMDATA_REQUEST).expect("Form");

        assert_eq!(form_data, expected_form, "Form results mismatch!");
    }
}
