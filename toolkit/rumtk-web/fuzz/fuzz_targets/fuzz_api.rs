#![no_main]

use libfuzzer_sys::fuzz_target;
use rumtk_core::buffers::buffer_to_str;

use rumtk_core::strings::rumtk_format;
use rumtk_web::*;
use std::thread::spawn;

fuzz_target!(|data: &[u8]| {
    let _ = spawn(|| {
        rumtk_web_run_app!()
    });

    // fuzzed code goes here
    if let Ok(str) = buffer_to_str(data) {
        rumtk_web_sync_get(&rumtk_format!("127.0.0.1:3000/{str}")).unwrap();
    }
});
