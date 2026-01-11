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
use crate::utils::types::RUMString;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

///
/// This is a core structure in a web project using the RUMTK framework. This structure contains
/// a series of fields that represent the web app initial state or configuration. The idea is that
/// the web app can come bundled with a JSON config file following this structure which we can load
/// at runtime. The settings will dictate a few key project behaviors such as properly labeling
/// some components with the company name or use the correct language text.
///
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct AppConf {
    pub title: RUMString,
    pub description: RUMString,
    pub lang: RUMString,
    pub theme: RUMString,
    pub custom_css: bool,
    //pub opts: TextMap,
}

impl AppConf {
    pub fn default() -> Self {
        Self::new(
            RUMString::from(""),
            RUMString::from(""),
            RUMString::from("en"),
            RUMString::from(""),
            false,
        )
    }

    pub fn default_site(title: RUMString, description: RUMString, custom_css: bool) -> Self {
        Self::new(
            title,
            description,
            RUMString::from("en"),
            RUMString::from(""),
            custom_css,
        )
    }

    pub fn new(
        title: RUMString,
        description: RUMString,
        lang: RUMString,
        theme: RUMString,
        custom_css: bool,
    ) -> Self {
        Self {
            title,
            description,
            lang,
            theme,
            custom_css,
        }
    }
}

pub type SharedAppConf = Arc<Mutex<AppConf>>;
pub type RouterAppConf = State<Arc<Mutex<AppConf>>>;
