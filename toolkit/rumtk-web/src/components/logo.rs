use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_TYPE};
use crate::utils::types::{HTMLResult, MMString, SharedAppState, URLParams, URLPath};
use crate::{mm_get_param_eq, mm_get_text_item, mm_render_html};
use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(
    source = "
        <style>

        </style>
        {% if custom_css_enabled %}
            <link href='/static/components/logo.css' rel='stylesheet'>
        {% endif %}
        <div class='centered logo'>
        {% if diamond %}
            <img src='/static/img/logo.webp' alt='Webp Logo' class='logo-{{ css_class }}' fetchpriority='high' />
        {% else %}
            <img src='/static/img/logo.svg' alt='SVG Logo' fetchpriority='high'/>
        {% endif %}
        </div>
    ",
    ext = "html"
)]
pub struct Logo {
    diamond: bool,
    css_class: MMString,
}

const DEFAULT_TYPE: &str = "diamond";

pub fn logo(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let diamond = mm_get_param_eq!(params, PARAMS_TYPE, DEFAULT_TYPE, false);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    mm_render_html!(Logo {
        diamond,
        css_class: MMString::from(css_class)
    })
}
