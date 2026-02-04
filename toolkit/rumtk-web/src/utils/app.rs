/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D.
 * Copyright (C) 2025  Nick Stephenson
 * Copyright (C) 2025  Ethan Dixon
 * Copyright (C) 2025  MedicalMasses L.L.C.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
 */
use crate::components::{form::Forms, UserComponents};
use crate::css::DEFAULT_OUT_CSS_DIR;
use crate::pages::UserPages;
use crate::utils::defaults::DEFAULT_LOCAL_LISTENING_ADDRESS;
use crate::utils::matcher::*;
use crate::{
    rumtk_web_api_process, rumtk_web_compile_css_bundle, rumtk_web_init_api_endpoints,
    rumtk_web_init_components, rumtk_web_init_forms, rumtk_web_init_job_manager,
    rumtk_web_init_pages,
};
use crate::{rumtk_web_fetch, rumtk_web_load_conf};

use rumtk_core::core::RUMResult;
use rumtk_core::dependencies::clap;
use rumtk_core::rumtk_resolve_task;
use rumtk_core::strings::RUMString;
use rumtk_core::threading::threading_functions::get_default_system_thread_count;
use rumtk_core::types::{RUMCLIParser, RUMTcpListener};

use crate::api::UserAPIEndpoints;
use axum::routing::{get, post};
use axum::Router;
use tower_http::compression::{CompressionLayer, DefaultPredicate};
use tower_http::services::ServeDir;

const DEFAULT_UPLOAD_LIMIT: usize = 10240;

///
/// RUMTK WebApp CLI Args
///
#[derive(RUMCLIParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///
    /// Website title to use internally. It can be omitted if defined in the app.json config file
    /// bundled with your app.
    ///
    #[arg(long, default_value = "")]
    pub title: RUMString,
    ///
    /// Website description string. It can be omitted if defined in the app.json config file
    /// bundled with your app.
    ///
    #[arg(long, default_value = "")]
    pub description: RUMString,
    ///
    /// Company to display in website.
    ///
    #[arg(long, default_value = "")]
    pub company: RUMString,
    ///
    /// Copyright year to display in website.
    ///
    #[arg(short, long, default_value = "")]
    pub copyright: RUMString,
    ///
    /// Directory to scan on startup to find custom CSS sources to bundle into a minified CSS file
    /// that can be quickly pulled by the app client side.
    ///
    /// This option can provide an alternative to direct component retrieval of CSS fragments.
    /// Meaning, you could bundle all of your fragments into the master bundle at startup and
    /// turn off component level ```custom_css_enabled``` option in the ```app.json``` config.
    ///
    #[arg(long, default_value = DEFAULT_OUT_CSS_DIR)]
    pub css_source_dir: RUMString,
    ///
    /// Is the interface meant to be bound to the loopback address and remain hidden from the
    /// outside world.
    ///
    /// It follows the format ```IPv4:port``` and it is a string.
    ///
    /// If a NIC IP is defined via `--ip`, that value will override this flag.
    ///
    #[arg(short, long, default_value = DEFAULT_LOCAL_LISTENING_ADDRESS)]
    pub ip: RUMString,
    ///
    /// Specify the size limit for a file upload post request.
    ///
    #[arg(long, default_value_t = DEFAULT_UPLOAD_LIMIT)]
    pub upload_limit: usize,
    ///
    /// How many threads to use to serve the website. By default, we use
    /// ```get_default_system_thread_count()``` from ```rumtk-core``` to detect the total count of
    /// cpus available. We use the system's total count of cpus by default.
    ///
    #[arg(long, default_value_t = get_default_system_thread_count())]
    pub threads: usize,
    ///
    /// How many threads to use to serve the website. By default, we use
    /// ```get_default_system_thread_count()``` from ```rumtk-core``` to detect the total count of
    /// cpus available. We use the system's total count of cpus by default.
    ///
    #[arg(long, default_value_t = false)]
    pub skip_default_css: bool,
}

