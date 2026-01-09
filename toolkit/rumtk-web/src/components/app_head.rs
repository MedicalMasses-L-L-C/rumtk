use crate::static_components::{css::css, fontawesome::fontawesome, htmx::htmx, meta::meta};
use crate::utils::types::{HTMLResult, MMString, SharedAppState, URLParams, URLPath};
use crate::{mm_render_component, mm_render_html};
use askama::Template;

const DEFAULT_PAGE_NAME: &str = "index";

#[derive(Template)]
#[template(
    source = "
        <head>
            {{meta|safe}}
            {{css|safe}}
            {{fontawesome|safe}}
            {{htmx|safe}}
        </head>
    ",
    ext = "html"
)]
pub struct AppShellHead {
    meta: MMString,
    css: MMString,
    fontawesome: MMString,
    htmx: MMString,
}

///
///     !!!!!!!!!!!!!!!!!!!!!!!WARNING!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
///
///      The snippet below will add key static imports relying on CDN free bandwidth where possible.
///      Keep in mind this can be dangerous security wise if the CDN or DNS services are manipulated
///      because we will fallback on a local file version that may be of an older version.
///
///      It is not ideal but it will allow continuance of service for websites during CDN outages
///      which do happen.
///
pub fn app_head(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    //Let's render the head component
    let html_meta = mm_render_component!(meta, state);

    //Let's render the head component
    let html_css = mm_render_component!(css);

    //Let's render the head component
    let html_fontawesome = mm_render_component!(fontawesome);

    //Let's render the head component
    let html_htmx = mm_render_component!(htmx);

    mm_render_html!(AppShellHead {
        meta: html_meta,
        css: html_css,
        fontawesome: html_fontawesome,
        htmx: html_htmx
    })
}
