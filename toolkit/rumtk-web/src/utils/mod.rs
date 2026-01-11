mod conf;
pub mod defaults;
pub mod matcher;
pub mod types;

use axum::response::Html;
use std::collections::HashMap;

use crate::utils::types::AppState;
use axum::{routing::get, Router};
use std::sync::Arc;
use std::sync::Mutex;
use tower_http::compression::{CompressionLayer, DefaultPredicate, Predicate};
use tower_http::services::ServeDir;
use types::*;

pub fn html_render<T: askama::Template>(template: T) -> HTMLResult {
    let result = template.render();
    match result {
        Ok(html) => Ok(Html(html)),
        Err(e) => {
            let tn = std::any::type_name::<T>();
            Err(format!("Template {tn} render failed: {e:?}"))
        }
    }
}
/*
pub fn html_component_render(component: &str, params: &[(MMString, MMString)], app_state: SharedAppConf, components: &ComponentMap) -> MMString {
    let component = match components.get(component) {
        Some(x) => x,
        None => return MMString::default(),
    };

    match component(
        &[],
        &HashMap::from(params),
        app_state
    ) {
        Ok(x) => x.0,
        Err(e) => MMString::default(),
    }
}
*/
#[macro_export]
macro_rules! mm_get_misc_conf {
    ( $typ:expr ) => {{
        use $crate::utils::defaults::*;
        match $typ {
            SECTION_SOCIALS => {
                use crate::conf::misc::SOCIAL_URLS as DEFAULT_SOCIAL_ICONS;
                &DEFAULT_SOCIAL_ICONS
            }
            SECTION_SERVICES => {
                use crate::conf::misc::SERVICES as DEFAULT_SERVICES;
                &DEFAULT_SERVICES
            }
            SECTION_PRODUCTS => {
                use crate::conf::misc::PRODUCTS as DEFAULT_PRODUCTS;
                &DEFAULT_PRODUCTS
            }
            _ => {
                use crate::conf::misc::API_ENDPOINTS as API_DEFAULT;
                &API_DEFAULT
            }
        }
    }};
}

