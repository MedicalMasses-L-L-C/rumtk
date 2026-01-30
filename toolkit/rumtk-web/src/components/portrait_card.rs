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
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_SECTION, PARAMS_TYPE};
use crate::utils::types::{HTMLResult, RUMString, SharedAppState, URLParams, URLPath};
use crate::{
    rumtk_web_get_string, rumtk_web_get_text_item, rumtk_web_render_component,
    rumtk_web_render_html, RUMWebData, RUMWebTemplate,
};
use askama::Template;
use rumtk_core::strings::RUMStringConversions;

type PortraitGrid = Vec<Vec<RUMString>>;

#[derive(RUMWebTemplate, Debug)]
#[template(
    source = "
        {% if custom_css_enabled %}
            <link href='/static/components/portrait_card.css' rel='stylesheet'>
        {% endif %}
        <div class='centered twothird-width portrait-card-{{ css_class }}-container'>
            <table>
                <thead></thead>
                <tbody>
                {% for row in icon_data %}
                    <tr class='portrait-card-{{ css_class }}-row'>
                        {% for item in row %}
                        <td class='portrait-card-{{ css_class }}-item'>
                            {{item|safe}}
                        </td>
                        {% endfor %}
                    </tr>
                {% endfor %}
                </tbody>
            </table>
        </div>
    ",
    ext = "html"
)]
pub struct PortraitCard {
    icon_data: PortraitGrid,
    css_class: RUMString,
    custom_css_enabled: bool,
}

fn get_portrait_grid(section: &str, typ: &str, app_state: &SharedAppState) -> PortraitGrid {
    let text_conf = rumtk_web_get_string!(app_state, typ);

    let mut grid = Vec::with_capacity(text_conf.len());
    for (_r_name, r_list) in text_conf {
        let mut grid_row = Vec::with_capacity(r_list.len());
        for (i_name, _i_item) in r_list {
            let item = rumtk_web_render_component!(
                "contact_card",
                [
                    ("section".to_rumstring(), section.to_rumstring()),
                    ("type".to_rumstring(), i_name.clone()),
                ],
                app_state.clone()
            );
            grid_row.push(item);
        }
        grid.push(grid_row);
    }
    grid
}

pub fn portrait_card(
    _path_components: URLPath,
    params: URLParams,
    state: SharedAppState,
) -> HTMLResult {
    let section = rumtk_web_get_text_item!(params, PARAMS_SECTION, DEFAULT_TEXT_ITEM);
    let typ = rumtk_web_get_text_item!(params, PARAMS_TYPE, DEFAULT_TEXT_ITEM);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);
    let icon_data = get_portrait_grid(section, typ, &state);

    let custom_css_enabled = state.read().expect("Lock failure").get_config().custom_css;

    rumtk_web_render_html!(PortraitCard {
        icon_data,
        css_class: RUMString::from(css_class),
        custom_css_enabled
    })
}
