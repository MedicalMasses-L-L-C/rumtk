use crate::utils::defaults::{DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_SOCIAL_LIST};
use crate::utils::types::{HTMLResult, MMString, SharedAppState, URLParams, URLPath};
use crate::{mm_get_misc_conf, mm_get_text_item, mm_render_html};
use askama::Template;

const ICON_CSS: &str = "fa-brands fa-square-{}";

#[derive(Debug, Clone)]
struct Social {
    name: MMString,
    icon: MMString,
    url: &'static str,
}

type SocialsList = Vec<Social>;

#[derive(Template, Debug, Clone)]
#[template(
    source = "
        <style>

        </style>
        {% if custom_css_enabled %}
            <link href="/static/components/socials.css" rel="stylesheet">
        {% endif %}
        <div class="socials-{{ css_class }}-container">
          {% for icon in icons %}
            <a href="{{icon.url}}" aria-label="link-{{icon.name}}" class="f20 {{icon.icon}}"> </a>
          {% endfor %}
        </div>
    ",
    ext = "html"
)]
pub struct Socials {
    icons: SocialsList,
    css_class: MMString,
}

fn get_social_list(social_list: &str) -> SocialsList {
    let data = social_list.to_lowercase();
    let sl_names = data.split(',').collect::<Vec<&str>>();
    let sl_urls = mm_get_misc_conf!(SECTION_SOCIALS);
    let mut sl: SocialsList = SocialsList::with_capacity(sl_names.len());

    for name in sl_names {
        if name.is_empty() {
            continue;
        }

        let url = mm_get_text_item!(&sl_urls, name, "");
        sl.push(
            Social {
                name: MMString::from(name),
                icon: format!("fa-brands fa-{}", name),
                url,
            }
        )
    }

    sl
}

pub fn socials(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let social_list = mm_get_text_item!(params, PARAMS_SOCIAL_LIST, DEFAULT_NO_TEXT);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let icons = get_social_list(&social_list);

    mm_render_html!(
        Socials {
            icons,
            css_class: MMString::from(css_class),
        }
    )
}