async fn run_app(args: Args, skip_serve: bool) -> RUMResult<()> {
    let state = rumtk_web_load_conf!(&args);
    let comression_layer: CompressionLayer = CompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true)
        .compress_when(DefaultPredicate::new());
    let app = Router::new()
        /* Robots.txt */
        .route("/robots.txt", get(rumtk_web_fetch!(default_robots_matcher)))
        /* Components */
        .route(
            "/component/{*name}",
            get(rumtk_web_fetch!(default_component_matcher)),
        )
        /* Pages */
        .route("/", get(rumtk_web_fetch!(default_page_matcher)))
        .route("/{*page}", get(rumtk_web_fetch!(default_page_matcher)))
        /* Post Handling */
        .route("/api/", post(rumtk_web_api_process!(default_api_matcher)))
        //.layer(DefaultBodyLimit::max(args.upload_limit))
        .route(
            "/api/{*page}",
            post(rumtk_web_api_process!(default_api_matcher)),
        )
        //.layer(DefaultBodyLimit::max(args.upload_limit))
        /* Services */
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state)
        .layer(comression_layer);

    let listener = RUMTcpListener::bind(&args.ip.as_str())
        .await
        .expect("There was an issue biding the listener.");
    println!("listening on {}", listener.local_addr().unwrap());

    if !skip_serve {
        axum::serve(listener, app)
            .await
            .expect("There was an issue with the server.");
    }

    Ok(())
}

pub fn app_main(
    pages: &UserPages,
    components: &UserComponents,
    forms: &Forms,
    apis: &UserAPIEndpoints,
    skip_serve: bool,
    skip_default_css: bool,
) {
    let args = Args::parse();

    rumtk_web_init_components!(components);
    rumtk_web_init_pages!(pages);
    rumtk_web_init_forms!(forms);
    rumtk_web_init_api_endpoints!(apis);
    rumtk_web_compile_css_bundle!(
        &args.css_source_dir,
        &args.skip_default_css | skip_default_css
    );

    rumtk_web_init_job_manager!(&args.threads);
    let task = run_app(args, skip_serve);
    rumtk_resolve_task!(task);
}

