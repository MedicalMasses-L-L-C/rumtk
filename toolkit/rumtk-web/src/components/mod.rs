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
use crate::utils::types::ComponentMap;
use phf_macros::phf_map;

//AppShell
pub mod app_body;
pub mod app_head;
pub mod app_shell;

// Components
mod Form;
mod contact_button;
mod contact_card;
pub mod div;
mod footer;
mod info_card;
mod item_card;
mod logo;
mod navbar;
mod navlink;
mod portrait_card;
mod socials;
mod spacer;
mod title;

pub static COMPONENTS: ComponentMap = phf_map! {
    "logo" => logo::logo,
    "info_card" => info_card::info_card,
    "portrait_card" => portrait_card::portrait_card,
    "title" => title::title,
    "footer" => footer::footer,
    "navbar" => navbar::navbar,
    "contact_card" => contact_card::contact_card,
    "contact_button" => contact_button::contact_button,
    "socials" => socials::socials,
    "item_card" => item_card::item_card,
    "navlink" => navlink::navlink,
    "label" => Form::label::label,
    "text_card" => Form::text_card::text_card,
    "form" => Form::form::form,
    "spacer" => spacer::spacer,
    "div" => div::div,
};
