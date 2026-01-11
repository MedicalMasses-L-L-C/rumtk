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
use crate::utils::defaults::{
    DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_SOCIAL_LIST,
};
use crate::utils::types::{HTMLResult, RUMString, SharedAppConf, URLParams, URLPath};
use crate::{mm_get_text_item, mm_render_component, mm_render_html};
use askama::Template;

#[derive(Debug, Clone)]
struct FooterItem {
    typ: RUMString,
    icon_url: RUMString,
    text: RUMString,
}

#[derive(Debug, Clone)]
struct FooterSection {
    typ: RUMString,
    items: Vec<FooterItem>,
}

#[derive(Template, Debug, Clone)]
#[template(
    source = "
        <style>

        </style>
        {% if custom_css_enabled %}
            <link href='/static/components/footer.css' rel='stylesheet'>
        {% endif %}
        <div class='footer-{{ css_class }}-container'>
            <p class='f16'>
                {{company}} &copy; {{copyright_year}}
            </p>
            {{button|safe}}
            {{socials|safe}}
        </div>
    ",
    ext = "html"
)]
pub struct Footer {
    company: RUMString,
    copyright_year: i32,
    button: RUMString,
    socials: RUMString,
    css_class: RUMString,
    custom_css_enabled: bool,
}

pub fn footer(path_components: URLPath, params: URLParams, state: SharedAppConf) -> HTMLResult {
    let social_list = mm_get_text_item!(params, PARAMS_SOCIAL_LIST, DEFAULT_NO_TEXT);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let custom_css_enabled = state.lock().expect("Lock failure").custom_css;

    let contact_button = mm_render_component!(
        "contact_button",
        [
            ("type", "contact"),
            ("function", "goto_contact"),
            ("class", "centered")
        ],
        state,
        COMPONENTS
    );
    let socials =
        mm_render_component!("socials", [("social_list", social_list)], state, COMPONENTS);

    mm_render_html!(Footer {
        button: contact_button,
        socials,
        css_class: RUMString::from(css_class),
        custom_css_enabled
    })
}
