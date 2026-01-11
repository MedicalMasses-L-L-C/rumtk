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
use super::conf::*;
use axum::extract::{Path, Query};
use axum::response::Html;
use phf::{Map, OrderedMap};
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

pub type MMString = String;
pub type URLPath<'a, 'b> = &'a [&'b str];
pub type AsyncURLPath = Arc<Vec<MMString>>;
pub type URLParams<'a> = &'a HashMap<MMString, MMString>;
pub type AsyncURLParams = Arc<HashMap<MMString, MMString>>;
pub type HTMLResult = Result<Html<MMString>, MMString>;
pub type RenderedPageComponents = Vec<MMString>;
/* Router Match Types */
pub type RouterComponents = Path<Vec<MMString>>;
pub type RouterParams = Query<HashMap<MMString, MMString>>;

/* Config Types */
pub type ComponentFunction = fn(URLPath, URLParams, SharedAppConf) -> HTMLResult;
pub type PageFunction = fn(SharedAppConf) -> RenderedPageComponents;
pub type AsyncReturn = Arc<Pin<Box<dyn Future<Output = HTMLResult>>>>;
pub type AsyncComponentFunction = fn(AsyncURLPath, AsyncURLParams, SharedAppConf) -> AsyncReturn;
pub type ComponentMap = Map<&'static str, ComponentFunction>;
pub type PageMap = Map<&'static str, PageFunction>;
pub type TextMap = OrderedMap<&'static str, &'static str>;
pub type NestedTextMap = OrderedMap<&'static str, &'static TextMap>;
pub type NestedNestedTextMap = OrderedMap<&'static str, &'static NestedTextMap>;
