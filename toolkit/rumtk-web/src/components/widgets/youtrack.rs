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
use crate::{rumtk_web_get_config, ComponentResult, RUMWebTemplate, RUMWebTemplateSafe, DEFAULT_TEXTMAP};
use rumtk_core::strings::RUMString;

pub const YOUTRACK_SANITIZER_TAGS: &str = "script";
pub const YOUTRACK_SANITIZER_ATTRIBUTES: &[&str] = &[
    "data-yt-url",
    "data-theme",
    "data-lang",
];

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        <script
            id='2cf86344-cf91-4511-a292-de40ae00fc19'
            data-yt-url='https://{{portal}}.youtrack.cloud'
            src='https://{{portal}}.youtrack.cloud/static/simplified/form/form-entry.js'
            data-theme='{{theme}}'
            data-lang='{{lang}}'
            async
            defer
            >
        </script>
    ",
    ext = "html"
)]
pub struct YouTrack {
    portal: RUMString,
    uuid: RUMString,
    theme: RUMString,
    lang: RUMString,
}

impl RUMWebTemplateSafe for YouTrack {}

pub fn youtrack<'a>(_path_components: URLPath<'a, 'a>, params: URLParams<'a>, state: SharedAppState) -> ComponentResult<YouTrack> {
    let route_info = match rumtk_web_get_config!(state).router.get_service_route(&"youtrack".to_string()){
        Some(portal) => portal.clone(),
        None => DEFAULT_TEXTMAP.clone(),
    };
    let portal = route_info.get("portal").unwrap().clone();
    let uuid = route_info.get("uuid").unwrap().clone();
    let theme = rumtk_web_get_config!(state).theme.clone();
    let lang = rumtk_web_get_config!(state).lang.clone();

    Ok(YouTrack {
        portal,
        uuid,
        theme,
        lang,
    })
}

