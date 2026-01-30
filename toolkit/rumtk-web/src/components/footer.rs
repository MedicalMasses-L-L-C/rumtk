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
use crate::utils::defaults::{
    DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_SOCIAL_LIST,
};
use crate::utils::types::{HTMLResult, RUMString, SharedAppState, URLParams, URLPath};
use crate::{
    rumtk_web_get_text_item, rumtk_web_render_component, rumtk_web_render_html, RUMWebTemplate,
};
use askama::Template;

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        {% if custom_css_enabled %}
            <link href='/static/components/footer.css' rel='stylesheet'>
        {% endif %}
        <footer role='contentinfo' class='footer-{{ css_class }}-container'>
            <p class='f16'>
                {{company}} &copy; {{copyright}}
            </p>
            {{button|safe}}
            {{socials|safe}}
        </footer>
    ",
    ext = "html"
)]
pub struct Footer {
    company: RUMString,
    copyright: RUMString,
    button: RUMString,
    socials: RUMString,
    css_class: RUMString,
    custom_css_enabled: bool,
}

pub fn footer(_path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let social_list = rumtk_web_get_text_item!(params, PARAMS_SOCIAL_LIST, DEFAULT_NO_TEXT);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let custom_css_enabled = state.read().expect("Lock failure").get_config().custom_css;
    let company = state
        .read()
        .expect("Lock failure")
        .get_config()
        .company
        .clone();
    let copyright = state
        .read()
        .expect("Lock failure")
        .get_config()
        .copyright
        .clone();

    let contact_button = match state
        .read()
        .expect("Lock failure")
        .get_config()
        .footer_conf
        .disable_contact_button
    {
        true => RUMString::default(),
        false => rumtk_web_render_component!(
            "contact_button",
            [
                ("type", "contact"),
                ("function", "goto_contact"),
                ("class", "centered")
            ],
            state
        ),
    };
    let socials = rumtk_web_render_component!("socials", [("social_list", social_list)], state);

    rumtk_web_render_html!(Footer {
        company,
        copyright,
        button: contact_button,
        socials,
        css_class: RUMString::from(css_class),
        custom_css_enabled
    })
}
