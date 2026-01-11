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
    DEFAULT_CONTACT_ITEM, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_FUNCTION, PARAMS_TYPE,
};
use crate::utils::types::{HTMLResult, RUMString, SharedAppConf, URLParams, URLPath};
use crate::{rumtk_web_get_text_item, rumtk_web_render_component, rumtk_web_render_html};
use askama::Template;

#[derive(Template, Debug)]
#[template(
    source = "
        <style>
            .contact-centered-button-container {
                max-width: fit-content;
                margin-inline: auto;

                height: 90px;
            }

            .contact-centered-button {
                background: radial-gradient(circle,var(--color-darkpurple) 0%, var(--color-indigo) 70%);

                color: var(--color-bg-white);

                border-radius: 15px;
            }
        </style>
        {% if custom_css_enabled %}
            <link href='/static/components/contact_button.css' rel='stylesheet'>
        {% endif %}
        <script type='module' id='contact_button'>
            export function goto_contact() {
                window.location.href = './contact';
            }

            // @ts-ignore
            window.goto_contact = goto_contact;
        </script>
        <div class='contact-{{ css_class }}-button-container'>
            <button class='contact-{{ css_class }}-button' onclick='{{ send_function }}()'>
                {{title|safe}}
            </button>
        </div>
    ",
    ext = "html"
)]
pub struct ContactButton {
    title: RUMString,
    typ: RUMString,
    send_function: RUMString,
    css_class: RUMString,
    custom_css_enabled: bool,
}

pub fn contact_button(
    path_components: URLPath,
    params: URLParams,
    state: SharedAppConf,
) -> HTMLResult {
    let typ = rumtk_web_get_text_item!(params, PARAMS_TYPE, DEFAULT_CONTACT_ITEM);
    let send_function = rumtk_web_get_text_item!(params, PARAMS_FUNCTION, DEFAULT_CONTACT_ITEM);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let custom_css_enabled = state.lock().expect("Lock failure").custom_css;

    let title = rumtk_web_render_component!("title", [("type", typ)], state, COMPONENTS);

    rumtk_web_render_html!(ContactButton {
        title,
        typ: RUMString::from(typ),
        send_function: RUMString::from(send_function),
        css_class: RUMString::from(css_class),
        custom_css_enabled
    })
}
