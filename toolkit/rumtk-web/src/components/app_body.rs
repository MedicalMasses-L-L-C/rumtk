use askama::Template;
use crate::{mm_render_html};
use crate::utils::types::{HTMLResult, MMString};

#[derive(Template)]
#[template(path = "pages/body.html")]
struct AppBody<'a> {
    elements: &'a [MMString],
}

pub fn app_body(elements: &[MMString]) -> HTMLResult {
    mm_render_html!(
        AppBody {
            elements
        }
    )
}