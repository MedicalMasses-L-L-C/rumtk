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
use crate::defaults::DEFAULT_NO_TEXT;
use crate::{
    rumtk_web_collect_page, rumtk_web_get_param, rumtk_web_render_component,
    rumtk_web_render_contents, rumtk_web_render_html, HTMLResult, RUMWebTemplate, SharedAppState,
    URLParams, URLPath,
};
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
        rumtk_web_get_param!(path_components, 0, RUMString::from(DEFAULT_NO_TEXT));

    //Let's render the main tag contents
    let body_components = rumtk_web_collect_page!(page, state);
    let contents = rumtk_web_render_component!(|| -> HTMLResult {
        rumtk_web_render_contents(&body_components)
    });

    rumtk_web_render_html!(Main { contents })
}
