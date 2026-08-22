/*
 *     rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 *     This toolkit aims to be reliable, simple, performant, and standards compliant.
 *     Copyright (C) 2026  Luis M. Santos, M.D. <lsantos@medicalmasses.com>
 *     Copyright (C) 2026  MedicalMasses L.L.C. <contact@medicalmasses.com>
 *
 *     This program is free software: you can redistribute it and/or modify
 *     it under the terms of the GNU General Public License as published by
 *     the Free Software Foundation, either version 3 of the License, or
 *     (at your option) any later version.
 *
 *     This program is distributed in the hope that it will be useful,
 *     but WITHOUT ANY WARRANTY; without even the implied warranty of
 *     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *     GNU General Public License for more details.
 *
 *     You should have received a copy of the GNU General Public License
 *     along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

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
use crate::components::app::logo::{logo, Logo};
use crate::components::app::navlink::{navlink, NavLink};
use crate::components::html::{header, Header};
use crate::components::title::{title, Title};
use crate::defaults::{PARAMS_TARGET, PARAMS_TITLE};
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_SOURCE_URL, PARAMS_TYPE};
use crate::utils::types::{RUMString, SharedAppState, URLParams, URLPath};
use crate::{rumtk_web_get_config, rumtk_web_get_text_item, rumtk_web_params_map, ComponentResult, PageConf, RUMWebData, RUMWebTemplate, RUMWebTemplateSafe};
use rumtk_core::base::RUMResult;

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
        {% if !disable_links %}
        <div class='header-{{ css_class }}-navactions gap-10'>
            {% for item in nav_links %}
                {{item|safe}}
            {% endfor %}
        </div>
        {% else %}
        {{title|safe}}
        {% endif %}
        <div class='header-{{ css_class }}-misc gap-10'>
        </div>
    ",
    ext = "html"
)]
pub struct Nav {
    company: RUMString,
    logo: Logo,
    title: Title,
    nav_links: Vec<NavLink>,
    disable_links: bool,
    disable_logo: bool,
    css_class: RUMString,
}

impl RUMWebTemplateSafe for Nav {}

fn get_nav_links(itms: &Vec<(RUMString, PageConf)>, app_state: SharedAppState) -> RUMResult<Vec<NavLink>> {
    let mut nav_links = Vec::<NavLink>::with_capacity(itms.len());
    for (k, itm) in itms {
        nav_links.push(
            navlink(
                &[],
                &RUMWebData::from([
                    (PARAMS_TITLE.to_string(), k.to_string()),
                    (PARAMS_TARGET.to_string(), itm.url.to_string()),
                ]),
                app_state.clone(),
            )?,
        );
    }

    Ok(nav_links)
}

#[derive(RUMWebTemplate, Debug, Clone)]
#[template(
    source = "
        {{app_nav}}
    ",
    ext = "html"
)]
pub struct AppNav {
    app_nav: Header<Nav>,
}

impl RUMWebTemplateSafe for AppNav {}

pub fn app_nav<'a>(_path_components: URLPath<'a, 'a>, params: URLParams<'a>, state: SharedAppState) -> ComponentResult<AppNav> {
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM).to_string();

    let company = rumtk_web_get_config!(state).company.clone();

    let title_params = rumtk_web_params_map!([(
                PARAMS_TYPE,
                rumtk_web_get_config!(state).title.as_str()
            )]);
    let title = title(
            _path_components,
            title_params.get_inner(),
            state.clone()
        )?;

    let links = match &rumtk_web_get_config!(state).router.pages {
        Some(pages) => {
            let mut links = Vec::<(RUMString, PageConf)>::with_capacity(pages.len());
            for (k, v) in pages.iter() {
                links.push((k.clone(), v.clone()));
            }
            links
        },
        None => vec![],
    };
    let disable_links = rumtk_web_get_config!(state).header_conf.disable_navlinks;
    let nav_links = get_nav_links(&links, state.clone())?;

    let disable_logo =
        rumtk_web_get_config!(state).header_conf.disable_logo;
    let logo_params = rumtk_web_params_map!([
                    (
                        PARAMS_SOURCE_URL,
                        rumtk_web_get_config!(state).header_conf.logo_source.clone().unwrap_or_default().as_str()
                    ),
                    (
                        PARAMS_CSS_CLASS,
                        rumtk_web_get_config!(state).header_conf.logo_size.as_str()
                    ),
                ]
    );
    let logo = logo(_path_components, logo_params.get_inner(), state.clone())?;

    let contents = Nav {
        company,
        logo,
        title,
        nav_links,
        disable_logo,
        disable_links,
        css_class
    };

    let app_nav = header(
        contents,
        params,
        state
    )?;

    Ok(AppNav {
        app_nav
    })
}
