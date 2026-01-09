use crate::mm_render_html;
use crate::utils::types::HTMLResult;
use askama::Template;

#[derive(Template)]
#[template(
    source = "
            <link rel='stylesheet' href='/static/css/bundle.min.css' onerror='this.onerror=null;this.href=\'/static/css/bundle.css\';' />
    ",
    ext = "html"
)]
pub struct CSS {}

pub fn css() -> HTMLResult {
    mm_render_html!(CSS {})
}
