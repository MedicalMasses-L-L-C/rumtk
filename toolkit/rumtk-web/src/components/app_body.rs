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
use crate::components::COMPONENTS;
use crate::utils::defaults::DEFAULT_TEXT_ITEM;
use crate::utils::types::{HTMLResult, RUMString, SharedAppConf, URLParams, URLPath};
use crate::{mm_collect_page, mm_get_param, mm_get_text_item, mm_render_component, mm_render_html};
use askama::Template;

#[derive(Template)]
#[template(
    source = "
    {% for element in elements %}
        {{ element|safe }}
    {% endfor %}
    ",
    ext = "html"
)]
pub struct AppBodyContents<'a> {
    elements: &'a [RUMString],
}

fn app_body_contents(elements: &[RUMString]) -> HTMLResult {
    mm_render_html!(AppBodyContents { elements })
}

#[derive(Template)]
#[template(
    source = "
        <body class='f12 {{theme}}'>
            {{header|safe}}
            <div class='' id='content'>
                <div class='gap20'>
    
                </div>
                {{body|safe}}
                <div class='gap5'>
    
                </div>
            </div>
            {{footer|safe}}
        </body>
    ",
    ext = "html"
)]
pub struct AppBody {
    theme: RUMString,
    header: RUMString,
    body: RUMString,
    footer: RUMString,
}

pub fn app_body(path_components: URLPath, params: URLParams, state: SharedAppConf) -> HTMLResult {
    let page: RUMString = mm_get_param!(path_components, 0, RUMString::from(DEFAULT_TEXT_ITEM));
    let theme = mm_get_text_item!(params, "theme", DEFAULT_TEXT_ITEM);

    //Let's render the body to html
    let body_components = mm_collect_page!(page, state);
    let body = app_body_contents(&body_components)?.0;

    //Let's render the header and footer
    //<div class="" hx-get="/component/navbar" hx-target="#navbar" hx-trigger="load" id="navbar"></div>
    let header = mm_render_component!("navbar", [("", "")], state, COMPONENTS);
    //<div class="" hx-get="/component/footer?social_list=linkedin,github" hx-target="#footer" hx-trigger="load" id="footer"></div>
    let footer = mm_render_component!(
        "footer",
        [("social_list", "linkedin,github")],
        state,
        COMPONENTS
    );

    mm_render_html!(AppBody {
        theme: RUMString::from(theme),
        header,
        body,
        footer
    })
}
