use crate::utils::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_TYPE, SECTION_SERVICES};
use crate::utils::types::{HTMLResult, MMString, SharedAppState, TextMap, URLParams, URLPath};
use crate::{mm_get_misc_conf, mm_get_text_item, mm_render_html};
use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(
    source = "
        <style>

        </style>
        {% if custom_css_enabled %}
            <link href='/static/components/item_card.css' rel='stylesheet'>
        {% endif %}
        <div class='item-card-{{css_class}}-container'>
            {% for (service_name, service_description) in services %}
            <div>
                <details>
                    <summary class='f16 item-card-{{css_class}}-title'>
                        {{ service_name.to_uppercase() }}
                    </summary>
                    <pre class='item-card-{{css_class}}-details'>
                        {{ service_description }}
                    </pre>
                </details>
            </div>
            {% endfor %}
        </div>
    ",
    ext = "html"
)]
pub struct ItemCard {
    services: &'static TextMap,
    css_class: MMString,
}

pub fn item_card(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let typ = mm_get_text_item!(params, PARAMS_TYPE, SECTION_SERVICES);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);
    let services = mm_get_misc_conf!(typ);

    mm_render_html!(ItemCard {
        services,
        css_class: MMString::from(css_class),
    })
}
