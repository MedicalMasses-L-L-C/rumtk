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
use crate::components::contact_card::contact_card;
use crate::utils::defaults::{
    DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_SECTION, PARAMS_TYPE,
};
use crate::utils::types::{HTMLResult, RUMString, SharedAppConf, URLParams, URLPath};
use crate::{mm_get_conf, mm_get_text_item, mm_render_html};
use askama::Template;
use axum::response::Html;
use std::collections::HashMap;

#[derive(Debug)]
struct PortraitItem {
    user: RUMString,
    portrait: RUMString,
    contact: RUMString,
}

type PortraitGrid = Vec<Vec<PortraitItem>>;

#[derive(Template, Debug)]
#[template(
    source = "
        <style>

        </style>
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
                            <img src='{{ item.portrait }}' alt='{{ item.user }}' class='portrait-card-{{ css_class }}-item-portrait' fetchpriority='low' />
                            {{item.contact|safe}}
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

fn get_portrait_grid(
    section: &str,
    typ: &str,
    lang: &str,
    app_state: &SharedAppConf,
) -> PortraitGrid {
    let img_conf = mm_get_conf!(typ);
    let text_conf = mm_get_conf!(typ, lang);

    let mut grid = Vec::with_capacity(text_conf.len());
    let default_html = Html::<RUMString>(RUMString::default());
    for (r_name, r_list) in text_conf {
        let mut grid_row = Vec::with_capacity(r_list.len());
        for (i_name, i_item) in *r_list {
            grid_row.push(PortraitItem {
                user: i_name.to_string(),
                portrait: RUMString::from(mm_get_text_item!(&img_conf, i_name, "")),
                contact: match contact_card(
                    &[],
                    &HashMap::from([
                        ("section".to_string(), section.to_string()),
                        ("type".to_string(), i_name.to_string()),
                    ]),
                    app_state.clone(),
                ) {
                    Ok(v) => v.0,
                    Err(_) => default_html.0.clone(),
                },
            });
        }
        grid.push(grid_row);
    }
    grid
}

pub fn portrait_card(
    path_components: URLPath,
    params: URLParams,
    state: SharedAppConf,
) -> HTMLResult {
    let section = mm_get_text_item!(params, PARAMS_SECTION, DEFAULT_TEXT_ITEM);
    let typ = mm_get_text_item!(params, PARAMS_TYPE, DEFAULT_TEXT_ITEM);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);
    let icon_data = get_portrait_grid(section, typ, DEFAULT_NO_TEXT, &state);

    let custom_css_enabled = state.lock().expect("Lock failure").custom_css;

    mm_render_html!(PortraitCard {
        icon_data,
        css_class: RUMString::from(css_class),
        custom_css_enabled
    })
}
