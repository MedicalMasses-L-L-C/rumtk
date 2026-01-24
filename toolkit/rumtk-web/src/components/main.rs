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
use crate::defaults::DEFAULT_TEXT_ITEM;
use crate::{
    rumtk_web_collect_page, rumtk_web_get_param, rumtk_web_render_component,
    rumtk_web_render_contents, rumtk_web_render_html, HTMLResult, RUMWebTemplate, SharedAppState,
    URLParams, URLPath,
};
use askama::Template;
use rumtk_core::strings::RUMString;

#[derive(RUMWebTemplate)]
#[template(
    source = "
        <main class='' id='main-content'>
             <div class='padding-bottom-200'>

             </div>
             {{contents|safe}}
             <div class='padding-bottom-50'>

           </div>
        </main>
    ",
    ext = "html"
)]
pub struct Main {
    contents: RUMString,
}

pub fn main(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let page: RUMString =
        rumtk_web_get_param!(path_components, 0, RUMString::from(DEFAULT_TEXT_ITEM));

    //Let's render the main tag contents
    let body_components = rumtk_web_collect_page!(page, state);
    let contents = rumtk_web_render_component!(|| -> HTMLResult {
        rumtk_web_render_contents(&body_components)
    });

    rumtk_web_render_html!(Main { contents })
}
