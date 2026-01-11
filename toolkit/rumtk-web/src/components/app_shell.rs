use crate::components::{app_body::app_body, app_head::app_head};
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, LANG_EN};
use crate::utils::types::{HTMLResult, MMString, SharedAppConf, URLParams, URLPath};
use crate::{mm_get_text_item, mm_render_component, mm_render_html};
use askama::Template;

const DEFAULT_PAGE_NAME: &str = "index";

#[derive(Template)]
#[template(
    source = "
        <!DOCTYPE html>
        <html lang='{{lang}}'>
        {{head|safe}}
        {{body|safe}}
        </html>
    ",
    ext = "html"
)]
pub struct AppShell {
    head: MMString,
    lang: MMString,
    body: MMString,
}

pub fn app_shell(path_components: URLPath, params: URLParams, state: SharedAppConf) -> HTMLResult {
    let lang = mm_get_text_item!(params, "lang", LANG_EN);
    let theme = mm_get_text_item!(params, "theme", DEFAULT_TEXT_ITEM);
    // TODO: We need to reevaluate how to validate the options that should be standardized to avoid parameter injection as an attack vector.
    //owned_state.opts = *params.clone();

    //Config App
    let mut owned_state = state.lock().expect("Lock failure");
    owned_state.lang = MMString::from(lang);
    owned_state.theme = MMString::from(theme);

    //Let's render the head component
    let head =
        mm_render_component!(|| -> HTMLResult { app_head(path_components, params, state.clone()) });

    //Let's render the head component
    let body =
        mm_render_component!(|| -> HTMLResult { app_body(path_components, params, state.clone()) });

    mm_render_html!(AppShell {
        lang: MMString::from(lang),
        head,
        body
    })
}
