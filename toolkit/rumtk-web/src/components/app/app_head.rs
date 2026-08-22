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
use crate::components::app::css::*;
use crate::components::app::fontawesome::*;
use crate::components::app::htmx::*;
use crate::components::app::meta::*;
use crate::utils::types::{SharedAppState, URLParams, URLPath};
use crate::{ComponentResult, RUMWebTemplate, RUMWebTemplateSafe};

#[derive(RUMWebTemplate)]
#[template(
    source = "
        <head>
            {{meta}}
            {{css}}
            {{fontawesome}}
            {{htmx}}
        </head>
    ",
    ext = "html"
)]
pub struct AppShellHead {
    meta: Meta,
    css: CSS,
    fontawesome: FontAwesome,
    htmx: HTMX,
}

impl RUMWebTemplateSafe for AppShellHead {}

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
pub fn app_head<'a>(
    _path_components: URLPath<'a, 'a>,
    _params: URLParams<'a>,
    state: SharedAppState,
) -> ComponentResult<AppShellHead> {
    Ok(AppShellHead {
        meta: meta(state.clone())?,
        css: css()?,
        fontawesome: fontawesome(state.clone())?,
        htmx: htmx()?
    })
}
