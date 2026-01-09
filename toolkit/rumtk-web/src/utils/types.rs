use std::collections::HashMap;
use std::pin::Pin;
use axum::extract::State;
use std::sync::{Arc, Mutex};
use axum::extract::{Path, Query};
use axum::response::Html;
use phf::{Map, OrderedMap};

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
    pub lang: MMString,
    pub theme: MMString,
    //pub opts: TextMap,
}

impl AppState {
    pub fn default() -> AppState {
        AppState{
            lang: MMString::from("en"),
            theme: MMString::from(""),
            //opts: phf_ordered_map!(),
        }
    }
}

pub type SharedAppState = Arc<Mutex<AppState>>;
pub type RouterAppState = State<Arc<Mutex<AppState>>>;

/* Config Types */
pub type ComponentFunction = fn(URLPath, URLParams, SharedAppState) -> HTMLResult;
pub type PageFunction = fn(SharedAppState) -> RenderedPageComponents;
pub type AsyncReturn = Arc<Pin<Box<dyn Future<Output=HTMLResult>>>>;
pub type AsyncComponentFunction = fn(AsyncURLPath, AsyncURLParams, SharedAppState) -> AsyncReturn;
pub type ComponentMap = Map<&'static str, ComponentFunction>;
pub type PageMap = Map<&'static str, PageFunction>;
pub type TextMap = OrderedMap<&'static str, &'static str>;
pub type NestedTextMap = OrderedMap<&'static str, &'static TextMap>;
pub type NestedNestedTextMap = OrderedMap<&'static str, &'static NestedTextMap>;