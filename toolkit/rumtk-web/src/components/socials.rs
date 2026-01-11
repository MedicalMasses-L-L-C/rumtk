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
    DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_SOCIAL_LIST,
};
use crate::utils::types::{HTMLResult, RUMString, SharedAppConf, URLParams, URLPath};
use crate::{mm_get_misc_conf, mm_get_text_item, mm_render_html};
use askama::Template;

const ICON_CSS: &str = "fa-brands fa-square-{}";

#[derive(Debug, Clone)]
struct Social {
    name: RUMString,
    icon: RUMString,
    url: &'static str,
}

type SocialsList = Vec<Social>;

#[derive(Template, Debug, Clone)]
#[template(
    source = "
        <style>

        </style>
        {% if custom_css_enabled %}
            <link href='/static/components/socials.css' rel='stylesheet'>
        {% endif %}
        <div class='socials-{{ css_class }}-container'>
          {% for icon in icons %}
            <a href='{{icon.url}}' aria-label='link-{{icon.name}}' class='f20 {{icon.icon}}'> </a>
          {% endfor %}
        </div>
    ",
    ext = "html"
)]
pub struct Socials {
    icons: SocialsList,
    css_class: RUMString,
    custom_css_enabled: bool,
}

fn get_social_list(social_list: &str) -> SocialsList {
    let data = social_list.to_lowercase();
    let sl_names = data.split(',').collect::<Vec<&str>>();
    let sl_urls = mm_get_misc_conf!(SECTION_SOCIALS);
    let mut sl: SocialsList = SocialsList::with_capacity(sl_names.len());

    for name in sl_names {
        if name.is_empty() {
            continue;
        }

        let url = mm_get_text_item!(&sl_urls, name, "");
        sl.push(Social {
            name: RUMString::from(name),
            icon: format!("fa-brands fa-{}", name),
            url,
        })
    }

    sl
}

pub fn socials(path_components: URLPath, params: URLParams, state: SharedAppConf) -> HTMLResult {
    let social_list = mm_get_text_item!(params, PARAMS_SOCIAL_LIST, DEFAULT_NO_TEXT);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let custom_css_enabled = state.lock().expect("Lock failure").custom_css;

    let icons = get_social_list(&social_list);

    mm_render_html!(Socials {
        icons,
        css_class: RUMString::from(css_class),
        custom_css_enabled
    })
}
