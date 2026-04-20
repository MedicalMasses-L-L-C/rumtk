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
use crate::utils::defaults::DEFAULT_TEXT_ITEM;
use crate::utils::types::{HTMLResult, RUMString, SharedAppState, URLParams, URLPath};
use crate::{
    rumtk_web_conf_get, rumtk_web_get_text_item, rumtk_web_render_component, rumtk_web_render_html,
    AppConf, AppState, RUMWebTemplate,
};
use askama::Template;
use rumtk_core::rumtk_critical_section_read;

#[derive(RUMWebTemplate)]
#[template(
    source = "
        <body class='f12 theme-{{theme}}'>
            <a href='#main-content'>Skip to main content</a>
            {{header|safe}}
            {{main|safe}}
            {{footer|safe}}
        </body>
    ",
    ext = "html"
)]
pub struct AppBody {
    theme: RUMString,
    header: RUMString,
    main: RUMString,
    footer: RUMString,
}

pub fn app_body(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let theme = rumtk_web_get_text_item!(params, "theme", DEFAULT_TEXT_ITEM);

    //Let's render the header and footer
    //<div class="" hx-get="/component/navbar" hx-target="#navbar" hx-trigger="load" id="navbar"></div>
    let header = rumtk_web_render_component!("header", [("", "")], state);
    let main = rumtk_web_render_component!("main", path_components, [("", "")], state);
    //<div class="" hx-get="/component/footer?social_list=linkedin,github" hx-target="#footer" hx-trigger="load" id="footer"></div>
    let footer = rumtk_web_render_component!(
        "footer",
        [(
            "social_list",
            rumtk_web_conf_get!(state, |conf: &AppConf| {
                conf.footer_conf.socials_list.clone()
            })?
        )],
        state
    );

    rumtk_web_render_html!(AppBody {
        theme: RUMString::from(theme),
        header,
        main,
        footer
    })
}
