/*
 *     rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 *     This toolkit aims to be reliable, simple, performant, and standards compliant.
 *     Copyright (C) 2026  Luis M. Santos, M.D.
 *     Copyright (C) 2026  MedicalMasses L.L.C.
 *
 *     This program is free software: you can redistribute it and/or modify
 *     it under the terms of the GNU General Public License as published by
 *     the Free Software Foundation, either version 3 of the License, or
 *     (at your option) any later version.
 *
 *     This program is distributed in the hope that it will be useful,
 *     but WITHOUT ANY WARRANTY; without even the implied warranty of
 *     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *     GNU General Public License for more details.
 *
 *     You should have received a copy of the GNU General Public License
 *     along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

pub mod data {
    use crate::form_data::{compile_form_data, FormResult};
    use crate::{FormData, RouterForm};
    use axum::extract::FromRequest;
    use axum::extract::Request;
    use axum::http::header::{CONTENT_LENGTH, CONTENT_TYPE, HOST};
    use axum::http::Method;
    use rumtk_core::base::RUMVec;
    use rumtk_core::rumtk_resolve_task;
    use std::io::Write;

    type TESTDATA_REQUEST_FUNCTION = fn() -> Request;
    type TESTDATA_REQUEST_BODY_FUNCTION = fn() -> RUMVec<u8>;
    type TESTDATA_FORMDATA_FUNCTION = fn() -> FormData;

    /// Credit: https://users.rust-lang.org/t/testing-multipart-form-data-fails-with-cryptic-error-message/100633
    const TESTDATA_FORM_BODY: TESTDATA_REQUEST_BODY_FUNCTION = || {
        let mut buffer = RUMVec::<u8>::new();

        write!(buffer, "------WebKitFormBoundary7MA4YWxkTrZu0gW\r\n").unwrap();
        write!(
            buffer,
            "Content-Disposition: form-data; name=\"username\"\r\n"
        )
            .unwrap();
        write!(buffer, "\r\n").unwrap();
        write!(buffer, "JohnDoe\r\n").unwrap();
        write!(buffer, "------WebKitFormBoundary7MA4YWxkTrZu0gW\r\n").unwrap();
        write!(
            buffer,
            "Content-Disposition: form-data; name=\"profile_pic\"; filename=\"avatar.png\"\r\n"
        )
            .unwrap();
        write!(buffer, "Content-Type: image/png\r\n").unwrap();
        write!(buffer, "\r\n").unwrap();
        write!(buffer, "[Binary Data of the Image File]\r\n").unwrap();
        write!(buffer, "------WebKitFormBoundary7MA4YWxkTrZu0gW\r\n").unwrap();

        buffer
    };

    const TESTDATA_FORM_BODY_EMPTY_BOUNDARY: TESTDATA_REQUEST_BODY_FUNCTION = || {
        let mut buffer = RUMVec::<u8>::new();

        write!(buffer, "------WebKitFormBoundary7MA4YWxkTrZu0gW\r\n").unwrap();

        buffer
    };

    pub const TESTDATA_FORMDATA_REQUEST: TESTDATA_REQUEST_FUNCTION = || -> Request {
        let body_buffer = TESTDATA_FORM_BODY();
        Request::builder()
            .method(Method::POST)
            .uri("/upload")
            .header(HOST, "localhost")
            .header(
                CONTENT_TYPE,
                "multipart/form-data; boundary=----WebKitFormBoundary7MA4YWxkTrZu0gW",
            )
            .body(body_buffer.into())
            .unwrap()
    };

    pub const TESTDATA_FORMDATA_EMPTY_REQUEST: TESTDATA_REQUEST_FUNCTION = || -> Request {
        let body_buffer = RUMVec::<u8>::new();
        Request::builder()
            .method(Method::POST)
            .uri("/upload")
            .header(HOST, "localhost")
            .header(
                CONTENT_TYPE,
                "multipart/form-data; boundary=----WebKitFormBoundary7MA4YWxkTrZu0gW",
            )
            .body(body_buffer.into())
            .unwrap()
    };

    pub const TESTDATA_FORMDATA_EMPTY_REQUEST_WITH_BOUNDARIES: TESTDATA_REQUEST_FUNCTION = || -> Request {
        let body_buffer = TESTDATA_FORM_BODY_EMPTY_BOUNDARY();
        Request::builder()
            .method(Method::POST)
            .uri("/upload")
            .header(HOST, "localhost")
            .header(
                CONTENT_TYPE,
                "multipart/form-data; boundary=----WebKitFormBoundary7MA4YWxkTrZu0gW",
            )
            .header(CONTENT_LENGTH, "38")
            .body(body_buffer.into())
            .unwrap()
    };

    pub const TESTDATA_EXPECTED_FORMDATA: TESTDATA_FORMDATA_FUNCTION = || -> FormData {
        let mut expected_form = FormData::default();

        expected_form
            .form
            .insert("username".into(), "JohnDoe".into());
        expected_form.form.insert(
            "profile_pic".into(),
            "[Binary Data of the Image File]".into(),
        );

        expected_form
    };

    pub const TESTDATA_EXPECTED_FORMDATA_EMPTY: TESTDATA_FORMDATA_FUNCTION = || -> FormData {
        let mut expected_form = FormData::default();

        expected_form
    };

    async fn create_form(form_fxn: TESTDATA_REQUEST_FUNCTION) -> FormResult {
        let req = form_fxn();
        let mut raw_form = RouterForm::from_request(req, &())
            .await
            .expect("Multipart form expected.");
        compile_form_data(&mut raw_form).await
    }

    pub fn create_test_form(form_fxn: TESTDATA_REQUEST_FUNCTION) -> FormResult {
        let handle = create_form(form_fxn);
        rumtk_resolve_task!(handle)
    }

    pub const UNTRIMMED_HTML_RENDER: &str = "\n        \n           \n        \n        <div class='div-default'>default</div>\n    \n        \n    ";
    pub const RAW_HTML_PREFORMATTED: &str = "\n        <div>\n            <pre>\n                # started on Thu Apr 30 18:10:00 2026\n\n\n Performance counter stats for &#39;../target/debug/rumtk-hl7-v2-parse&#39;:\n\n         9,894,221      cache-references:u                                                    \n           386,828      cache-misses:u                   #    3.91% of all cache refs         \n       883,435,175      cycles:u                                                              \n     2,319,594,505      instructions:u                   #    2.63  insn per cycle            \n       353,631,271      branches:u                                                            \n             5,549      faults:u                                                              \n                 0      migrations:u                                                          \n\n       7.212089850 seconds time elapsed\n\n       0.208060000 seconds user\n       0.021617000 seconds sys\n\n\n\n            </pre>\n        </div>\n    ";
    pub const TRIMMED_HTML_RENDER: &str = "<div class='div-default'>default</div>";
    pub const TRIMMED_HTML_RENDER_META: &str = "<meta charset='UTF-8'><meta http-equiv='Content-Type' content='text/html; charset=utf-8' /><meta name='viewport' content='width=device-width, initial-scale=1.0' /><meta http-equiv='X-UA-Compatible' content='IE=edge,chrome=1'/><meta name='description' content=''><title></title><link rel='icon' type='image/png' href='/static/img/icon.png'>";
    pub const TRIMMED_HTML_TITLE_RENDER: &str = "<div class='f14 centered title-default-container'><a id='default'><h2 class='title-default'>DEFAULT</h2><h2 class='title-default-overlay no-select'>DEFAULT</h2></a></div>";
    pub const JOB_LOADER_TEST_PATTERN: &str = "hx-trigger='every 2s' hx-swap='outerHTML' hx-target='#loader-";
    pub const TRIMMED_HTML_PREFORMATTED: &str = "<div><pre>\n                # started on Thu Apr 30 18:10:00 2026\n\n\n Performance counter stats for &#39;../target/debug/rumtk-hl7-v2-parse&#39;:\n\n         9,894,221      cache-references:u                                                    \n           386,828      cache-misses:u                   #    3.91% of all cache refs         \n       883,435,175      cycles:u                                                              \n     2,319,594,505      instructions:u                   #    2.63  insn per cycle            \n       353,631,271      branches:u                                                            \n             5,549      faults:u                                                              \n                 0      migrations:u                                                          \n\n       7.212089850 seconds time elapsed\n\n       0.208060000 seconds user\n       0.021617000 seconds sys\n\n\n\n            </pre></div>";
}

