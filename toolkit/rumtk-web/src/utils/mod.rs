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
pub use rumtk_core::strings::{RUMString, RUMStringConversions};

pub mod app;
pub mod conf;
pub mod defaults;
pub mod form_data;
pub mod jobs;
pub mod matcher;
pub mod packaging;
pub mod render;
pub mod response;
pub mod testdata;
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
        use $crate::RUMWebDataProxy;
        RUMWebDataProxy::from($params)
    }};
}