///
/// This is the main macro for defining your applet and launching it.
/// Usage is very simple and the only decision from a user is whether to pass a list of
/// [UserPages](UserPages) or a list of [UserPages](UserPages) and a list
/// of [UserComponents](UserComponents).
///
/// These lists are used to automatically register your pages
/// (e.g. `/index => ('index', my_index_function)`) and your custom components
/// (e.g. `button => ('button', my_button_function)`
///
/// This macro will load CSS from predefined sources, concatenate their contents with predefined CSS,
/// minified the concatenated results, and generate a bundle css file containing the minified results.
/// The CSS bundle is written to file `./static/css/bundle.min.css`.
///
/// ***Note: anything in ./static will be considered static assets that need to be served.***
///
/// This macro will also parse the command line automatically with a few predefined options and
/// use that information to override the config defaults.
///
/// By default, the app is launched to `127.0.0.1:3000` which is the loopback address.
///
/// App is served with the best compression algorithm allowed by the client browser.
///
/// For testing purposes, the function
///
/// ## Example Usage
///
/// ### With Page and Component definition
/// ```
///     use rumtk_core::strings::{rumtk_format};
///     use rumtk_web::{
///         rumtk_web_run_app,
///         rumtk_web_render_component,
///         rumtk_web_render_html,
///         rumtk_web_get_text_item
///     };
///     use rumtk_web::components::form::{FormElementBuilder, props::InputProps, FormElements};
///     use rumtk_web::{SharedAppState, RenderedPageComponents};
///     use rumtk_web::{APIPath, URLPath, URLParams, HTMLResult, RUMString, RouterForm, FormData, RUMWebData};
///     use rumtk_web::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CONTENTS, PARAMS_CSS_CLASS};
///     use rumtk_web::utils::types::RUMWebTemplate;
///
///
///
///
///     // About page
///     pub fn about(app_state: SharedAppState) -> RenderedPageComponents {
///         let title_coop = rumtk_web_render_component!("title", [("type", "coop_values")], app_state.clone());
///         let title_team = rumtk_web_render_component!("title", [("type", "meet_the_team")], app_state.clone());
///     
///         let text_card_story = rumtk_web_render_component!("text_card", [("type", "story")], app_state.clone());
///         let text_card_coop = rumtk_web_render_component!("text_card", [("type", "coop_values")], app_state.clone());
///     
///         let portrait_card = rumtk_web_render_component!("portrait_card", [("section", "company"), ("type", "personnel")], app_state.clone());
///     
///         let spacer_5 = rumtk_web_render_component!("spacer", [("size", "5")], app_state.clone());
///     
///         vec![
///             text_card_story,
///             spacer_5.clone(),
///             title_coop,
///             text_card_coop,
///             spacer_5,
///             title_team,
///             portrait_card
///         ]
///     }
///
///     //Custom component
///     #[derive(RUMWebTemplate, Debug)]
///     #[template(
///             source = "
///                {% if custom_css_enabled %}
///                    <link href='/static/components/div.css' rel='stylesheet'>
///                {% endif %}
///                <div class='div-{{css_class}}'>{{contents|safe}}</div>
///            ",
///            ext = "html"
///     )]
///     struct MyDiv {
///         contents: RUMString,
///         css_class: RUMString,
///         custom_css_enabled: bool,
///     }
///
///     fn my_div(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
///         let contents = rumtk_web_get_text_item!(params, PARAMS_CONTENTS, DEFAULT_TEXT_ITEM);
///         let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);
///
///         let custom_css_enabled = state.read().expect("Lock failure").get_config().custom_css;
///
///         rumtk_web_render_html!(MyDiv {
///             contents: RUMString::from(contents),
///             css_class: RUMString::from(css_class),
///             custom_css_enabled
///         })
///     }
///
///     fn my_form (builder: FormElementBuilder) -> FormElements {
///         vec![
///             builder("input", "", InputProps::default(), "default")
///         ]
///     }
///
///     fn my_api_handler(path: APIPath, params: RUMWebData, form: FormData, state: SharedAppState) -> HTMLResult {
///         Err(rumtk_format!(
///             "No handler registered for API endpoint => {}",
///             path
///         ))
///     }
///
///     //Requesting to immediately exit instead of indefinitely serving pages so this example can be used as a unit test.
///     let skip_serve = true;
///     let skip_default_css = false;
///
///     let result = rumtk_web_run_app!(
///         vec![("about", about)],
///         vec![("my_div", my_div)], //Optional, can be omitted alongside the skip_serve flag
///         vec![("my_form", my_form)], //Optional, can be omitted alongside the skip_serve flag
///         vec![("v2/add", my_api_handler)], //Optional, can be omitted alongside the skip_serve flag
///         skip_serve, //Omit in production code. This is used so that this example can work as a unit test.
///         skip_default_css //Omit in production code. This is used so that this example can work as a unit test.
///     );
/// ```
///
#[macro_export]
macro_rules! rumtk_web_run_app {
    (  ) => {{
        use $crate::utils::app::app_main;

        app_main(&vec![], &vec![], &vec![], &vec![], false, false)
    }};
    ( $pages:expr ) => {{
        use $crate::utils::app::app_main;

        app_main(&$pages, &vec![], &vec![], &vec![], false, false)
    }};
    ( $pages:expr, $components:expr ) => {{
        use $crate::utils::app::app_main;

        app_main(&$pages, &$components, &vec![], &vec![], false, false)
    }};
    ( $pages:expr, $components:expr, $forms:expr ) => {{
        use $crate::utils::app::app_main;

        app_main(&$pages, &$components, &$forms, &vec![], false, false)
    }};
    ( $pages:expr, $components:expr, $forms:expr, $apis:expr ) => {{
        use $crate::utils::app::app_main;

        app_main(&$pages, &$components, &$forms, &$apis, false, false)
    }};
    ( $pages:expr, $components:expr, $forms:expr, $apis:expr, $skip_serve:expr, $skip_default_css:expr ) => {{
        use $crate::utils::app::app_main;

        app_main(
            &$pages,
            &$components,
            &$forms,
            &$apis,
            $skip_serve,
            $skip_default_css,
        )
    }};
}
