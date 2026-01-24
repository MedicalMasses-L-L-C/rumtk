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
    DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_SOCIAL_LIST, SECTION_SOCIALS,
};
use crate::utils::types::{HTMLResult, RUMString, SharedAppState, URLParams, URLPath};
use crate::{rumtk_web_get_conf, rumtk_web_get_text_item, rumtk_web_render_html, RUMWebTemplate};
use askama::Template;
use rumtk_core::strings::{rumtk_format, RUMStringConversions};

#[derive(Debug, Clone)]
struct Social {
    name: RUMString,
    icon: RUMString,
    url: RUMString,
}

type SocialsList = Vec<Social>;

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        <style>
            .socials-default-container {
                display: flex;
                color: var(--color-bg-white);
                padding: 1em;
                filter: contrast(10%);
            }
        </style>
        {% if custom_css_enabled %}
            <link href='/static/components/socials.css' rel='stylesheet'>
        {% endif %}
        <div class='socials-{{ css_class }}-container gap-10'>
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

fn get_social_list(social_list: &str, state: &SharedAppState) -> SocialsList {
    let data = social_list.to_lowercase();
    let sl_names = data.split(',').collect::<Vec<&str>>();
    let sl_urls = rumtk_web_get_conf!(state, SECTION_SOCIALS);
    let mut sl: SocialsList = SocialsList::with_capacity(sl_names.len());

    for name in sl_names {
        if name.is_empty() {
            continue;
        }

        let url = rumtk_web_get_text_item!(&sl_urls, name, "").to_rumstring();
        sl.push(Social {
            name: RUMString::from(name),
            icon: rumtk_format!("fa-brands fa-{}", name),
            url,
        })
    }

    sl
}

pub fn socials(_path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let social_list = rumtk_web_get_text_item!(params, PARAMS_SOCIAL_LIST, DEFAULT_NO_TEXT);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let custom_css_enabled = state.read().expect("Lock failure").config.custom_css;

    let icons = get_social_list(&social_list, &state);

    rumtk_web_render_html!(Socials {
        icons,
        css_class: RUMString::from(css_class),
        custom_css_enabled
    })
}
