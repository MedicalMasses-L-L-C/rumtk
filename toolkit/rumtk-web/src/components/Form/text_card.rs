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
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_TYPE};
use crate::utils::types::{HTMLResult, RUMString, SharedAppConf, URLParams, URLPath};
use crate::{mm_get_text_item, mm_render_html};
use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(
    source = "
        <style>
            .card-default {
                max-width: 1700px;
                padding: 20px;
                background-color: var(--color-indigo);

                border-radius: 15px;
            }
        </style>
        {% if custom_css_enabled %}
            <link href='/static/components/form/text_card.css' rel='stylesheet'>
        {% endif %}
        <div class='centered card-{{css_class}}'>
          <div hx-get='/component/label?type={{typ}}' hx-target='this' hx-trigger='load'> </div>
        </div>
    ",
    ext = "html"
)]
struct TextCard {
    typ: RUMString,
    css_class: RUMString,
    custom_css_enabled: bool,
}

pub fn text_card(path_components: URLPath, params: URLParams, state: SharedAppConf) -> HTMLResult {
    let typ = mm_get_text_item!(params, PARAMS_TYPE, DEFAULT_TEXT_ITEM);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let custom_css_enabled = state.lock().expect("Lock failure").custom_css;

    mm_render_html!(TextCard {
        typ: RUMString::from(typ),
        css_class: RUMString::from(css_class),
        custom_css_enabled
    })
}
