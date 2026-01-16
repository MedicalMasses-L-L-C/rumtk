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
pub mod app;
pub mod conf;
pub mod defaults;
pub mod matcher;
pub mod packaging;
pub mod render;
pub mod types;

pub use render::*;
pub use types::*;

#[macro_export]
macro_rules! rumtk_web_get_text_item {
    ( $store:expr, $item:expr, $default:expr) => {{
        match $store.get($item) {
            Some(x) => x,
            None => $default,
        }
    }};
}

#[macro_export]
macro_rules! rumtk_web_get_param_eq {
    ( $params:expr, $indx:expr, $comparison:expr, $default:expr ) => {{
        match $params.get($indx) {
            Some(x) => *x == $comparison,
            None => $default,
        }
    }};
}

#[macro_export]
macro_rules! rumtk_web_get_param {
    ( $params:expr, $indx:expr, $default:expr ) => {{
        match $params.get($indx) {
            Some(x) => x.parse().unwrap_or($default),
            None => $default,
        }
    }};
}

#[macro_export]
macro_rules! rumtk_web_params_map {
    ( $params:expr ) => {{
        use std::collections::HashMap;
        let mut params = HashMap::<RUMString, RUMString>::with_capacity($params.len());

        for (k, v) in $params.iter() {
            params.insert(
                RUMString::from(k.to_string()),
                RUMString::from(v.to_string()),
            );
        }
        params
    }};
}

#[macro_export]
macro_rules! rumtk_web_fetch {
    ( $matcher:expr ) => {{
        use axum::extract::{Path, Query, State};
        use axum::response::Html;
        use $crate::utils::types::{RouterAppConf, RouterComponents, RouterParams};

        async |Path(path_params): RouterComponents,
               Query(params): RouterParams,
               State(state): RouterAppConf|
               -> Html<String> {
            match $matcher(path_params, params, state).await {
                Ok(res) => res,
                Err(e) => {
                    error!("{}", e);
                    return Html(String::default());
                }
            }
        }
    }};
}
