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
use crate::utils::defaults::DEFAULT_TEXT_ITEM;
use crate::utils::types::{HTMLResult, RUMString, SharedAppState, URLParams, URLPath};
use crate::{
    rumtk_web_get_param, rumtk_web_get_text_item, rumtk_web_render_component,
    rumtk_web_render_html, RUMWebTemplate,
};
use askama::Template;

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
    let page: RUMString =
        rumtk_web_get_param!(path_components, 0, RUMString::from(DEFAULT_TEXT_ITEM));
    let theme = rumtk_web_get_text_item!(params, "theme", DEFAULT_TEXT_ITEM);

    //Let's render the header and footer
    //<div class="" hx-get="/component/navbar" hx-target="#navbar" hx-trigger="load" id="navbar"></div>
    let header = rumtk_web_render_component!("header", [("", "")], state);
    let main = rumtk_web_render_component!("main", [("", "")], state);
    //<div class="" hx-get="/component/footer?social_list=linkedin,github" hx-target="#footer" hx-trigger="load" id="footer"></div>
    let footer = rumtk_web_render_component!(
        "footer",
        [(
            "social_list",
            state
                .read()
                .expect("Lock failure")
                .get_config()
                .footer_conf
                .socials_list
                .clone()
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
