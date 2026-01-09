use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_ITEM, PARAMS_TYPE};
use crate::utils::types::{HTMLResult, MMString, SharedAppState, URLParams, URLPath};
use crate::{mm_get_text_item, mm_render_html};
use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(
    source = "
        <style>
            .card-default {
                max-width: 1700px;
                padding: 20px;
                background-color: var(--color-indigo);

                border-radius: 15px;
            }
        </style>
        <link href="/static/components/form/text_card.css" rel="stylesheet">
        <div class="centered card-{{css_class}}">
          <div hx-get="/component/label?type={{typ}}" hx-target="this" hx-trigger="load"> </div>
        </div>
    ",
    ext = "html"
)]
struct TextCard {
    typ: MMString,
    css_class: MMString,
}

pub fn text_card(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let typ = mm_get_text_item!(params, PARAMS_TYPE, DEFAULT_TEXT_ITEM);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    mm_render_html!(
        TextCard {
            typ: MMString::from(typ),
            css_class: MMString::from(css_class),
        }
    )
}
