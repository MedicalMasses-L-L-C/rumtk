#![feature(random)]
#![feature(thread_id_value)]
#![no_main]

use libfuzzer_sys::fuzz_target;
use rumtk_core::buffers::buffer_to_str;
use rumtk_core::rumtk_sleep;
use rumtk_core::strings::rumtk_format;
use rumtk_web::components::title::title;
use rumtk_web::*;
use std::random::random;
use std::sync::LazyLock;
use std::thread::{spawn, JoinHandle, ThreadId};

const EXPECTED_RESPONSE_SIZE: usize = 2012;
static PORT: LazyLock<u16> = LazyLock::new(|| random::<u16>(..));
static ADDRESS: LazyLock<RUMString> = LazyLock::new(|| {
    let port = *PORT;
    rumtk_format!("http://127.0.0.1:{port}")
});
static APP: LazyLock<JoinHandle<()>> = LazyLock::new(|| spawn(|| {
    let app_components = rumtk_web_register_app_components!(
                vec![
                    ("index", index),
                ]
    );
    let app_switches = rumtk_web_register_app_switches!(
                false,
                true,
                true
            );
    rumtk_web_run_app!(
                app_components,
                app_switches,
                Some(*PORT)
            );
}));
static mut APP_ID: Option<ThreadId> = None;

pub fn index(app_state: SharedAppState) -> RenderedPageComponentsResult {
    let title_params = rumtk_web_params_map!([("title", "Hello World!")]);
    let title = title(&[], title_params.get_inner(), app_state.clone())?.to_string();
    Ok(vec![title])
}

fuzz_target!(|data: &[u8]| {
    let app_id = unsafe {APP_ID}.unwrap_or_else(|| {
            let id = (*APP).thread().id();
            unsafe {
                APP_ID = Some(id);
            }
            rumtk_sleep!(5);
            id
        }).as_u64().to_string();
    // fuzzed code goes here
    if let Ok(str) = buffer_to_str(data) {
        let addr = (*ADDRESS).clone();
        let request_url = rumtk_format!("{addr}/{str}");
        println!("Request[{request_url}]");
        let (code, output) = rumtk_web_sync_get(&request_url).unwrap();
        println!("Request[{request_url} => {code}] responded with: {} bytes", output.len());
        if code == 200 {
            let output_str = buffer_to_str(&output).unwrap();
            assert_eq!(output.len(), EXPECTED_RESPONSE_SIZE,"Applet[{app_id}] responded improperly with wrong response size!");
            assert!(output_str.contains(">Hello World!</h1>"), "Applet[{app_id}] responded improperly with wrong response content!");
        } 
    }
});
