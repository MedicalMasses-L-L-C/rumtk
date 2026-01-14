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
use crate::utils::defaults::{
    DEFAULT_CONTACT_ITEM, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_SECTION, PARAMS_TYPE,
    SECTION_CONTACT,
};
use crate::utils::types::{HTMLResult, RUMString, SharedAppConf, TextMap, URLParams, URLPath};
use crate::utils::{DEFAULT_NESTEDTEXTMAP, DEFAULT_TEXTMAP};
use crate::{rumtk_web_get_string, rumtk_web_get_text_item, rumtk_web_render_html};
use askama::Template;

#[derive(Template, Debug)]
#[template(
    source = "
        <style>
            .contact-card-default-container {
            }

            .contact-card-default-container > p {
                margin: 0;
            }

            .contact-card-centered-container {
                max-width: fit-content;
                margin-inline: auto;
            }

            .contact-card-centered-container > p {
                margin: 0;
            }
        </style>
        {% if custom_css_enabled %}
            <link href='/static/components/contact_card.css' rel='stylesheet'>
        {% endif %}
        <div class='f14 centered'>
            <div class='f18 contact-card-{{ css_class }}-container'>
                {% for (details_typ, details_data) in contact_lines %}
                    {% if details_typ == &\"phrase\" && !details_data.is_empty() %}
                    <p class='italics f18' >
                        '{{ details_data }}'
                    </p>
                    {% else if details_typ == &\"email\" && !details_data.is_empty() %}
                    <p>
                        <a  class=' f14 no-text-color' href='mailto:{{ details_data }}'>{{ details_data }}</a>
                    </p>
                    {% else if details_typ == &\"phone\" && !details_data.is_empty() %}
                    <p>
                        <a  class='f14 no-text-color' href='tel:{{ details_data }}'>{{ details_data }}</a>
                    </p>
                    {% else if !details_data.is_empty() %}
                    <p class='f14' >
                        {{ details_data }}
                    </p>
                    {% endif %}
                {% endfor %}
            </div>
        </div>
    ",
    ext = "html"
)]
pub struct ContactCard<'a> {
    contact_lines: &'a TextMap,
    css_class: RUMString,
    custom_css_enabled: bool,
}

pub fn contact_card(
    _path_components: URLPath,
    params: URLParams,
    state: SharedAppConf,
) -> HTMLResult {
    let section = rumtk_web_get_text_item!(params, PARAMS_SECTION, DEFAULT_CONTACT_ITEM);
    let typ = rumtk_web_get_text_item!(params, PARAMS_TYPE, DEFAULT_CONTACT_ITEM);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let custom_css_enabled = state.lock().expect("Lock failure").custom_css;

    let text_conf = rumtk_web_get_string!(state, SECTION_CONTACT);
    let contact_item = rumtk_web_get_text_item!(&text_conf, section, &DEFAULT_NESTEDTEXTMAP());
    let contact_lines: &TextMap = rumtk_web_get_text_item!(&contact_item, typ, &DEFAULT_TEXTMAP());

    rumtk_web_render_html!(ContactCard {
        contact_lines,
        css_class: RUMString::from(css_class),
        custom_css_enabled
    })
}
