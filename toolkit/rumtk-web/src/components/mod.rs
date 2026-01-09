use phf_macros::phf_map;
use crate::utils::types::ComponentMap;

//AppShell
pub mod app_shell;
pub mod app_body;

// Components
mod logo;
mod info_card;
mod portrait_card;
mod title;
mod footer;
mod navbar;
mod contact_card;
mod igeki_api;
mod contact_button;
mod socials;
mod item_card;
mod Form;
mod navlink;
mod spacer;
pub mod div;

pub static COMPONENTS: ComponentMap = phf_map! {
    "logo" => logo::logo,
    "info_card" => info_card::info_card,
    "portrait_card" => portrait_card::portrait_card,
    "title" => title::title,
    "footer" => footer::footer,
    "navbar" => navbar::navbar,
    "contact_card" => contact_card::contact_card,
    "igeki_api" => igeki_api::igeki_api,
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