use crate::mm_render_html;
use crate::utils::types::{HTMLResult, MMString};
use askama::Template;

const TEMPLATE: &str = r"
{% for element in elements %}
    {{ element|safe }}
{% endfor %}
";

#[derive(Template)]
#[template(
    source = "
    {% for element in elements %}
        {{ element|safe }}
    {% endfor %}
    ",
    ext = "html"
)]
pub struct AppBody<'a> {
    elements: &'a [MMString],
}

pub fn app_body(elements: &[MMString]) -> HTMLResult {
    mm_render_html!(AppBody { elements })
}
