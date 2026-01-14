/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D.
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
use rumtk_core::strings::RUMString;
use rumtk_core::threading::threading_functions::get_default_system_thread_count;

use crate::css::DEFAULT_OUT_CSS_DIR;
use crate::utils::defaults::DEFAULT_LOCAL_LISTENING_ADDRESS;
use crate::utils::matcher::*;
use crate::{rumtk_web_fetch, rumtk_web_load_conf};

use crate::pages::UserPages;
use axum::routing::get;
use axum::Router;
use clap::Parser;
use tower_http::compression::{CompressionLayer, DefaultPredicate};
use tower_http::services::ServeDir;
use tracing::error;

///
/// RUMTK WebApp CLI Args
///
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    ///
    /// Website title to use internally. It can be omitted if defined in the app.json config file
    /// bundled with your app.
    ///
    #[arg(short, long, default_value = "")]
    title: RUMString,
    ///
    /// Website description string. It can be omitted if defined in the app.json config file
    /// bundled with your app.
    ///
    #[arg(short, long, default_value = "")]
    description: RUMString,
    ///
    /// Copyright year to display in website.
    ///
    #[arg(short, long, default_value = "")]
    copyright: RUMString,
    ///
    /// Directory to scan on startup to find custom CSS sources to bundle into a minified CSS file
    /// that can be quickly pulled by the app client side.
    ///
    /// This option can provide an alternative to direct component retrieval of CSS fragments.
    /// Meaning, you could bundle all of your fragments into the master bundle at startup and
    /// turn off component level ```custom_css_enabled``` option in the ```app.json``` config.
    ///
    #[arg(short, long, default_value = DEFAULT_OUT_CSS_DIR)]
    css_source_dir: RUMString,
    ///
    /// Is the interface meant to be bound to the loopback address and remain hidden from the
    /// outside world.
    ///
    /// It follows the format ```IPv4:port``` and it is a string.
    ///
    /// If a NIC IP is defined via `--ip`, that value will override this flag.
    ///
    #[arg(short, long, default_value = DEFAULT_LOCAL_LISTENING_ADDRESS)]
    ip: RUMString,
    ///
    /// How many threads to use to serve the website. By default, we use
    /// ```get_default_system_thread_count()``` from ```rumtk-core``` to detect the total count of
    /// cpus available. We use the system's total count of cpus by default.
    ///
    #[arg(short, long, default_value_t = get_default_system_thread_count())]
    threads: usize,
}

pub async fn run_app(args: &Args) {
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
        /* Services */
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state)
        .layer(comression_layer);

    let listener = tokio::net::TcpListener::bind(&args.ip.as_str())
        .await
        .expect("There was an issue biding the listener.");
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("There was an issue with the server.");
}

///
/// This is the main macro for defining your applet and launching it.
/// Usage is very simple and the only decision from a user is whether to pass a list of
/// [UserPages](crate::pages::UserPages) or a list of [UserPages](crate::pages::UserPages) and a list
/// of [UserComponents](crate::components::UserComponents).
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
#[macro_export]
macro_rules! rumtk_web_run_app {
    ( $pages:expr ) => {{
        use $crate::{
            rumtk_web_compile_css_bundle, rumtk_web_init_components, rumtk_web_init_pages,
        };
        let args = Args::parse();

        rumtk_web_init_components!(vec![]);
        rumtk_web_init_pages!($pages);
        rumtk_web_compile_css_bundle!(&args.css_source_dir);

        run_app(args).await;
    }};
    ( $pages:expr, $components:expr ) => {{
        use $crate::{
            rumtk_web_compile_css_bundle, rumtk_web_init_components, rumtk_web_init_pages,
        };
        let args = Args::parse();

        rumtk_web_init_components!($components);
        rumtk_web_init_pages!($pages);
        rumtk_web_compile_css_bundle!(&args.css_source_dir);

        run_app(args);
    }};
}
