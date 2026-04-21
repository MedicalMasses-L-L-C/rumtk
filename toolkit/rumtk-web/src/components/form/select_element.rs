/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2026  Luis M. Santos, M.D. <lsantos@medicalmasses.com>
 * Copyright (C) 2026  MedicalMasses L.L.C. <contact@medicalmasses.com>
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
use crate::components::form::props::InputProps;
use crate::utils::types::HTMLResult;
use crate::{rumtk_web_render_html, RUMWebTemplate};

type SelectOptions<'a> = Vec<(&'a str, &'a str)>;

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        <select size='{{ count }}' {{props|safe}}>
          {% for (item_value, item_text) in items %}
            <option value='{{item_value}}'>{{item_text}}</option>
          {% endfor %}
        </select>
    ",
    ext = "html"
)]
pub struct SelectElement<'a> {
    items: SelectOptions<'a>,
    props: &'a str,
    count: usize,
    css_class: &'a str
}

fn parse_select_data(data: &str) -> Vec<(&str, &str)> {
    let rows: Vec<&str> = data.split(",").collect();
    let mut result: SelectOptions = vec![];
    
    for item in rows {
        let pair: Vec<&str> = item.split("=").collect();
        
        let r = match pair.len() {
            0 => {
                return result;
            },
            1 => {
                let val = pair[0];
                (val, val)
            },
            _ => {
                let val = pair[0];
                let text = pair[1];
                
                (val, text)
            }
        };
        
        result.push(r);
    }
    
    result
}

pub fn select_element(_element: &str, data: &str, props: InputProps, css_class: &str) -> HTMLResult {
    let items = parse_select_data(data);
    let count = items.len();
    
    rumtk_web_render_html!(SelectElement {
        items,
        props: &props.to_rumstring().replace("\\\\", "\\"),
        count,
        css_class,
    })
}
