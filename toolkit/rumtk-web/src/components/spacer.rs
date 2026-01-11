use crate::utils::defaults::PARAMS_SIZE;
use crate::utils::types::{HTMLResult, SharedAppConf, URLParams, URLPath};
use crate::{mm_get_text_item, mm_render_html};
use askama::Template;

#[derive(Template, Debug)]
#[template(
    source = "
        <style>

        </style>
        {% if custom_css_enabled %}
            <link href='/static/components/spacer.css' rel='stylesheet'>
        {% endif %}
        <div style='padding-bottom: {{size}}0px'></div>
    ",
    ext = "html"
)]
pub struct Spacer {
    size: usize,
    custom_css_enabled: bool,
}

pub fn spacer(path_components: URLPath, params: URLParams, state: SharedAppConf) -> HTMLResult {
    let size = mm_get_text_item!(params, PARAMS_SIZE, "0")
        .parse::<usize>()
        .unwrap_or(0);

    let custom_css_enabled = state.lock().expect("Lock failure").custom_css;

    mm_render_html!(Spacer {
        size,
        custom_css_enabled
    })
}
