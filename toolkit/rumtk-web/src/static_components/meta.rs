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
use crate::utils::types::{HTMLResult, RUMString, SharedAppState};
use crate::{rumtk_web_render_html, RUMWebTemplate};
use askama::Template;

#[derive(RUMWebTemplate)]
#[template(
    source = "
            <meta charset='UTF-8'>
            <meta http-equiv='Content-Type' content='text/html; charset=utf-8' />
            <meta name='viewport' content='width=device-width, initial-scale=1.0' />
            <meta http-equiv='X-UA-Compatible' content='IE=edge,chrome=1'/>
            <meta name='description' content='{{description}}'>
            <title>{{title}}</title>
            <link rel='icon' type='image/png' href='/static/img/icon.png'>
    ",
    ext = "html"
)]
pub struct Meta {
    title: RUMString,
    description: RUMString,
}

pub fn meta(state: SharedAppState) -> HTMLResult {
    let owned_state = state.read().expect("Lock failure");

    rumtk_web_render_html!(Meta {
        title: owned_state.get_config().title.clone(),
        description: owned_state.get_config().description.clone()
    })
}
