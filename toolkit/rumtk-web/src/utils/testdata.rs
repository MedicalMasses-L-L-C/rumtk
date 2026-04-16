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

use std::io::Write;

use crate::form_data::{compile_form_data, FormResult};
use crate::{FormData, RouterForm};
use axum::extract::{FromRequest, Request};
use axum::http::header::{CONTENT_TYPE, HOST};
use axum::http::Method;
use rumtk_core::core::RUMVec;
use rumtk_core::rumtk_resolve_task;

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

async fn create_form(form_fxn: TESTDATA_REQUEST_FUNCTION) -> FormResult {
    let req = form_fxn();
    let mut raw_form = RouterForm::from_request(req, &())
        .await
        .expect("Multipart form expected.");
    compile_form_data(&mut raw_form).await
}

pub fn create_test_form(form_fxn: TESTDATA_REQUEST_FUNCTION) -> FormResult {
    let handle = create_form(form_fxn);
    rumtk_resolve_task!(handle)?
}

pub const UNTRIMMED_HTML_RENDER: &str = "\n        \n           \n        \n        <div class='div-default'>default</div>\n    \n        \n    ";
pub const TRIMMED_HTML_RENDER: &str = "<div class='div-default'>default</div>";
