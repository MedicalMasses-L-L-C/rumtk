use axum::extract::State;
use axum::extract::{Path, Query};
use axum::response::Html;
use phf::{Map, OrderedMap};
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

pub type MMString = String;
pub type URLPath<'a, 'b> = &'a [&'b str];
pub type AsyncURLPath = Arc<Vec<MMString>>;
pub type URLParams<'a> = &'a HashMap<MMString, MMString>;
pub type AsyncURLParams = Arc<HashMap<MMString, MMString>>;
pub type HTMLResult = Result<Html<MMString>, MMString>;
pub type RenderedPageComponents = Vec<MMString>;
/* Router Match Types */
pub type RouterComponents = Path<Vec<MMString>>;
pub type RouterParams = Query<HashMap<MMString, MMString>>;

pub struct AppState {
    pub title: MMString,
    pub description: MMString,
    pub lang: MMString,
    pub theme: MMString,
    pub custom_css: bool,
    //pub opts: TextMap,
}

impl AppState {
    pub fn default() -> Self {
        Self::new(
            MMString::from(""),
            MMString::from(""),
            MMString::from("en"),
            MMString::from(""),
            false,
        )
    }

    pub fn default_site(title: MMString, description: MMString, custom_css: bool) -> Self {
        Self::new(
            title,
            description,
            MMString::from("en"),
            MMString::from(""),
            custom_css,
        )
    }

    pub fn new(
        title: MMString,
        description: MMString,
        lang: MMString,
        theme: MMString,
        custom_css: bool,
    ) -> Self {
        AppState {
            title,
            description,
            lang,
            theme,
            custom_css,
        }
    }
}

pub type SharedAppState = Arc<Mutex<AppState>>;
pub type RouterAppState = State<Arc<Mutex<AppState>>>;

/* Config Types */
pub type ComponentFunction = fn(URLPath, URLParams, SharedAppState) -> HTMLResult;
pub type PageFunction = fn(SharedAppState) -> RenderedPageComponents;
pub type AsyncReturn = Arc<Pin<Box<dyn Future<Output = HTMLResult>>>>;
pub type AsyncComponentFunction = fn(AsyncURLPath, AsyncURLParams, SharedAppState) -> AsyncReturn;
pub type ComponentMap = Map<&'static str, ComponentFunction>;
pub type PageMap = Map<&'static str, PageFunction>;
pub type TextMap = OrderedMap<&'static str, &'static str>;
pub type NestedTextMap = OrderedMap<&'static str, &'static TextMap>;
pub type NestedNestedTextMap = OrderedMap<&'static str, &'static NestedTextMap>;
