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
pub use super::conf::*;
use axum::extract::{Multipart, Path, Query};
use phf::Map;
pub use rumtk_core::strings::RUMString;
pub use rumtk_core::strings::{AsStr, CompactStringExt, RUMStringConversions, StringLike};
use rumtk_core::types::RUMHashMap;
use std::fmt::{Debug, Display};
use std::sync::Arc;

pub type RUMWebData = RUMHashMap<RUMString, RUMString>;
pub type URLPath<'a, 'b> = &'a [&'b str];
pub type AsyncURLPath = Arc<Vec<RUMString>>;
pub type URLParams<'a> = &'a RUMWebData;
pub type AsyncURLParams = Arc<RUMWebData>;

/* Responses */
pub use crate::utils::response::*;

pub type RenderedPageComponents = Vec<RUMString>;
pub type RenderedPageComponentsResult = RUMResult<RenderedPageComponents>;
/* Router Match Types */
pub type RouterComponents = Path<Vec<RUMString>>;
pub type RouterParams = Query<RUMWebData>;
pub type RouterForm = Multipart;

/* Config Types */
pub type ComponentFunction = fn(URLPath, URLParams, SharedAppState) -> HTMLResult;
pub type PageFunction = fn(SharedAppState) -> RenderedPageComponentsResult;
pub type ComponentMap = Map<&'static str, ComponentFunction>;
pub type PageMap = Map<&'static str, PageFunction>;

/* API Types */
pub use crate::utils::form_data::{FormBuffer, FormData};
pub type RouterAPIPath = Path<RUMString>;
pub type APIPath = RUMString;
pub type APIFunction = fn(APIPath, RUMWebData, FormData, SharedAppState) -> HTMLResult;

pub use askama::Template as RUMWebTemplate;
use rumtk_core::core::RUMResult;


///
/// ```
/// use rumtk_web::RUMWebDataProxy;
///
/// let data = [("", "")];
/// let result = RUMWebDataProxy::from(&data);
///
/// ```
///
pub struct RUMWebDataProxy(RUMWebData);

impl RUMWebDataProxy {
    pub fn get_inner(&self) -> &RUMWebData {
        &self.0
    }

    pub fn from_params<K, V, const N: usize>(data: &[(K, V); N]) -> Self
    where
        K: Display + Sized,
        V: Display + Sized,
    {
        let mut new_params = RUMWebData::with_capacity(data.len());

        for (k, v) in data.iter() {
            new_params.insert(
                RUMString::from(k.to_string()),
                RUMString::from(v.to_string()),
            );
        }
        RUMWebDataProxy(new_params)
    }
}

impl From<&RUMWebData> for RUMWebDataProxy {
    fn from(data: &RUMWebData) -> Self {
        RUMWebDataProxy(data.clone())
    }
}

impl<const N: usize> From<&&[(&str, &str); N]> for RUMWebDataProxy {
    fn from(data: &&[(&str, &str); N]) -> Self {
        Self::from_params(data)
    }
}

impl<const N: usize> From<&[(&str, &str); N]> for RUMWebDataProxy {
    fn from(data: &[(&str, &str); N]) -> Self {
        Self::from_params(data)
    }
}

impl<const N: usize> From<&[(&str, &RUMString); N]> for RUMWebDataProxy {
    fn from(data: &[(&str, &RUMString); N]) -> Self {
        Self::from_params(data)
    }
}
