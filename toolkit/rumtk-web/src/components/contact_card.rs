use crate::utils::defaults::{
    DEFAULT_CONTACT_ITEM, DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_SECTION,
    PARAMS_TYPE,
};
use crate::utils::types::{
    HTMLResult, MMString, NestedNestedTextMap, NestedTextMap, SharedAppState, TextMap, URLParams,
    URLPath,
};
use crate::{mm_get_conf, mm_get_text_item, mm_render_html};
use askama::Template;
use phf_macros::phf_ordered_map;

#[derive(Template, Debug)]
#[template(
    source = "
        <style>
            .contact-card-default-container {
            }

            .contact-card-default-container > p {
                margin: 0;
            }

            .contact-card-centered-container {
                max-width: fit-content;
                margin-inline: auto;
            }

            .contact-card-centered-container > p {
                margin: 0;
            }
        </style>
        {% if custom_css_enabled %}
            <link href='/static/components/contact_card.css' rel='stylesheet'>
        {% endif %}
        <div class='f14 centered'>
            <div class='f18 contact-card-{{ css_class }}-container'>
                {% for (details_typ, details_data) in contact_lines %}
                    {% if details_typ == &\"phrase\" && !details_data.is_empty() %}
                    <p class='italics f18' >
                        '{{ details_data }}'
                    </p>
                    {% else if details_typ == &\"email\" && !details_data.is_empty() %}
                    <p>
                        <a  class=' f14 no-text-color' href='mailto:{{ details_data }}'>{{ details_data }}</a>
                    </p>
                    {% else if details_typ == &\"phone\" && !details_data.is_empty() %}
                    <p>
                        <a  class='f14 no-text-color' href='tel:{{ details_data }}'>{{ details_data }}</a>
                    </p>
                    {% else if !details_data.is_empty() %}
                    <p class='f14' >
                        {{ details_data }}
                    </p>
                    {% endif %}
                {% endfor %}
            </div>
        </div>
    ",
    ext = "html"
)]
pub struct ContactCard {
    contact_lines: &'static TextMap,
    css_class: MMString,
}

pub fn contact_card(
    path_components: URLPath,
    params: URLParams,
    state: SharedAppState,
) -> HTMLResult {
    let section = mm_get_text_item!(params, PARAMS_SECTION, DEFAULT_CONTACT_ITEM);
    let typ = mm_get_text_item!(params, PARAMS_TYPE, DEFAULT_CONTACT_ITEM);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let text_conf: &NestedNestedTextMap = mm_get_conf!(SECTION_CONTACT, DEFAULT_NO_TEXT);
    let contact_item: &&NestedTextMap =
        mm_get_text_item!(&text_conf, &section, &&phf_ordered_map!());
    let contact_lines: &TextMap = mm_get_text_item!(&contact_item, &typ, &phf_ordered_map!());

    mm_render_html!(ContactCard {
        contact_lines,
        css_class: MMString::from(css_class),
    })
}
