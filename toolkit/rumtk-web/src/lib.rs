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
#![feature(once_cell_get_mut)]
#![feature(macro_metavar_expr)]
extern crate core;

pub mod api;
pub mod components;
pub mod css;
pub mod pages;
pub mod static_components;
pub mod utils;
pub mod js;

pub use app::*;
pub use utils::*;

///
/// Add utils unit tests here to ensure internal functions work.
///
#[cfg(test)]
mod tests {
    use crate::defaults::PARAMS_TITLE;
    use crate::testdata::{create_test_form, TESTDATA_EXPECTED_FORMDATA, TESTDATA_EXPECTED_FORMDATA_EMPTY, TESTDATA_FORMDATA_EMPTY_REQUEST, TESTDATA_FORMDATA_EMPTY_REQUEST_WITH_BOUNDARIES, TESTDATA_FORMDATA_REQUEST, TRIMMED_HTML_TITLE_RENDER};
    use crate::{
        rumtk_web_init_components, rumtk_web_render_component, rumtk_web_render_template,
        rumtk_web_render_redirect, RUMWebRedirect, SharedAppState,
    };
    use crate::{RUMWebResponse, RUMWebTemplate};
    use rumtk_core::strings::RUMStringConversions;

    ///////////////////////////////////FormData/////////////////////////////////////////////////
    #[test]
    fn test_compile_form() {
        let expected_form = TESTDATA_EXPECTED_FORMDATA();
        let form_data = create_test_form(TESTDATA_FORMDATA_REQUEST).expect("Form");

        assert_eq!(form_data, expected_form, "Form results mismatch!");
    }

    #[test]
    fn test_compile_empty_form() {
        let expected_form = TESTDATA_EXPECTED_FORMDATA_EMPTY();
        let form_data = create_test_form(TESTDATA_FORMDATA_EMPTY_REQUEST).expect("Form");

        assert_eq!(form_data, expected_form, "Form results mismatch!");
    }

    #[test]
    fn test_compile_empty_form_with_boundaries() {
        let expected_form = TESTDATA_EXPECTED_FORMDATA_EMPTY();
        let form_data = create_test_form(TESTDATA_FORMDATA_EMPTY_REQUEST_WITH_BOUNDARIES).expect("Form");

        assert_eq!(form_data, expected_form, "Form results mismatch!");
    }

    ///////////////////////////////////Response/////////////////////////////////////////////////
    #[test]
    fn test_render_redirect_response() {
        let url = "http://localhost/redirected";
        let redirect =
            rumtk_web_render_redirect!(RUMWebRedirect::Redirect(url.to_rumstring())).unwrap();
        let redirect_code = redirect.get_code();
        let redirect_url = redirect.get_url();
        assert_eq!(redirect_url, url, "Redirect url mismatch!");
        assert_eq!(redirect_code, 303, "Wrong redirect code!");
    }

    #[test]
    fn test_render_redirect_response_temporary() {
        let url = "http://localhost/redirected";
        let redirect =
            rumtk_web_render_redirect!(RUMWebRedirect::RedirectTemporary(url.to_rumstring()))
                .unwrap();
        let redirect_code = redirect.get_code();
        let redirect_url = redirect.get_url();
        assert_eq!(redirect_url, url, "Redirect url mismatch!");
        assert_eq!(redirect_code, 307, "Wrong redirect code!");
    }

    #[test]
    fn test_render_redirect_response_permanent() {
        let url = "http://localhost/redirected";
        let redirect =
            rumtk_web_render_redirect!(RUMWebRedirect::RedirectPermanent(url.to_rumstring()))
                .unwrap();
        let redirect_code = redirect.get_code();
        let redirect_url = redirect.get_url();
        assert_eq!(redirect_url, url, "Redirect url mismatch!");
        assert_eq!(redirect_code, 308, "Wrong redirect code!");
    }

    #[test]
    fn test_render_standard_web_component() {
        rumtk_web_init_components!(None);

        let params = [(PARAMS_TITLE, "Hello World!")];
        let state = SharedAppState::default();
        let rendered = rumtk_web_render_component!("title", params, state).unwrap().to_rumstring();

        assert_eq!(
            rendered, TRIMMED_HTML_TITLE_RENDER,
            "Commponent rendered improperly!"
        );
    }

    #[test]
    fn test_render() {
        #[derive(RUMWebTemplate)]
        #[template(source = "<div></div>", ext = "html")]
        struct Div {}

        let result = rumtk_web_render_template(Div {}, RUMWebRedirect::None).unwrap();
        let expected = RUMWebResponse::into_get_response("<div></div>");

        assert_eq!(result, expected, "Test Div template rendered improperly!");
    }

    ///////////////////////////////////HTML/////////////////////////////////////////////////
    #[test]
    fn test_render_html_component() {}
}
