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
