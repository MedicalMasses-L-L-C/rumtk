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
use crate::static_components::{css::css, fontawesome::fontawesome, htmx::htmx, meta::meta};
use crate::utils::types::{HTMLResult, RUMString, SharedAppConf, URLParams, URLPath};
use crate::{rumtk_web_render_component, rumtk_web_render_html};
use askama::Template;

const DEFAULT_PAGE_NAME: &str = "index";

#[derive(Template)]
#[template(
    source = "
        <head>
            {{meta|safe}}
            {{css|safe}}
            {{fontawesome|safe}}
            {{htmx|safe}}
        </head>
    ",
    ext = "html"
)]
pub struct AppShellHead {
    meta: RUMString,
    css: RUMString,
    fontawesome: RUMString,
    htmx: RUMString,
}

///
///     !!!!!!!!!!!!!!!!!!!!!!!WARNING!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
///
///      The snippet below will add key static imports relying on CDN free bandwidth where possible.
///      Keep in mind this can be dangerous security wise if the CDN or DNS services are manipulated
///      because we will fallback on a local file version that may be of an older version.
///
///      It is not ideal but it will allow continuance of service for websites during CDN outages
///      which do happen.
///
pub fn app_head(path_components: URLPath, params: URLParams, state: SharedAppConf) -> HTMLResult {
    //Let's render the head component
    let html_meta = rumtk_web_render_component!(meta, state);

    //Let's render the head component
    let html_css = rumtk_web_render_component!(css);

    //Let's render the head component
    let html_fontawesome = rumtk_web_render_component!(fontawesome);

    //Let's render the head component
    let html_htmx = rumtk_web_render_component!(htmx);

    rumtk_web_render_html!(AppShellHead {
        meta: html_meta,
        css: html_css,
        fontawesome: html_fontawesome,
        htmx: html_htmx
    })
}
