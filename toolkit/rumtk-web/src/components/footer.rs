use askama::Template;
use crate::{mm_get_text_item, mm_render_component, mm_render_html};
use crate::components::COMPONENTS;
use crate::utils::types::{HTMLResult, MMString, SharedAppState, URLParams, URLPath};
use crate::utils::defaults::{DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_SOCIAL_LIST};

#[derive(Debug, Clone)]
struct FooterItem {
    typ: MMString,
    icon_url: MMString,
    text: MMString,
}

#[derive(Debug, Clone)]
struct FooterSection {
    typ: MMString,
    items: Vec<FooterItem>,
}

#[derive(Template, Debug, Clone)]
#[template(path = "components/footer.html")]
struct Footer {
    button: MMString,
    socials: MMString,
    css_class: MMString,
}

pub fn footer(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let social_list = mm_get_text_item!(params, PARAMS_SOCIAL_LIST, DEFAULT_NO_TEXT);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);
    
    let contact_button = mm_render_component!("contact_button", [("type", "contact"), ("function", "goto_contact"), ("class", "centered")], state, COMPONENTS);
    let socials = mm_render_component!("socials", [("social_list", social_list)], state, COMPONENTS);

    mm_render_html!(
        Footer {
            button: contact_button,
            socials,
            css_class: MMString::from(css_class),
        }
    )
}
