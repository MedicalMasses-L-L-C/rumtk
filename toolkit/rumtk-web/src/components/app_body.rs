use crate::components::COMPONENTS;
use crate::utils::defaults::DEFAULT_TEXT_ITEM;
use crate::utils::types::{HTMLResult, MMString, SharedAppState, URLParams, URLPath};
use crate::{mm_collect_page, mm_get_param, mm_get_text_item, mm_render_component, mm_render_html};
use askama::Template;

#[derive(Template)]
#[template(
    source = "
    {% for element in elements %}
        {{ element|safe }}
    {% endfor %}
    ",
    ext = "html"
)]
pub struct AppBodyContents<'a> {
    elements: &'a [MMString],
}

fn app_body_contents(elements: &[MMString]) -> HTMLResult {
    mm_render_html!(AppBodyContents { elements })
}

#[derive(Template)]
#[template(
    source = "
        <body class='f12 {{theme}}'>
            {{header|safe}}
            <div class='' id='content'>
                <div class='gap20'>
    
                </div>
                {{body|safe}}
                <div class='gap5'>
    
                </div>
            </div>
            {{footer|safe}}
        </body>
    ",
    ext = "html"
)]
pub struct AppBody {
    theme: MMString,
    header: MMString,
    body: MMString,
    footer: MMString,
}

pub fn app_body(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let page: MMString = mm_get_param!(path_components, 0, MMString::from(DEFAULT_TEXT_ITEM));
    let theme = mm_get_text_item!(params, "theme", DEFAULT_TEXT_ITEM);

    //Let's render the body to html
    let body_components = mm_collect_page!(page, state);
    let body = app_body_contents(&body_components)?.0;

    //Let's render the header and footer
    //<div class="" hx-get="/component/navbar" hx-target="#navbar" hx-trigger="load" id="navbar"></div>
    let header = mm_render_component!("navbar", [("", "")], state, COMPONENTS);
    //<div class="" hx-get="/component/footer?social_list=linkedin,github" hx-target="#footer" hx-trigger="load" id="footer"></div>
    let footer = mm_render_component!(
        "footer",
        [("social_list", "linkedin,github")],
        state,
        COMPONENTS
    );

    mm_render_html!(AppBody {
        theme: MMString::from(theme),
        header,
        body,
        footer
    })
}
