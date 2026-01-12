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
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_TYPE, SECTION_SERVICES};
use crate::utils::types::{HTMLResult, RUMString, SharedAppConf, TextMap, URLParams, URLPath};
use crate::{rumtk_web_get_conf, rumtk_web_get_text_item, rumtk_web_render_html};
use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(
    source = "
        <style>

        </style>
        {% if custom_css_enabled %}
            <link href='/static/components/item_card.css' rel='stylesheet'>
        {% endif %}
        <div class='item-card-{{css_class}}-container'>
            {% for (service_name, service_description) in services %}
            <div>
                <details>
                    <summary class='f16 item-card-{{css_class}}-title'>
                        {{ service_name.to_uppercase() }}
                    </summary>
                    <pre class='item-card-{{css_class}}-details'>
                        {{ service_description }}
                    </pre>
                </details>
            </div>
            {% endfor %}
        </div>
    ",
    ext = "html"
)]
pub struct ItemCard {
    services: TextMap,
    css_class: RUMString,
    custom_css_enabled: bool,
}

pub fn item_card(path_components: URLPath, params: URLParams, state: SharedAppConf) -> HTMLResult {
    let typ = rumtk_web_get_text_item!(params, PARAMS_TYPE, SECTION_SERVICES);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);
    let services = rumtk_web_get_conf!(state, typ);

    let custom_css_enabled = state.lock().expect("Lock failure").custom_css;

    rumtk_web_render_html!(ItemCard {
        services,
        css_class: RUMString::from(css_class),
        custom_css_enabled
    })
}
