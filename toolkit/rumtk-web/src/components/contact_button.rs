use askama::Template;
use crate::{mm_render_html, mm_get_text_item, mm_render_component};
use crate::components::COMPONENTS;
use crate::utils::types::{HTMLResult, MMString, SharedAppState, URLParams, URLPath};
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_FUNCTION, DEFAULT_CONTACT_ITEM, PARAMS_TYPE, PARAMS_CSS_CLASS};

#[derive(Template, Debug)]
#[template(path = "components/contact_button.html")]
struct ContactButton {
    title: MMString,
    typ: MMString,
    send_function: MMString,
    css_class: MMString,
}

pub fn contact_button(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let typ = mm_get_text_item!(params, PARAMS_TYPE, DEFAULT_CONTACT_ITEM);
    let send_function = mm_get_text_item!(params, PARAMS_FUNCTION, DEFAULT_CONTACT_ITEM);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let title = mm_render_component!("title", [("type", typ)], state, COMPONENTS);

    mm_render_html!(
        ContactButton {
            title,
            typ: MMString::from(typ),
            send_function: MMString::from(send_function),
            css_class: MMString::from(css_class)
        }
    )
}
