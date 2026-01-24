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
use rumtk_core::core::RUMResult;
use rumtk_core::strings::{
    rumtk_format, RUMArrayConversions, RUMString, RUMStringConversions, ToCompactString,
};
use rumtk_core::types::{RUMBuffer, RUMHashMap, RUMID};

use crate::utils::defaults::*;
use crate::{RUMWebData, RouterForm};

pub type FormBuffer = RUMBuffer;

#[derive(Default, Debug)]
pub struct FormData {
    pub form: RUMWebData,
    pub files: RUMHashMap<RUMString, FormBuffer>,
}

pub type FormResult = RUMResult<FormData>;

pub async fn get_type(content_type: &str) -> &'static str {
    match content_type {
        CONTENT_TYPE_PDF => FORM_DATA_TYPE_PDF,
        _ => FORM_DATA_TYPE_DEFAULT,
    }
}

pub async fn compile_form_data(form: &mut RouterForm) -> FormResult {
    let mut form_data = FormData::default();

    while let Some(mut field) = form.next_field().await.unwrap() {
        let typ = match field.content_type() {
            Some(content_type) => get_type(content_type).await,
            None => FORM_DATA_TYPE_DEFAULT,
        };
        let name = field.name().unwrap_or_default().to_rumstring();

        let data = match field.bytes().await {
            Ok(bytes) => bytes,
            Err(e) => return Err(rumtk_format!("Field data transfer failed because {}!", e)),
        };

        if typ == FORM_DATA_TYPE_DEFAULT {
            form_data.form.insert(name, data.to_vec().to_rumstring());
        } else {
            let file_id = RUMID::new_v4().to_compact_string();
            &form_data.files.insert(file_id.clone(), data);
            &form_data.form.insert(name, file_id);
        }
    }

    Ok(form_data)
}
