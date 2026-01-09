use crate::utils::defaults::{DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_ITEM, PARAMS_TYPE};
use crate::utils::types::{HTMLResult, MMString, SharedAppState, URLParams, URLPath};
use crate::{mm_get_conf, mm_get_text_item, mm_render_html, mm_render_markdown};
use askama::Template;
use phf_macros::phf_ordered_map;

#[derive(Template, Debug, Clone)]
#[template(
    source = "
        <style>
            .label-default {
                text-wrap: wrap;
                margin: auto;
            }
        </style>
        {% if custom_css_enabled %}
            <link href="/static/components/form/label.css" rel="stylesheet">
        {% endif %}
        <pre class="label-{{css_class}}">
            {{text|safe}}
        </pre>
    ",
    ext = "html"
)]
struct Label {
    text: MMString,
    css_class: MMString,
}

pub fn label(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let typ = mm_get_text_item!(params, PARAMS_TYPE, DEFAULT_TEXT_ITEM);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let text_store = mm_get_conf!(SECTION_TEXT, DEFAULT_NO_TEXT);
    let en_text = mm_get_text_item!(&text_store, "0", &&phf_ordered_map!());
    let itm = mm_get_text_item!(&en_text, &typ, &&phf_ordered_map!());
    let desc = mm_get_text_item!(&itm, "description", DEFAULT_NO_TEXT);
    let html = mm_render_markdown!(desc);

    mm_render_html!(
        Label {
            text: html,
            css_class: MMString::from(css_class),
        }
    )
}
