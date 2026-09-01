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
use crate::components::html::{script, Script};
use crate::{rumtk_web_get_config, ComponentResult, RUMWebTemplate, RUMWebTemplateSafe, SharedAppState, TextMap, DEFAULT_SCRIPT_IMPORT, DEFAULT_TEXT};
use rumtk_core::strings::RUMString;
use std::sync::LazyLock;

static DEFAULT_GCAPTCHA_JS_URL: LazyLock<RUMString> = LazyLock::new(|| "https://www.google.com/recaptcha/enterprise.js".to_string());

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        {{import|safe}}
        <div
            class='g-recaptcha'
            data-sitekey='{{site_key}}'
            data-action='LOGIN'
            data-theme='{{theme}}'
            data-callback='enableSubmitButton'
            data-expired-callback='disableSubmitButton'
            >
        </div>
        <br/>
        <script>
          function enableSubmitButton() {
            document.getElementById('{{widget_id}}').disabled = false;
          }

          function disableSubmitButton() {
            document.getElementById('{{widget_id}}').disabled = true;
          }
        </script>
    ",
    ext = "html"
)]
pub struct GoogleCaptcha {
    theme: RUMString,
    widget_id: RUMString,
    import: Script,
    site_key: RUMString,
}

impl RUMWebTemplateSafe for GoogleCaptcha {}

pub fn gcaptcha<'a>(widget_id: &str, captcha_config: &TextMap, state: SharedAppState) -> ComponentResult<GoogleCaptcha> {
    let theme = rumtk_web_get_config!(state).theme.clone();
    let site_key = match captcha_config.get("site-key") {
        Some(key_site) => key_site.clone(),
        None => DEFAULT_TEXT.to_string()
    };
    let import_url = match captcha_config.get("import-url") {
        Some(import_url) => import_url.clone(),
        None => (*DEFAULT_GCAPTCHA_JS_URL).clone(),
    };
    let import = script(DEFAULT_SCRIPT_IMPORT, &import_url, false)?;

    Ok(GoogleCaptcha {
        theme,
        widget_id: widget_id.to_string(),
        import,
        site_key,
    })
}
