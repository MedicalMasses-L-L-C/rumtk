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
use crate::utils::types::HTMLResult;
use crate::{rumtk_web_render_html, RUMWebTemplate};
use askama::Template;

#[derive(Debug)]
pub struct HTMXElement {
    version: &'static str,
    sha: &'static str,
}

#[derive(RUMWebTemplate, Debug)]
#[template(
    source = "
        <script src='https://cdn.jsdelivr.net/npm/htmx.org@{{lib.version}}/dist/htmx.min.js' integrity='{{lib.sha}}' crossorigin='anonymous'></script>
    ",
    ext = "html"
)]
pub struct HTMX {
    lib: HTMXElement,
}

pub fn htmx() -> HTMLResult {
    rumtk_web_render_html!(HTMX {
        lib: HTMXElement {
            version: "2.0.8",
            sha: "sha384-/TgkGk7p307TH7EXJDuUlgG3Ce1UVolAOFopFekQkkXihi5u/6OCvVKyz1W+idaz"
        }
    })
}
