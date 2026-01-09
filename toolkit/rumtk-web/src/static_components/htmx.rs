use crate::mm_render_html;
use crate::utils::types::HTMLResult;
use askama::Template;

#[derive(Debug)]
pub struct HTMXElement {
    version: &'static str,
    sha: &'static str,
}

#[derive(Template, Debug)]
#[template(
    source = "
        <script src='https://cdn.jsdelivr.net/npm/htmx.org@{{lib.version}}/dist/htmx.min.js' integrity='{{lib.sha}}' crossorigin='anonymous'></script>
    ",
    ext = "html"
)]
pub struct HTMX {
    lib: HTMXElement,
}

pub fn htmx() -> HTMLResult {
    mm_render_html!(HTMX {
        lib: HTMXElement {
            version: "2.0.8",
            sha: "sha384-/TgkGk7p307TH7EXJDuUlgG3Ce1UVolAOFopFekQkkXihi5u/6OCvVKyz1W+idaz"
        }
    })
}
