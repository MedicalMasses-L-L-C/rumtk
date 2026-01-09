use crate::utils::defaults::{DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, OPT_INVERTED_DIRECTION, PARAMS_CSS_CLASS, PARAMS_INVERTED, PARAMS_ITEM};
use crate::utils::types::{HTMLResult, MMString, SharedAppState, URLParams, URLPath};
use crate::{mm_get_conf, mm_get_param_eq, mm_get_text_item, mm_render_html};
use askama::Template;
use phf_macros::phf_ordered_map;

#[derive(Template, Debug, Clone)]
#[template(
    source = "
        <style>

        </style>
        {% if custom_css_enabled %}
            <link href="/static/components/info_card.css" rel="stylesheet">
        {% endif %}
        <div class="info-card-{{ css_class }}-container">
            {% if inverted %}
                <pre class="info-card-{{ css_class }}-descbox">
                    {{ description }}
                </pre>
                <div class="f18 info-card-{{ css_class }}-titlebox">
                    {{ title }}
                </div>
            {% else %}
                <div class="f18 info-card-{{ css_class }}-titlebox">
                    {{ title }}
                </div>
                <pre class="info-card-{{ css_class }}-descbox">
                    {{ description }}
                </pre>
            {% endif %}
        </div>
    ",
    ext = "html"
)]
pub struct InfoCard {
    title: &'static str,
    description: &'static str,
    inverted: bool,
    css_class: MMString,
}

pub fn info_card(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let card_text_item = mm_get_text_item!(params, PARAMS_ITEM, DEFAULT_TEXT_ITEM);
    let inverted = mm_get_param_eq!(params, PARAMS_INVERTED, OPT_INVERTED_DIRECTION, false);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let text_store = mm_get_conf!(SECTION_TEXT, DEFAULT_NO_TEXT);
    let en_text = mm_get_text_item!(&text_store, "0", &&phf_ordered_map!());
    let itm = mm_get_text_item!(&en_text, &card_text_item, &&phf_ordered_map!());
    let title = mm_get_text_item!(&itm, "title", DEFAULT_NO_TEXT);
    let desc = mm_get_text_item!(&itm, "description", DEFAULT_NO_TEXT);

    mm_render_html!(
        InfoCard {
            title,
            description: desc,
            inverted,
            css_class: MMString::from(css_class),
        }
    )
}
