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
pub use super::conf::*;
use axum::extract::{Multipart, Path, Query};
use axum::response::Html;
use phf::Map;
use std::collections::HashMap;
use std::sync::Arc;

pub use rumtk_core::strings::RUMString;
pub use rumtk_core::strings::{CompactStringExt, RUMStringConversions, UTFStringExtensions};

pub type RUMWebData = HashMap<RUMString, RUMString>;
pub type URLPath<'a, 'b> = &'a [&'b str];
pub type AsyncURLPath = Arc<Vec<RUMString>>;
pub type URLParams<'a> = &'a RUMWebData;
pub type AsyncURLParams = Arc<RUMWebData>;
pub type HTMLResult = Result<Html<String>, RUMString>;
pub type RenderedPageComponents = Vec<RUMString>;
/* Router Match Types */
pub type RouterComponents = Path<Vec<RUMString>>;
pub type RouterParams = Query<RUMWebData>;
pub type RouterForm = Multipart;

/* Config Types */
pub type ComponentFunction = fn(URLPath, URLParams, SharedAppConf) -> HTMLResult;
pub type PageFunction = fn(SharedAppConf) -> RenderedPageComponents;
pub type ComponentMap = Map<&'static str, ComponentFunction>;
pub type PageMap = Map<&'static str, PageFunction>;
