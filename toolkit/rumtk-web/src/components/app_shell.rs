use askama::Template;
use crate::{mm_collect_page, mm_get_param, mm_get_text_item, mm_render_component, mm_render_html};
use crate::components::app_body::app_body;
use crate::components::COMPONENTS;
use crate::utils::types::{HTMLResult, MMString, URLPath, URLParams, SharedAppState};
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, LANG_EN};

const DEFAULT_PAGE_NAME: &str = "index";

#[derive(Template)]
#[template(path = "index.html")]
struct AppShell {
    lang: MMString,
    theme: MMString,
    header: MMString,
    body: MMString,
    footer: MMString,
}

pub fn app_shell(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let page: MMString = mm_get_param!(path_components, 0, MMString::from(DEFAULT_TEXT_ITEM));

    let lang = mm_get_text_item!(params, "lang", LANG_EN);
    let theme = mm_get_text_item!(params, "theme", DEFAULT_TEXT_ITEM);

    //Let's render the body to html
    let body_components = mm_collect_page!(page, state);
    let body = app_body(&body_components)?.0;

    //Let's render the header and footer
    //<div class="" hx-get="/component/navbar" hx-target="#navbar" hx-trigger="load" id="navbar"></div>
    let header = mm_render_component!("navbar", [("", "")], state, COMPONENTS);
    //<div class="" hx-get="/component/footer?social_list=linkedin,github" hx-target="#footer" hx-trigger="load" id="footer"></div>
    let footer = mm_render_component!("footer", [("social_list", "linkedin,github")], state, COMPONENTS);

    //Config App
    let mut owned_state = state.lock().expect("Lock failure");
    owned_state.lang = MMString::from(lang);
    owned_state.theme = MMString::from(theme);
    // TODO: We need to reevaluate how to validate the options that should be standardized to avoid parameter injection as an attack vector.
    //owned_state.opts = *params.clone();

    mm_render_html!(
        AppShell {
            lang: MMString::from(lang),
            theme: MMString::from(theme),
            header,
            body,
            footer
        }
    )
}