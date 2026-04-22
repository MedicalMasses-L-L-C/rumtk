/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D. <lsantos@medicalmasses.com>
 * Copyright (C) 2025  Ethan Dixon
 * Copyright (C) 2025  MedicalMasses L.L.C. <contact@medicalmasses.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
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

///
/// Struct encapsulating custom-made items to register with the framework.
///
/// ## Pages
/// The `pages` field accepts an optional of [UserPages] which is a vector of [PageItem](crate::pages::PageItem).
///
/// ```text
///     vec![
///         ("my_page", my_page_function),
///         ...
///     ];
/// ```
///
/// The page function is of type [PageFunction](crate::utils::types::PageFunction).
///
/// It is important to understand that a `page` is a function that simply list the series of components to be rendered.
/// We rely on `CSS` for the actual layout in 2D space. Therefore, a page function should not prescribe the page layout per se.
///
/// ## Components
/// The `components` field takes an optional of [UserComponents] which is a vector of [UserComponentItem](crate::components::UserComponentItem).
///
/// ```text
///     vec![
///         ("my_component", my_component_function),
///         ...
///     ];
/// ```
///
/// The component function is of type [PageFunction](crate::utils::types::ComponentFunction).
///
/// ## Forms
/// The `forms` field takes an optional of [Forms] which is a vector of [FormItem](crate::components::form::FormItem).
///
/// ```text
///     vec![
///         ("my_form", my_form_function),
///         ...
///     ];
/// ```
///
/// The form function is of type [FormBuilderFunction](crate::components::form::FormBuilderFunction).
///
/// Although a `form` is treated as a type of component in the framework, its implementation and behavior is closer to
/// a `page` in that its main role is to define the vector of [FormElementBuilder](crate::components::form::FormElementBuilder).
/// These element builder functions further renders the actual form component to be inserted in linear order. Again, these functions
/// say nothing about the layout of the form as that is handled via `CSS`.
///
/// ## APIs
/// The `apis` field takes an optional of [UserAPIEndpoints] which is a vector of [APIItem](crate::api::APIItem).
///
/// ```text
///     vec![
///         ("/my/api/endpoint", my_api_handler),
///         ...
///     ];
/// ```
///
/// The api function is of type [APIFunction](crate::utils::types::APIFunction).
///
/// These API functions are your handlers directly mapped to the REST route you wish to intercept. This enables a
/// simple key-value pair approach to defining API endpoints in your web app. These handlers can queue asynchronous
/// pipeline jobs, return HTML fragments, redirect the current page somewhere else, or a combination of these.
/// It is a powerful interface for organizing your routing.
///
#[derive(Default, Debug, PartialEq)]
pub struct AppComponents<'a> {
    pub pages: Option<UserPages<'a>>,
    pub components: Option<UserComponents<'a>>,
    pub forms: Option<Forms<'a>>,
    pub apis: Option<UserAPIEndpoints<'a>>,
}

///
/// Struct for defining the global switches to drive the initialization of the web app. This struct
/// works hand in hand with [AppComponents].
///
#[derive(Default, Debug, PartialEq)]
pub struct AppSwitches {
    pub skip_serve: bool,
    pub skip_default_css: bool,
}

///
/// Main API function for running and serving the web application.
///
/// It takes an [AppComponents] instance and a few switches to help preconfigure the framework to
/// use custom-made components and to register API endpoints.
///
/// See [rumtk_web_run_app](crate::rumtk_web_run_app) for more details.
///
/// ## Example
/// ```
/// use rumtk_web::app_main;
/// use rumtk_web::{rumtk_web_register_app_switches, rumtk_web_register_app_components};
///
/// // We pass true to the switches because we do not want the web server to actually serve the page
/// // It would hang the test otherwise...
/// app_main(
///     rumtk_web_register_app_components!(),
///     rumtk_web_register_app_switches!(true)
/// ).expect("Issue occurred while running the app");
/// ```
///
pub fn app_main(app_components: AppComponents<'_>, switches: AppSwitches) -> RUMResult<()> {
    let args = Args::parse();

    rumtk_web_init_components!(app_components.components);
    rumtk_web_init_pages!(app_components.pages);
    rumtk_web_init_forms!(app_components.forms);
    rumtk_web_init_api_endpoints!(app_components.apis);
    rumtk_web_compile_css_bundle!(
        &args.css_source_dir,
        &args.skip_default_css | switches.skip_default_css
    );

    rumtk_web_init_job_manager!(&args.threads);
    let task = run_app(args, switches.skip_serve);
    rumtk_resolve_task!(task)
}

