/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2026  Luis M. Santos, M.D.
 * Copyright (C) 2026  MedicalMasses L.L.C.
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
use crate::utils::types::HTMLResult;
use axum::response::Html;
use rumtk_core::strings::{rumtk_format, RUMStringConversions};

pub fn html_render<T: askama::Template>(template: T) -> HTMLResult {
    let result = template.render();
    match result {
        Ok(html) => Ok(Html(html.to_rumstring())),
        Err(e) => {
            let tn = std::any::type_name::<T>();
            Err(rumtk_format!("Template {tn} render failed: {e:?}"))
        }
    }
}
