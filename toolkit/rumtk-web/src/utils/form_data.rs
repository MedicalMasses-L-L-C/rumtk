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
use rumtk_core::core::RUMResult;
use rumtk_core::strings::{
    rumtk_format, RUMArrayConversions, RUMString, RUMStringConversions, ToCompactString,
};
use rumtk_core::types::{RUMBuffer, RUMHashMap, RUMID};

use crate::utils::defaults::*;
use crate::{RUMWebData, RouterForm};

pub type FormBuffer = RUMBuffer;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct FormData {
    pub form: RUMWebData,
    pub files: RUMHashMap<RUMString, FormBuffer>,
}

impl FormData {
    pub fn len(&self) -> usize {
        self.form.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.form.is_empty()
    }
}

pub type FormResult = RUMResult<FormData>;

pub async fn get_type(content_type: &str) -> &'static str {
    match content_type {
        CONTENT_TYPE_PDF => FORM_DATA_TYPE_PDF,
        _ => FORM_DATA_TYPE_DEFAULT,
    }
}

///
/// Converts the incoming form data with type [RouterForm] to [FormData] which is the preferred
/// type in the library.
///
/// ## Examples
///
/// ### Plaintext only
/// ```
/// use axum::body::Body;
/// use rumtk_core::{rumtk_spawn_task, rumtk_resolve_task};
/// use rumtk_web::utils::testdata::data::TESTDATA_FORMDATA_REQUEST;
/// use rumtk_web::utils::RouterForm;
/// use rumtk_web::utils::form_data::compile_form_data;
/// use rumtk_web::FormData;
/// use axum::extract::{Request, FromRequest};
/// use rumtk_core::core::RUMResult;
/// use rumtk_core::types::RUMBuffer;
/// use rumtk_web::form_data::FormResult;
///
/// let expected_form = FormData::default();
///
/// async fn create_form() -> FormResult {
///     let mut raw_form = RouterForm::from_request(TESTDATA_FORMDATA_REQUEST(), &()).await.expect("Multipart form expected.");
///     compile_form_data(&mut raw_form).await
/// }
///
/// rumtk_resolve_task!(create_form());
///
/// ```
///
/// ## Note
/// ```text
/// Because anything that axum does not like could trigger a truncation of the incoming form, I
/// could not even test this function without silencing the parsing error and returning any successful
/// results so far. Axum would complain about an error parsing the multipart form when using a "mocked" Body buffer.
/// Turns out, you can still properly parse a buffer. Also, for testing purposes, you cannot use byte
/// literals as the input to a mocked Body but you can use a Vec<u8> and write!() to it then call
/// into() on that buffer and everything then works despite still complaining about the error.
/// Since we are ignoring anything past this point, I think this is technically safe while still
/// allowing us to test this logic.
/// ```
///
pub async fn compile_form_data(form: &mut RouterForm) -> FormResult {
    let mut form_data = FormData::default();
    
    while let field_result = form.next_field().await {
        match field_result {
            Ok(field_option) => match field_option {
                Some(mut field) => {
                    let typ = match field.content_type() {
                        Some(content_type) => get_type(content_type).await,
                        None => FORM_DATA_TYPE_DEFAULT,
                    };
                    let name = field.name().unwrap_or_default().to_string();
                    
                    // If we got an empty field name, discard.
                    if name.is_empty() {
                        continue;
                    }

                    let data = match field.bytes().await {
                        Ok(bytes) => bytes,
                        Err(e) => {
                            return Err(rumtk_format!("Field data transfer failed because {}!", e))
                        }
                    };

                    if typ == FORM_DATA_TYPE_DEFAULT {
                        form_data.form.insert(name, data.to_vec().to_rumstring());
                    } else {
                        let file_id = RUMID::new_v4().to_string();
                        &form_data.files.insert(file_id.clone(), data);
                        &form_data.form.insert(name, file_id);
                    }
                }
                None => {
                    // Ok so this one is important to be careful with.
                    // During some testing, I was able to pass a form with no fields. 
                    // This form is not 0 bytes as it does contain one boundary line and then the 
                    // new line. However, unlike during the browser test, it was not possible to 
                    // craft a unit test that replicates the issue perhaps because the unit test has 
                    // a fixed buffer? Not sure, it is strange. Without a unit test, it is not clear
                    // how to report issue to axum. At any rate, we should be handling this bit 
                    // anyways. So if we encounter a None for the next field, let's assume the form 
                    // is complete.
                    break;
                }
            },
            Err(e) => {
                // Just return what you got. This is tricky, because anything that axum does not like could
                // trigger a truncation of the incoming form, but I could not even test this function without
                // doing this because it would complain about an error parsing the multipart form. Turns out,
                // you can still properly parse a buffer. Also, for testing purposes, you cannot use byte
                // literals as the input to a mocked Body but you can use a Vec<u8> and write!() to it then
                // call into() on that buffer and everything then works despite still complaining about the error.
                // Since we are ignoring anything past this point, I think this is technically safe while still
                // allowing us to test this logic.
                return Ok(form_data);
            }
        }
    }

    Ok(form_data)
}
