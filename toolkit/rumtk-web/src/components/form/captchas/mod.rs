/*
 *     rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 *     This toolkit aims to be reliable, simple, performant, and standards compliant.
 *     Copyright (C) 2026  Luis M. Santos, M.D. <lsantos@medicalmasses.com>
 *     Copyright (C) 2026  MedicalMasses L.L.C. <contact@medicalmasses.com>
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
mod gcaptcha;

use super::captchas::gcaptcha::gcaptcha;
use crate::components::form::form_node::{FormNode, ToFormNode};
use crate::utils::types::SharedAppState;
use crate::{rumtk_web_get_config, ComponentResult, RUMWebTemplate, RUMWebTemplateSafe, DEFAULT_TEXTMAP};
use rumtk_core::strings::{rumtk_format, RUMString};
use std::string::ToString;

const DEFAULT_CAPTCHA: &str = "gcaptcha";

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        {{captcha|safe}}
    ",
    ext = "html"
)]
pub struct Captcha {
    captcha: RUMString
}

impl RUMWebTemplateSafe for Captcha {}

impl ToFormNode<Captcha> for Captcha {}

pub fn captcha<'a>(widget_id: &str, state: SharedAppState) -> ComponentResult<FormNode> {
    let captcha_config = rumtk_web_get_config!(state).captcha.clone().unwrap_or_else(|| DEFAULT_TEXTMAP.clone());
    let captcha_type = match captcha_config.get("captcha-type") {
        Some(typ) => &typ,
        None => DEFAULT_CAPTCHA
    };
    match captcha_type {
        "gcaptcha" => {
            let rendered_captcha = gcaptcha(widget_id, &captcha_config, state.clone())?;
            Ok(Captcha {
                captcha: rendered_captcha.to_string()
            }.to_form_node())
        }
        _ => Err(rumtk_format!("Captcha <{captcha_type}> not implemented!")),
    }
}
