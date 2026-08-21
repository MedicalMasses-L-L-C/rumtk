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
use crate::components::html::{footer, header, Footer, Header};
use crate::components::main::{main, Main};
use crate::utils::defaults::DEFAULT_EMPTY_PARAMS;
use crate::utils::types::{SharedAppState, URLParams, URLPath};
use crate::{rumtk_web_get_config, ComponentResult, RUMWebTemplate};
use rumtk_core::strings::AsStr;

#[derive(RUMWebTemplate)]
#[template(
    source = "
        <body class='f12 fw300 theme-{{theme}}'>
            <a href='#main-content header' hidden>Skip to main content</a>
            {{header|safe}}
            {{main|safe}}
            {{footer|safe}}
        </body>
    ",
    ext = "html"
)]
pub struct AppBody<'a> {
    theme: &'a str,
    header: Header<'a>,
    main: Main,
    footer: Footer<'a>,
}

pub fn app_body(path_components: URLPath, params: URLParams, state: SharedAppState) -> ComponentResult<AppBody> {
    let theme = rumtk_web_get_config!(state).theme.clone();

    //Let's render the header and footer
    //<div class="" hx-get="/component/navbar" hx-target="#navbar" hx-trigger="load" id="navbar"></div>
    let header = header(path_components, DEFAULT_EMPTY_PARAMS, state)?;
    let main = main(path_components, DEFAULT_EMPTY_PARAMS, state)?;
    //<div class="" hx-get="/component/footer?social_list=linkedin,github" hx-target="#footer" hx-trigger="load" id="footer"></div>
    let footer = footer(
        path_components,
        rumtk[(
            "social_list",
            rumtk_web_get_config!(state).footer_conf.socials_list.as_str()
        )],
        state
    )?.to_string();

    Ok(AppBody {
        theme: theme.as_str(),
        header,
        main,
        footer
    })
}
