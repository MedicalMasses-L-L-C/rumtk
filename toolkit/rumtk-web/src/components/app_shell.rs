use crate::components::app_body::app_body;
use crate::components::COMPONENTS;
use crate::utils::defaults::{DEFAULT_TEXT_ITEM, LANG_EN};
use crate::utils::types::{HTMLResult, MMString, SharedAppState, URLParams, URLPath};
use crate::{mm_collect_page, mm_get_param, mm_get_text_item, mm_render_component, mm_render_html};
use askama::Template;

const DEFAULT_PAGE_NAME: &str = "index";

#[derive(Template)]
#[template(
    source = "
        <!DOCTYPE html>
        <html lang="{{lang}}">
        <head>
            <meta charset="UTF-8">
            <meta http-equiv="Content-Type" content="text/html; charset=utf-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <meta http-equiv="X-UA-Compatible" content="IE=edge,chrome=1"/>
            <meta name="description" content="{{description}}">
            <title>{{title}}</title>
            <link rel="icon" type="image/png" href="/static/img/icon.png">

            <!--- Styles (minified) for speed --->
            <style>
                @import "/static/core/default/theme.css";
                @import "/static/core/default/images.css";
                @import "/static/core/default/tags.css";
                @import "/static/core/basic_styles.css";
                @import "/static/core/fonts.css";
                @import "/static/core/animations.css";
                @import "/static/core/gap.css";

                body {
                    background-color: var(--color-jaguar);
                    color: white;

                    margin: auto;
                    width: 100%;
                    max-width: 100%;
                    height: auto;
                }
            </style>

            <!--- Fontawesome Inlined (minified) for speed --->
            <link rel="stylesheet" href="/static/fontawesome-free/css/fontawesome.min.css" />

            <!--- Fontawesome Regular Inlined (minified) for speed --->
            <link rel="stylesheet" href="/static/fontawesome-free/css/regular.min.css" />

            <!--- Fontawesome Brands Inlined (minified) for speed --->
            <link rel="stylesheet" href="/static/fontawesome-free/css/brands.min.css" />

            <!--- HTMX Inlined (minified) for speed --->
            <script type="module" src="/static/lib/htmx.min.js"></script>
        </head>
        <body class="f12 {{theme}}">
        {{header|safe}}
        <div class="" id="content">
            <div class="gap20">

            </div>
            {{body|safe}}
            <div class="gap5">

            </div>
        </div>
        {{footer|safe}}
        </body>
        </html>
    ",
    ext = "html"
)]
pub struct AppShell {
    title: MMString,
    description: MMString,
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