#[macro_export]
macro_rules! mm_get_conf {
    ( $name:expr ) => {{
        use crate::conf::{IMG_DEFAULT, img};
        use crate::utils::defaults::SECTION_PERSONNEL;
        match $name {
            SECTION_PERSONNEL => &img::IMG_PERSONNEL,
            _ => &IMG_DEFAULT,
        }
    }};
    ( $name:expr, $lang:expr ) => {{
        use crate::utils::defaults::{
            LANG_ES, SECTION_CONTACT, SECTION_LINKS, SECTION_PERSONNEL, SECTION_TEXT,
            SECTION_TITLES,
        };
        match $lang {
            LANG_ES => {
                use crate::conf::{TEXT_DEFAULT, text_en};
                match $name {
                    SECTION_TEXT => &text_en::TEXT,
                    SECTION_PERSONNEL => &text_en::TEXT_PERSONNEL_INFO,
                    SECTION_CONTACT => &text_en::TEXT_CONTACT_INFO,
                    SECTION_TITLES => &text_en::TEXT_TITLES_TEXT,
                    SECTION_LINKS => &text_en::TEXT_LINKS_TEXT,
                    _ => &TEXT_DEFAULT,
                }
            }
            _ => {
                use crate::conf::{TEXT_DEFAULT, text_en};
                match $name {
                    SECTION_TEXT => &text_en::TEXT,
                    SECTION_PERSONNEL => &text_en::TEXT_PERSONNEL_INFO,
                    SECTION_CONTACT => &text_en::TEXT_CONTACT_INFO,
                    SECTION_TITLES => &text_en::TEXT_TITLES_TEXT,
                    SECTION_LINKS => &text_en::TEXT_LINKS_TEXT,
                    _ => &TEXT_DEFAULT,
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! mm_get_text_item {
    ( $store:expr, $item:expr, $default:expr) => {{
        match $store.get($item) {
            Some(x) => x,
            None => $default,
        }
    }};
}

#[macro_export]
macro_rules! mm_get_param_eq {
    ( $params:expr, $indx:expr, $comparison:expr, $default:expr ) => {{
        match $params.get($indx) {
            Some(x) => *x == $comparison,
            None => $default,
        }
    }};
}

#[macro_export]
macro_rules! mm_get_param {
    ( $params:expr, $indx:expr, $default:expr ) => {{
        match $params.get($indx) {
            Some(x) => x.parse().unwrap_or($default),
            None => $default,
        }
    }};
}

#[macro_export]
macro_rules! mm_params_map {
    ( $params:expr ) => {{
        use std::collections::HashMap;
        let mut params = HashMap::<MMString, MMString>::with_capacity($params.len());

        for (k, v) in $params.iter() {
            params.insert(k.to_string(), v.to_string());
        }
        params
    }};
}

#[macro_export]
macro_rules! mm_render_component {
    ( $component_fxn:expr ) => {{
        match $component_fxn() {
            Ok(x) => x.0,
            Err(e) => MMString::default(),
        }
    }};
    ( $component_fxn:expr, $app_state:expr ) => {{
        match $component_fxn($app_state.clone()) {
            Ok(x) => x.0,
            Err(e) => MMString::default(),
        }
    }};
    ( $component:expr, $params:expr, $app_state:expr, $components:expr ) => {{
        use $crate::components::div::div;
        use $crate::mm_params_map;
        use $crate::utils::types::ComponentFunction;
        let component = match $components.get($component) {
            Some(x) => x,
            None => &(div as ComponentFunction),
        };

        match component(&[], &mm_params_map!($params), $app_state.clone()) {
            Ok(x) => x.0,
            _ => MMString::default(),
        }
    }};
}

#[macro_export]
macro_rules! mm_collect_page {
    ( $page:expr, $app_state:expr ) => {{
        use $crate::pages::PAGES;
        use $crate::utils::types::{PageFunction, RenderedPageComponents};

        let page = match PAGES.get(&$page) {
            Some(x) => x,
            None => &(|_| -> RenderedPageComponents { vec![] } as PageFunction),
        };

        page($app_state.clone())
    }};
}

#[macro_export]
macro_rules! mm_fetch {
    ( $matcher:expr ) => {{
        use axum::extract::{Path, Query, State};
        use axum::response::Html;
        use $crate::utils::types::{MMString, RouterAppConf, RouterComponents, RouterParams};
        async |Path(path_params): RouterComponents,
               Query(params): RouterParams,
               State(state): RouterAppConf|
               -> Html<String> {
            match $matcher(path_params, params, state).await {
                Ok(res) => res,
                Err(e) => {
                    error!("{}", e);
                    return Html(MMString::default());
                }
            }
        }
    }};
}

use crate::utils::matcher::*;
use tracing::error;

pub async fn run_app(ip: &str) {
    let state = Arc::new(Mutex::new(AppState::default()));
    let comression_layer: CompressionLayer = CompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true)
        .compress_when(DefaultPredicate::new());
    let app = Router::new()
        /* Robots.txt */
        .route("/robots.txt", get(mm_fetch!(default_robots_matcher)))
        /* Components */
        .route(
            "/component/{*name}",
            get(mm_fetch!(default_component_matcher)),
        )
        /* Pages */
        .route("/", get(mm_fetch!(default_page_matcher)))
        .route("/{*page}", get(mm_fetch!(default_page_matcher)))
        /* Services */
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state)
        .layer(comression_layer);

    let listener = tokio::net::TcpListener::bind(&ip)
        .await
        .expect("There was an issue biding the listener.");
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("There was an issue with the server.");
}

#[macro_export]
macro_rules! mm_run_app {
    (  ) => {{
        use $crate::utils::defaults::DEFAULT_LOCAL_LISTENING_ADDRESS;
        use $crate::utils::run_app;
        run_app(DEFAULT_LOCAL_LISTENING_ADDRESS).await;
    }};
    ( $ip:expr ) => {{
        use $crate::utils::run_app;
        run_app(&$ip).await;
    }};
    ( $ip:expr, $port:expr ) => {{
        use $crate::utils::run_app;
        run_app(&format!("{}:{}", $ip, $port)).await;
    }};
}

#[macro_export]
macro_rules! mm_render_html {
    ( $component:expr ) => {{
        use crate::utils::{html_render, types::HTMLResult};

        let closure = || -> HTMLResult { html_render($component) };

        closure()
    }};
}

///
///
/// If using raw strings, do not leave an extra line. The first input must have characters or you will get <pre><code> blocks regardless of what you do.
#[macro_export]
macro_rules! mm_render_markdown {
    ( $md:expr ) => {{
        use pulldown_cmark::{Options, Parser};
        use $crate::utils::types::MMString;

        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_MATH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_WIKILINKS);

        let input = MMString::from($md);
        let parser = Parser::new_ext(&input, options);
        let mut html_output = MMString::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);
        println!("{}", &html_output);

        html_output
    }};
}
