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
use crate::components::app::app_body::AppBody;
use crate::components::app::app_head::AppShellHead;
use crate::components::{app::app_body::app_body, app::app_head::app_head};
use crate::defaults::DEFAULT_THEME_ITEM;
use crate::utils::defaults::LANG_EN;
use crate::utils::types::{RUMString, SharedAppState, URLParams, URLPath};
use crate::{rumtk_web_get_text_item, rumtk_web_set_config, ComponentResult, RUMWebTemplate, RUMWebTemplateSafe};

#[derive(RUMWebTemplate)]
#[template(
    source = "
        <!DOCTYPE html>
        <html lang='{{lang}}'>
            {{head}}
            {{body}}
        </html>
    ",
    ext = "html"
)]
pub struct AppShell {
    head: AppShellHead,
    lang: RUMString,
    body: AppBody,
}

impl RUMWebTemplateSafe for AppShell {}

pub fn app_shell<'a>(path_components: URLPath<'a, 'a>, params: URLParams<'a>, state: SharedAppState) -> ComponentResult<AppShell> {
    let lang = rumtk_web_get_text_item!(params, "lang", LANG_EN).to_string();
    let theme = rumtk_web_get_text_item!(params, "theme", DEFAULT_THEME_ITEM).to_string();
    // TODO: We need to reevaluate how to validate the options that should be standardized to avoid parameter injection as an attack vector.
    //owned_state.opts = *params.clone();

    //Config App
    rumtk_web_set_config!(state).lang = lang.clone();
    rumtk_web_set_config!(state).theme = theme;

    Ok(AppShell {
        lang,
        head: app_head(path_components, params, state.clone())?,
        body: app_body(path_components, params, state.clone())?
    })
}
