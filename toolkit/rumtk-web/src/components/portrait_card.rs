/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D. <lsantos@medicalmasses.com>
 * Copyright (C) 2025  Ethan Dixon
 * Copyright (C) 2025  MedicalMasses L.L.C. <contact@medicalmasses.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_SECTION, PARAMS_TYPE};
use crate::utils::types::{HTMLResult, RUMString, SharedAppState, URLParams, URLPath};
use crate::{
    rumtk_web_get_config, rumtk_web_get_config_string, rumtk_web_get_text_item, rumtk_web_render_component,
    rumtk_web_render_template, RUMWebData, RUMWebTemplate,
};
use rumtk_core::core::RUMResult;
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

fn get_portrait_grid(section: &str, typ: &str, app_state: &SharedAppState) -> RUMResult<PortraitGrid> {
    let text_conf = rumtk_web_get_config_string!(app_state, typ);

    let mut grid = Vec::with_capacity(text_conf.len());
    for (_r_name, r_list) in text_conf {
        let mut grid_row = Vec::with_capacity(r_list.len());
        for (i_name, _i_item) in r_list {
            let item = rumtk_web_render_component!(
                "contact_card",
                [
                    ("section", section),
                    (PARAMS_TYPE, i_name.as_str()),
                ],
                app_state
            )?.to_string();
            grid_row.push(item);
        }
        grid.push(grid_row);
    }
    Ok(grid)
}

pub fn portrait_card(
    _path_components: URLPath,
    params: URLParams,
    state: SharedAppState,
) -> HTMLResult {
    let section = rumtk_web_get_text_item!(params, PARAMS_SECTION, DEFAULT_TEXT_ITEM);
    let typ = rumtk_web_get_text_item!(params, PARAMS_TYPE, DEFAULT_TEXT_ITEM);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);
    let icon_data = get_portrait_grid(section, typ, &state)?;

    let custom_css_enabled = rumtk_web_get_config!(state).custom_css;

    rumtk_web_render_template!(PortraitCard {
        icon_data,
        css_class: RUMString::from(css_class),
        custom_css_enabled
    })
}
