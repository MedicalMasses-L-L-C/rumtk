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
use crate::components::html::{header, Header};
use crate::components::navlink::navlink;
use crate::defaults::{PARAMS_CONTENTS, PARAMS_TARGET};
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_SOURCE_URL, PARAMS_TYPE, SECTION_LINKS};
use crate::utils::types::{HTMLResult, RUMString, SharedAppState, URLParams, URLPath};
use crate::{rumtk_web_get_config, rumtk_web_get_config_string, rumtk_web_get_text_item, rumtk_web_params_map, rumtk_web_render_component, rumtk_web_render_template, RUMWebData, RUMWebTemplate};

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        {% if !disable_logo %}
        <div class='header-{{ css_class }}-navlogo'>
            <a class='undecorated no-select' href='./' style='display:flex;flex-direction:row;align-items:center;'>
                {{logo|safe}}
                <h3 class='brand-name'> {{company}}</h3>
            </a>
        </div>
        {% endif %}
        <div class='header-{{ css_class }}-navactions gap-10'>
            {% for item in nav_links %}
                {{item|safe}}
            {% endfor %}
        </div>
        <div class='header-{{ css_class }}-misc gap-10'>
        </div>
    ",
    ext = "html"
)]
pub struct Nav<'a> {
    company: RUMString,
    logo: RUMString,
    nav_links: Vec<RUMString>,
    disable_logo: bool,
    css_class: &'a str,
}

fn get_nav_links(keys: &Vec<&RUMString>, app_state: SharedAppState) -> Vec<RUMString> {
    let mut nav_links = Vec::<RUMString>::with_capacity(keys.len());
    for key in keys {
        nav_links.push(
            navlink(
                &[],
                &RUMWebData::from([(PARAMS_TARGET.to_string(), key.to_string())]),
                app_state.clone(),
            )
            .unwrap_or_default()
            .to_string(),
        );
    }

    nav_links
}

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        {{app_nav|safe}}
    ",
    ext = "html"
)]
pub struct AppNav<'a> {
    app_nav: Header<'a>,
}

pub fn app_nav(_path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let company = rumtk_web_get_config!(state).company.clone();

    let links_store = rumtk_web_get_config_string!(state, SECTION_LINKS);
    let nav_keys = links_store.keys().collect::<Vec<&RUMString>>();
    let nav_links = match rumtk_web_get_config!(state).header_conf.disable_navlinks {
        true => vec![rumtk_web_render_component!(
            "title",
            [(
                PARAMS_TYPE,
                rumtk_web_get_config!(state).title.as_str()
            )],
            state
        )?.to_string()],
        false => get_nav_links(&nav_keys, state.clone()),
    };

    let disable_logo =
        rumtk_web_get_config!(state).header_conf.disable_logo;
    let logo = match disable_logo {
        true => RUMString::default(),
        false => rumtk_web_render_component!(
            "logo",
            [
                (
                    PARAMS_SOURCE_URL,
                    rumtk_web_get_config!(state).header_conf.logo_source.clone().unwrap_or_default().as_str()
                ),
                (
                    PARAMS_CSS_CLASS,
                    rumtk_web_get_config!(state).header_conf.logo_size.as_str()
                ),
            ],
            state
        )?.to_string(),
    };

    let contents = rumtk_web_render_template!(Nav {
        company,
        logo,
        nav_links,
        disable_logo,
        css_class
    })?.to_string();

    let app_params = rumtk_web_params_map!(
        [
            (PARAMS_CONTENTS, contents),
            (PARAMS_CSS_CLASS, css_class.to_string())
        ]
    );
    let app_nav = header(
        _path_components,
        app_params.get_inner(),
        state
    )?;

    rumtk_web_render_template!(AppNav {
        app_nav
    })
}
