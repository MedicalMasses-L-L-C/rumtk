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
use crate::utils::ConstTextMap;
use phf_macros::phf_ordered_map;
/*
   TextMap
*/
pub static DEFAULT_TEXT_MAP: ConstTextMap = phf_ordered_map!();

/*
   IP
*/
pub const DEFAULT_LOCAL_LISTENING_ADDRESS: &str = "127.0.0.1:3000";
pub const DEFAULT_OUTBOUND_LISTENING_ADDRESS: &str = "0.0.0.0:3000";

/*
   Misc
*/
pub const DEFAULT_TEXT_ITEM: &str = "default";
pub const DEFAULT_CONTACT_ITEM: &str = "company";
pub const DEFAULT_NO_TEXT: &str = "";

/*
    Options
*/
pub const OPT_INVERTED_DIRECTION: &str = "inverted";

/*
   Params
*/
pub const PARAMS_TITLE: &str = "title";
pub const PARAMS_TYPE: &str = "type";
pub const PARAMS_CSS_CLASS: &str = "class";
pub const PARAMS_SOCIAL_LIST: &str = "social_list";
pub const PARAMS_ITEM: &str = "item";
pub const PARAMS_INVERTED: &str = "inverted";
pub const PARAMS_SECTION: &str = "section";
pub const PARAMS_FUNCTION: &str = "function";
pub const PARAMS_TARGET: &str = "target";
pub const PARAMS_SIZE: &str = "size";
pub const PARAMS_CONTENTS: &str = "contents";
pub const PARAMS_MODULE: &str = "module";
pub const PARAMS_ENDPOINT: &str = "endpoint";

/*
   CONF SECTIONS
*/
pub const SECTION_TEXT: &str = "text";
pub const SECTION_PERSONNEL: &str = "personnel";
pub const SECTION_CONTACT: &str = "contact";
pub const SECTION_TITLES: &str = "titles";
pub const SECTION_API: &str = "api";
pub const SECTION_SOCIALS: &str = "socials";
pub const SECTION_SERVICES: &str = "services";
pub const SECTION_PRODUCTS: &str = "products";
pub const SECTION_LINKS: &str = "links";
pub const SECTION_MODULES: &str = "modules";
pub const SECTION_ENDPOINTS: &str = "endpoints";
pub const SECTION_DEFAULT: &str = "default";

/*
   Content Types
*/
pub const CONTENT_TYPE_PDF: &str = "application/pdf";
pub const CONTENT_TYPE_HTML: &str = "text/html";

/*
   Form Data Types
*/
pub const FORM_DATA_TYPE_PDF: &str = "pdf";
pub const FORM_DATA_TYPE_HTML: &str = "html";
pub const FORM_DATA_TYPE_DEFAULT: &str = "text";

/*
   LANGUAGES
*/
pub const LANG_EN: &str = "en";
pub const LANG_ES: &str = "es";

/*
   Icon
*/
pub const DEFAULT_ICON_STYLE: &str = "fa-solid";

/*
   Robots.txt
*/
pub const DEFAULT_ROBOT_TXT: &str = r"
User-agent: *
Disallow: /static/
";