///
/// Convenience macro for quickly building the [AppComponents] object. Feel free to pass an instance of
/// [AppComponents] directly to [run_app] or [rumtk_web_run_app](crate::rumtk_web_run_app).
///
/// Passing no parameters generates an "empty" instance, meaning you would be asking the framework that you only
/// care about built-in components. This also implies you do not want to process API endpoints.
///
/// ## Examples
///
/// ### Without Parameters
/// ```
/// use crate::rumtk_web::AppComponents;
/// use crate::rumtk_web::rumtk_web_register_app_components;
///
/// let expected = AppComponents::default();
/// let result = rumtk_web_register_app_components!();
///
/// assert_eq!(result, expected, "Default macro-generated instance of AppComponents are not the same!");
///
/// ```
///
/// ### With Existing Page
/// ```
/// use rumtk_web::pages::UserPages;
/// use crate::rumtk_web::AppComponents;
/// use crate::rumtk_web::pages::index::index;
/// use crate::rumtk_web::rumtk_web_register_app_components;
///
/// let my_pages: UserPages = vec![
///     ("myindex", index)
/// ];
/// let expected = AppComponents {
///     pages: Some(my_pages.clone()),
///     components: None,
///     forms: None,
///     apis: None,
///  };
/// let result = rumtk_web_register_app_components!(my_pages);
///
/// assert_eq!(result, expected, "Default macro-generated instance of AppComponents are not the same!");
///
/// ```
///
/// ### With Existing Page and Component
/// ```
/// use rumtk_web::components::UserComponents;
/// use rumtk_web::pages::UserPages;
/// use rumtk_web::AppComponents;
/// use rumtk_web::pages::index::index;
/// use rumtk_web::components::div::div;
/// use rumtk_web::rumtk_web_register_app_components;
///
/// let my_pages: UserPages = vec![
///     ("myindex", index)
/// ];
/// let my_components: UserComponents = vec![
///     ("mydiv", div)
/// ];
/// let expected = AppComponents {
///     pages: Some(my_pages.clone()),
///     components: Some(my_components.clone()),
///     forms: None,
///     apis: None,
///  };
/// let result = rumtk_web_register_app_components!(my_pages, my_components);
///
/// assert_eq!(result, expected, "Default macro-generated instance of AppComponents are not the same!");
///
/// ```
///
/// ### With Existing Page and Component and Form
/// ```
/// use rumtk_core::strings::RUMString;
/// use rumtk_web::AppComponents;
/// use rumtk_web::components::UserComponents;
/// use rumtk_web::pages::UserPages;
/// use rumtk_web::components::form::{FormElementBuilder, FormElements, Forms};
/// use rumtk_web::pages::index::index;
/// use rumtk_web::components::div::div;
/// use rumtk_web::components::form::props::InputProps;
/// use rumtk_web::rumtk_web_register_app_components;
///
/// fn upload_form(builder: FormElementBuilder) -> FormElements {
///     vec![
///         builder(
///             "input",
///             "",
///             InputProps {
///                 id: Some("file"),
///                 name: Some("file"),
///                 for_element: None,
///                 typ: Some("file"),
///                 value: None,
///                 max: None,
///                 placeholder: Some("path/to/file"),
///                 pattern: None,
///                 accept: Some(".pdf,application/pdf"),
///                 alt: None,
///                 aria_label: Some("PDF File Picker"),
///                 event_handlers: None,
///                 max_length: None,
///                 min_length: None,
///                 autocapitalize: false,
///                 autocomplete: false,
///                 autocorrect: false,
///                 autofocus: false,
///                 disabled: false,
///                 hidden: false,
///                 required: true,
///                 multiple: false,
///             },
///             ""
///         ),
///         builder(
///             "input",
///             "",
///             InputProps {
///                 id: Some("submit"),
///                 name: None,
///                 for_element: None,
///                 typ: Some("submit"),
///                 value: Some("Send"),
///                 max: None,
///                 placeholder: None,
///                 pattern: None,
///                 accept: None,
///                 alt: None,
///                 aria_label: Some("PDF File Submit Button"),
///                 event_handlers: None,
///                 max_length: None,
///                 min_length: None,
///                 autocapitalize: false,
///                 autocomplete: false,
///                 autocorrect: false,
///                 autofocus: false,
///                 disabled: false,
///                 hidden: false,
///                 required: false,
///                 multiple: false,
///             },
///             "f18"
///         ),
///         builder(
///             "progress",
///             "",
///             InputProps {
///                 id: Some("progress"),
///                 name: None,
///                 for_element: None,
///                 typ: None,
///                 value: Some("0"),
///                 max: Some("100"),
///                 placeholder: None,
///                 pattern: None,
///                 accept: None,
///                 alt: None,
///                 aria_label: Some("PDF File Submit Progress Bar"),
///                 event_handlers: None,
///                 max_length: None,
///                 min_length: None,
///                 autocapitalize: false,
///                 autocomplete: false,
///                 autocorrect: false,
///                 autofocus: false,
///                 disabled: false,
///                 hidden: true,
///                 required: false,
///                 multiple: false,
///             },
///             ""
///         ),
///     ]
/// }
///
/// let my_pages: UserPages = vec![
///     ("myindex", index)
/// ];
/// let my_components: UserComponents = vec![
///     ("mydiv", div)
/// ];
/// let my_forms: Forms = vec![
///     ("myform", upload_form)
/// ];
/// let expected = AppComponents {
///     pages: Some(my_pages.clone()),
///     components: Some(my_components.clone()),
///     forms: Some(my_forms.clone()),
///     apis: None,
///  };
/// let result = rumtk_web_register_app_components!(my_pages, my_components, my_forms);
///
/// assert_eq!(result, expected, "Default macro-generated instance of AppComponents are not the same!");
///
/// ```
///
/// ### With Existing Page and Component and Form and API Endpoint
/// ```
/// use rumtk_core::{rumtk_pipeline_run_async, rumtk_pipeline_command};
/// use rumtk_core::strings::{RUMString, RUMStringConversions, RUMArrayConversions};
/// use rumtk_web::{APIPath, AppComponents, FormData, HTMLResult, RUMWebData, SharedAppState};
/// use rumtk_web::{rumtk_web_get_job_manager, rumtk_web_render_component, rumtk_web_render_page_contents};
/// use rumtk_web::api::UserAPIEndpoints;
/// use rumtk_web::components::UserComponents;
/// use rumtk_web::pages::UserPages;
/// use rumtk_web::components::form::{FormElementBuilder, FormElements, Forms};
/// use rumtk_web::pages::index::index;
/// use rumtk_web::components::div::div;
/// use rumtk_web::components::form::props::InputProps;
/// use rumtk_web::jobs::{JobResult, JobResultType};
/// use rumtk_web::utils::defaults::{PARAMS_TARGET};
/// use rumtk_web::rumtk_web_register_app_components;
///
/// async fn upload_processor(form: FormData) -> JobResult {
///     let id = form.form.get("file").unwrap();
///     let file = form.files.get(id).unwrap();
///
///     let result = rumtk_pipeline_run_async!(
///         rumtk_pipeline_command!("cat", file.clone()),
///         rumtk_pipeline_command!("wc")
///     ).await?;
///
///     Ok(JobResultType::JSON(result.to_vec().to_rumstring()))
/// }
///
/// pub fn process_upload(path: APIPath, params: RUMWebData, form: FormData, state: SharedAppState) -> HTMLResult {
///     let job_id = rumtk_web_get_job_manager!()?.spawn_task(upload_processor(form))?;
///     let mydiv = rumtk_web_render_component!("mydiv", [(PARAMS_TARGET, job_id)], state)?.to_rumstring();
///
///     rumtk_web_render_page_contents!(
///         &vec![
///             mydiv
///         ]
///     )
/// }
///
/// fn upload_form(builder: FormElementBuilder) -> FormElements {
///     vec![
///         builder(
///             "input",
///             "",
///             InputProps {
///                 id: Some("file"),
///                 name: Some("file"),
///                 for_element: None,
///                 typ: Some("file"),
///                 value: None,
///                 max: None,
///                 placeholder: Some("path/to/file"),
///                 pattern: None,
///                 accept: Some(".pdf,application/pdf"),
///                 alt: None,
///                 aria_label: Some("PDF File Picker"),
///                 event_handlers: None,
///                 max_length: None,
///                 min_length: None,
///                 autocapitalize: false,
///                 autocomplete: false,
///                 autocorrect: false,
///                 autofocus: false,
///                 disabled: false,
///                 hidden: false,
///                 required: true,
///                 multiple: false,
///             },
///             ""
///         ),
///         builder(
///             "input",
///             "",
///             InputProps {
///                 id: Some("submit"),
///                 name: None,
///                 for_element: None,
///                 typ: Some("submit"),
///                 value: Some("Send"),
///                 max: None,
///                 placeholder: None,
///                 pattern: None,
///                 accept: None,
///                 alt: None,
///                 aria_label: Some("PDF File Submit Button"),
///                 event_handlers: None,
///                 max_length: None,
///                 min_length: None,
///                 autocapitalize: false,
///                 autocomplete: false,
///                 autocorrect: false,
///                 autofocus: false,
///                 disabled: false,
///                 hidden: false,
///                 required: false,
///                 multiple: false,
///             },
///             "f18"
///         ),
///         builder(
///             "progress",
///             "",
///             InputProps {
///                 id: Some("progress"),
///                 name: None,
///                 for_element: None,
///                 typ: None,
///                 value: Some("0"),
///                 max: Some("100"),
///                 placeholder: None,
///                 pattern: None,
///                 accept: None,
///                 alt: None,
///                 aria_label: Some("PDF File Submit Progress Bar"),
///                 event_handlers: None,
///                 max_length: None,
///                 min_length: None,
///                 autocapitalize: false,
///                 autocomplete: false,
///                 autocorrect: false,
///                 autofocus: false,
///                 disabled: false,
///                 hidden: true,
///                 required: false,
///                 multiple: false,
///             },
///             ""
///         ),
///     ]
/// }
///
/// let my_pages: UserPages = vec![
///     ("myindex", index)
/// ];
/// let my_components: UserComponents = vec![
///     ("mydiv", div)
/// ];
/// let my_forms: Forms = vec![
///     ("myform", upload_form)
/// ];
/// let my_endpoints: UserAPIEndpoints = vec![
///     ("/api/upload", process_upload)
/// ];
/// let expected = AppComponents {
///     pages: Some(my_pages.clone()),
///     components: Some(my_components.clone()),
///     forms: Some(my_forms.clone()),
///     apis: Some(my_endpoints.clone()),
///  };
/// let result = rumtk_web_register_app_components!(my_pages, my_components, my_forms, my_endpoints);
///
/// assert_eq!(result, expected, "Default macro-generated instance of AppComponents are not the same!");
///
/// ```
///
#[macro_export]
macro_rules! rumtk_web_register_app_components {
    (  ) => {{
        use $crate::utils::app::AppComponents;

        AppComponents::default()
    }};
    ( $pages:expr ) => {{
        use $crate::utils::app::AppComponents;

        AppComponents {
            pages: Some($pages),
            components: None,
            forms: None,
            apis: None,
        }
    }};
    ( $pages:expr, $components:expr ) => {{
        use $crate::utils::app::AppComponents;

        AppComponents {
            pages: Some($pages),
            components: Some($components),
            forms: None,
            apis: None,
        }
    }};
    ( $pages:expr, $components:expr, $forms:expr ) => {{
        use $crate::utils::app::AppComponents;

        AppComponents {
            pages: Some($pages),
            components: Some($components),
            forms: Some($forms),
            apis: None,
        }
    }};
    ( $pages:expr, $components:expr, $forms:expr, $apis:expr ) => {{
        use $crate::utils::app::AppComponents;

        AppComponents {
            pages: Some($pages),
            components: Some($components),
            forms: Some($forms),
            apis: Some($apis),
        }
    }};
}

