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
use crate::utils::types::{SharedAppState, URLParams, URLPath};
use crate::{rumtk_web_get_config, ComponentResult, RUMWebTemplate, RUMWebTemplateSafe};
use rumtk_core::strings::RUMString;

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        <script src='https://{{portal}}.youtrack.cloud/static/simplified/form/form-entry.js?auto=false'></script>
        <div style='position: fixed; bottom: 20px; right: 20px;'></div>
        <script>
        YTFeedbackForm.renderFeedbackButton(
        document.currentScript.previousElementSibling,
        {backendURL: 'https://{{portal}}.youtrack.cloud', formUUID: '2cf86344-cf91-4511-a292-de40ae00fc19', theme: '{{theme}}', language: '{{lang}}'}
        );
        </script>
    ",
    ext = "html"
)]
pub struct YouTrack {
    portal: RUMString,
    theme: RUMString,
    lang: RUMString,
}

impl RUMWebTemplateSafe for YouTrack {}

pub fn youtrack<'a>(_path_components: URLPath<'a, 'a>, params: URLParams<'a>, state: SharedAppState) -> ComponentResult<YouTrack> {
    let portal = match rumtk_web_get_config!(state).router.get_service_route(&"youtrack".to_string()){
        Some(portal) => portal.clone(),
        None => RUMString::default(),
    };
    let theme = rumtk_web_get_config!(state).theme.clone();
    let lang = rumtk_web_get_config!(state).lang.clone();

    Ok(YouTrack {
        portal,
        theme,
        lang,
    })
}

