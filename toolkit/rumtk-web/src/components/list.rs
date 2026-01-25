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
use crate::defaults::{DEFAULT_NO_TEXT, PARAMS_ID};
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_TYPE, SECTION_SERVICES};
use crate::utils::types::{HTMLResult, SharedAppState, TextMap, URLParams, URLPath};
use crate::{rumtk_web_get_conf, rumtk_web_get_text_item, rumtk_web_render_html, RUMWebTemplate};

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        {% if custom_css_enabled %}
            <link href='/static/components/list.css' rel='stylesheet'>
        {% endif %}
        <div class='item-{{css_class}}-container'>
            {% for (service_name, service_description) in services %}
            <div>
                <details>
                    <summary class='f16 item-{{css_class}}-title'>
                        {{ service_name.to_uppercase() }}
                    </summary>
                    <pre class='item-{{css_class}}-details'>
                        {{ service_description }}
                    </pre>
                </details>
            </div>
            {% endfor %}
        </div>
    ",
    ext = "html"
)]
pub struct List<'a> {
    items: &'a TextMap,
    css_class: &'a str,
    custom_css_enabled: bool,
}

pub fn list(_path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let typ = rumtk_web_get_text_item!(params, PARAMS_TYPE, SECTION_SERVICES);
    let clipboard_id = rumtk_web_get_text_item!(params, PARAMS_ID, DEFAULT_NO_TEXT);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let items = match state.read().unwrap().clipboard.get(clipboard_id) {
        Some(items) => items,
        None => &rumtk_web_get_conf!(state, typ),
    };

    let custom_css_enabled = state.read().expect("Lock failure").config.custom_css;

    rumtk_web_render_html!(List {
        items,
        css_class,
        custom_css_enabled
    })
}
