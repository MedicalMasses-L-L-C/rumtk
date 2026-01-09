use askama::Template;
use crate::{mm_render_html, mm_get_text_item};
use crate::utils::types::{HTMLResult, SharedAppState, URLParams, URLPath};
use crate::utils::defaults::{PARAMS_SIZE};

#[derive(Template, Debug)]
#[template(path = "components/spacer.html")]
struct Spacer {
    size: usize,
}

pub fn spacer(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let size = mm_get_text_item!(params, PARAMS_SIZE, "0").parse::<usize>().unwrap_or(0);

    mm_render_html!(
        Spacer {
            size
        }
    )
}
