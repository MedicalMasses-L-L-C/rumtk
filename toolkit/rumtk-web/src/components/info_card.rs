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
    DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, OPT_INVERTED_DIRECTION, PARAMS_CSS_CLASS, PARAMS_INVERTED,
    PARAMS_ITEM, SECTION_TEXT,
};
use crate::utils::types::{HTMLResult, RUMString, SharedAppState, URLParams, URLPath};
use crate::utils::DEFAULT_TEXTMAP;
use crate::{
    rumtk_web_get_param_eq, rumtk_web_get_string, rumtk_web_get_text_item, rumtk_web_render_html,
};
use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(
    source = "
        {% if custom_css_enabled %}
            <link href='/static/components/info_card.css' rel='stylesheet'>
        {% endif %}
        <div class='info-card-{{ css_class }}-container'>
            {% if inverted %}
                <pre class='info-card-{{ css_class }}-descbox'>
                    {{ description }}
                </pre>
                <div class='f18 info-card-{{ css_class }}-titlebox'>
                    {{ title }}
                </div>
            {% else %}
                <div class='f18 info-card-{{ css_class }}-titlebox'>
                    {{ title }}
                </div>
                <pre class='info-card-{{ css_class }}-descbox'>
                    {{ description }}
                </pre>
            {% endif %}
        </div>
    ",
    ext = "html"
)]
pub struct InfoCard<'a> {
    title: &'a str,
    description: &'a str,
    inverted: bool,
    css_class: RUMString,
    custom_css_enabled: bool,
}

pub fn info_card(
    _path_components: URLPath,
    params: URLParams,
    state: SharedAppState,
) -> HTMLResult {
    let card_text_item = rumtk_web_get_text_item!(params, PARAMS_ITEM, DEFAULT_TEXT_ITEM);
    let inverted = rumtk_web_get_param_eq!(params, PARAMS_INVERTED, OPT_INVERTED_DIRECTION, false);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let custom_css_enabled = state.read().expect("Lock failure").config.custom_css;

    let text_store = rumtk_web_get_string!(state, SECTION_TEXT);
    let itm = rumtk_web_get_text_item!(&text_store, card_text_item, &DEFAULT_TEXTMAP());
    let title = rumtk_web_get_text_item!(&itm, "title", DEFAULT_NO_TEXT);
    let desc = rumtk_web_get_text_item!(&itm, "description", DEFAULT_NO_TEXT);

    rumtk_web_render_html!(InfoCard {
        title,
        description: desc,
        inverted,
        css_class: RUMString::from(css_class),
        custom_css_enabled
    })
}