///
/// Convenience macro for generating a [AppSwitches] instance containing the boolean options a
/// framework consumer would like to opt-in.
///
/// ## Examples
/// ```
/// use rumtk_web::AppSwitches;
/// use rumtk_web::{rumtk_web_register_app_switches};
///
/// let expected = AppSwitches {
///     skip_serve: true,
///     skip_default_css: false
/// };
/// let switches = rumtk_web_register_app_switches!(true);
///
/// assert_eq!(switches, expected, "The switches constructed to config app does not match the expected.");
/// ```
///
#[macro_export]
macro_rules! rumtk_web_register_app_switches {
    (  ) => {{
        use $crate::utils::app::AppSwitches;

        AppSwitches::default()
    }};
    ( $skip_serve:expr ) => {{
        use $crate::utils::app::AppSwitches;

        AppSwitches {
            skip_serve: $skip_serve,
            skip_default_css: false,
        }
    }};
    ( $skip_serve:expr, $skip_default_css:expr ) => {{
        use $crate::utils::app::AppSwitches;

        AppSwitches {
            skip_serve: $skip_serve,
            skip_default_css: $skip_default_css,
        }
    }};
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
///     use rumtk_web::{rumtk_web_run_app, rumtk_web_register_app_components, rumtk_web_render_component, rumtk_web_render_template, rumtk_web_get_text_item, rumtk_web_register_app_switches, rumtk_web_get_config};
///     use rumtk_web::components::form::{FormElementBuilder, props::InputProps, FormElements};
///     use rumtk_web::{SharedAppState, RenderedPageComponentsResult};
///     use rumtk_web::{APIPath, URLPath, URLParams, HTMLResult, RUMString, RouterForm, FormData, RUMWebData, AppConf};
///     use rumtk_web::defaults::{DEFAULT_TEXT_ITEM, PARAMS_CONTENTS, PARAMS_CSS_CLASS, PARAMS_TYPE};
///     use rumtk_web::utils::types::RUMWebTemplate;
///
///
///
///
///     // About page
///     pub fn about(app_state: SharedAppState) -> RenderedPageComponentsResult {
///         let title_coop = rumtk_web_render_component!("title", [(PARAMS_TYPE, "coop_values")], app_state)?.to_rumstring();
///         let title_team = rumtk_web_render_component!("title", [(PARAMS_TYPE, "meet_the_team")], app_state)?.to_rumstring();
///     
///         let text_card_story = rumtk_web_render_component!("text_card", [(PARAMS_TYPE, "story")], app_state)?.to_rumstring();
///         let text_card_coop = rumtk_web_render_component!("text_card", [(PARAMS_TYPE, "coop_values")], app_state)?.to_rumstring();
///     
///         let portrait_card = rumtk_web_render_component!("portrait_card", [("section", "company"), (PARAMS_TYPE, "personnel")], app_state)?.to_rumstring();
///     
///         let spacer_5 = rumtk_web_render_component!("spacer", [("size", "5")], app_state)?.to_rumstring();
///     
///         Ok(vec![
///             text_card_story,
///             spacer_5.clone(),
///             title_coop,
///             text_card_coop,
///             spacer_5,
///             title_team,
///             portrait_card
///         ])
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
///         let custom_css_enabled = rumtk_web_get_config!(state).custom_css;
///
///         rumtk_web_render_template!(MyDiv {
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
///     let app_components = rumtk_web_register_app_components!(
///         vec![("about", about)],
///         vec![("my_div", my_div)], //Optional, can be omitted alongside the skip_serve flag
///         vec![("my_form", my_form)], //Optional, can be omitted alongside the skip_serve flag
///         vec![("v2/add", my_api_handler)] //Optional, can be omitted alongside the skip_serve flag
///     );
///     let app_switches = rumtk_web_register_app_switches!(
///         skip_serve, //Omit in production code. This is used so that this example can work as a unit test.
///         skip_default_css //Omit in production code. This is used so that this example can work as a unit test.
///     );
///     let result = rumtk_web_run_app!(
///         app_components,
///         app_switches
///     );
/// ```
///
#[macro_export]
macro_rules! rumtk_web_run_app {
    (  ) => {{
        use $crate::utils::app::app_main;
        use $crate::{rumtk_web_register_app_components, rumtk_web_register_app_switches};

        app_main(
            rumtk_web_register_app_components!(),
            rumtk_web_register_app_switches!(),
        )
    }};
    ( $app_components:expr ) => {{
        use $crate::rumtk_web_register_app_switches;
        use $crate::utils::app::app_main;

        app_main($app_components, rumtk_web_register_app_switches!())
    }};
    ( $app_components:expr, $switches:expr ) => {{
        use $crate::utils::app::app_main;

        app_main($app_components, $switches)
    }};
}
