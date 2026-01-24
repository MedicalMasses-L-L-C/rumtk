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
pub struct FontAwesomeCSSElement {
    file: &'static str,
    version: &'static str,
    sha: &'static str,
}

#[derive(RUMWebTemplate, Debug)]
#[template(
    source = "
        {% for e in elements %}
            <link rel='stylesheet' href='https://cdnjs.cloudflare.com/ajax/libs/font-awesome/{{e.version}}/css/{{e.file}}' integrity='{{e.sha}}' crossorigin='anonymous' referrerpolicy='no-referrer' onerror='this.onerror=null;this.href=\'/static/fontawesome-free/css/{{e.file}}\';' />
        {% endfor %}
    ",
    ext = "html"
)]
pub struct FontAwesome {
    elements: Vec<FontAwesomeCSSElement>,
}

pub fn fontawesome() -> HTMLResult {
    let elements = vec![
        FontAwesomeCSSElement {
            file: "fontawesome.min.css",
            version: "7.0.1",
            sha: "sha512-M5Kq4YVQrjg5c2wsZSn27Dkfm/2ALfxmun0vUE3mPiJyK53hQBHYCVAtvMYEC7ZXmYLg8DVG4tF8gD27WmDbsg==",
        },
        FontAwesomeCSSElement {
            file: "regular.min.css",
            version: "7.0.1",
            sha: "sha512-x3gns+l9p4mIK7vYLOCUoFS2P1gavFvnO9Its8sr0AkUk46bgf9R51D8xeRUwCSk+W93YbXWi19BYzXDNBH5SA==",
        },
        FontAwesomeCSSElement {
            file: "brands.min.css",
            version: "7.0.1",
            sha: "sha512-WxpJXPm/Is1a/dzEdhdaoajpgizHQimaLGL/QqUIAjIihlQqlPQb1V9vkGs9+VzXD7rgI6O+UsSKl4u5K36Ydw==",
        },
    ];

    rumtk_web_render_html!(FontAwesome { elements })
}
