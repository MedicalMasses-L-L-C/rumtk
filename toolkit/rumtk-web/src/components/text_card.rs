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
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_TYPE};
use crate::utils::types::{HTMLResult, RUMString, SharedAppState, URLParams, URLPath};
use crate::{rumtk_web_get_text_item, rumtk_web_render_component, rumtk_web_render_html};
use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(
    source = "
        <style>
            .text-card-default {
                max-width: 1700px;
                padding: 20px;
                background-color: var(--color-indigo);

                border-radius: 15px;
            }
        </style>
        {% if custom_css_enabled %}
            <link href='/static/components/text_card.css' rel='stylesheet'>
        {% endif %}
        <div class='centered text-card-{{css_class}}'>
          {{formatted_label|safe}}
        </div>
    ",
    ext = "html"
)]
pub struct TextCard {
    formatted_label: RUMString,
    css_class: RUMString,
    custom_css_enabled: bool,
}

pub fn text_card(
    _path_components: URLPath,
    params: URLParams,
    state: SharedAppState,
) -> HTMLResult {
    let typ = rumtk_web_get_text_item!(params, PARAMS_TYPE, DEFAULT_TEXT_ITEM);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let custom_css_enabled = state.read().expect("Lock failure").config.custom_css;

    let formatted_label = rumtk_web_render_component!("formatted_label", [("type", typ)], state);

    rumtk_web_render_html!(TextCard {
        formatted_label,
        css_class: RUMString::from(css_class),
        custom_css_enabled
    })
}
