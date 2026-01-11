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
pub mod conf;
pub mod defaults;
pub mod matcher;
pub mod render;
pub mod types;

use axum::response::Html;
use std::collections::HashMap;

use axum::{Router, routing::get};
pub use render::*;
use std::sync::Arc;
use std::sync::Mutex;
use tower_http::compression::{CompressionLayer, DefaultPredicate, Predicate};
use tower_http::services::ServeDir;
pub use types::*;

#[macro_export]
macro_rules! rumtk_web_get_misc_conf {
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
macro_rules! rumtk_web_get_conf {
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
macro_rules! rumtk_web_get_text_item {
    ( $store:expr, $item:expr, $default:expr) => {{
        match $store.get($item) {
            Some(x) => x,
            None => $default,
        }
    }};
}

#[macro_export]
macro_rules! rumtk_web_get_param_eq {
    ( $params:expr, $indx:expr, $comparison:expr, $default:expr ) => {{
        match $params.get($indx) {
            Some(x) => *x == $comparison,
            None => $default,
        }
    }};
}

#[macro_export]
macro_rules! rumtk_web_get_param {
    ( $params:expr, $indx:expr, $default:expr ) => {{
        match $params.get($indx) {
            Some(x) => x.parse().unwrap_or($default),
            None => $default,
        }
    }};
}

#[macro_export]
macro_rules! rumtk_web_params_map {
    ( $params:expr ) => {{
        use std::collections::HashMap;
        let mut params = HashMap::<RUMString, RUMString>::with_capacity($params.len());

        for (k, v) in $params.iter() {
            params.insert(k.to_string(), v.to_string());
        }
        params
    }};
}

#[macro_export]
macro_rules! rumtk_web_collect_page {
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
macro_rules! rumtk_web_fetch {
    ( $matcher:expr ) => {{
        use axum::extract::{Path, Query, State};
        use axum::response::Html;
        use $crate::utils::types::{RUMString, RouterAppConf, RouterComponents, RouterParams};
        async |Path(path_params): RouterComponents,
               Query(params): RouterParams,
               State(state): RouterAppConf|
               -> Html<String> {
            match $matcher(path_params, params, state).await {
                Ok(res) => res,
                Err(e) => {
                    error!("{}", e);
                    return Html(RUMString::default());
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

    let listener = tokio::net::TcpListener::bind(&ip)
        .await
        .expect("There was an issue biding the listener.");
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("There was an issue with the server.");
}

#[macro_export]
macro_rules! rumtk_web_run_app {
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
