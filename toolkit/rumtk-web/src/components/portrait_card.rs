use crate::components::contact_card::contact_card;
use crate::utils::defaults::{DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_SECTION, PARAMS_TYPE};
use crate::utils::types::{AppState, HTMLResult, MMString, SharedAppState, URLParams, URLPath};
use crate::{mm_get_conf, mm_get_param, mm_get_text_item, mm_render_html};
use askama::Template;
use axum::response::Html;
use std::collections::HashMap;

#[derive(Debug)]
struct PortraitItem {
    user: MMString,
    portrait: MMString,
    contact: MMString,
}

type PortraitGrid = Vec<Vec<PortraitItem>>;

#[derive(Template, Debug)]
#[template(
    source = "
        <style>

        </style>
        {% if custom_css_enabled %}
            <link href="/static/components/portrait_card.css" rel="stylesheet">
        {% endif %}
        <div class="centered twothird-width portrait-card-{{ css_class }}-container">
            <table>
                <thead></thead>
                <tbody>
                {% for row in icon_data %}
                    <tr class="portrait-card-{{ css_class }}-row">
                        {% for item in row %}
                        <td class="portrait-card-{{ css_class }}-item">
                            <img src="{{ item.portrait }}" alt="{{ item.user }}" class="portrait-card-{{ css_class }}-item-portrait" fetchpriority="low" />
                            {{item.contact|safe}}
                        </td>
                        {% endfor %}
                    </tr>
                {% endfor %}
                </tbody>
            </table>
        </div>
    ",
    ext = "html"
)]
pub struct PortraitCard {
    icon_data: PortraitGrid,
    css_class: MMString,
}

fn get_portrait_grid(section: &str, typ: &str, lang: &str, app_state: SharedAppState) -> PortraitGrid {
    let img_conf = mm_get_conf!(typ);
    let text_conf = mm_get_conf!(typ, lang);

    let mut grid = Vec::with_capacity(text_conf.len());
    let default_html = Html::<MMString>(MMString::default());
    for (r_name, r_list) in text_conf {
        let mut grid_row = Vec::with_capacity(r_list.len());
        for (i_name, i_item) in *r_list {
            grid_row.push(
                PortraitItem {
                    user: i_name.to_string(),
                    portrait: MMString::from(mm_get_text_item!(&img_conf, i_name, "")),
                    contact: match contact_card(
                        &[],
                        &HashMap::from([
                            ("section".to_string(), section.to_string()),
                            ("type".to_string(), i_name.to_string())
                        ]),
                        app_state.clone(),
                    ) {
                        Ok(v) => v.0,
                        Err(_) => default_html.0.clone(),
                    },
                }
            );
        }
        grid.push(grid_row);
    }
    grid
}

pub fn portrait_card(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let section = mm_get_text_item!(params, PARAMS_SECTION, DEFAULT_TEXT_ITEM);
    let typ = mm_get_text_item!(params, PARAMS_TYPE, DEFAULT_TEXT_ITEM);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);
    let icon_data = get_portrait_grid(section, typ, DEFAULT_NO_TEXT, state);

    mm_render_html!(
        PortraitCard {
            icon_data,
            css_class: MMString::from(css_class)
        }
    )
}
