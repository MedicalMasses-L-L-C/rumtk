mod forms;
mod pages;

use rumtk_web::rumtk_web_run_app;

fn main() {
    rumtk_web_run_app!(
        vec![("index", pages::index::index),],
        vec![],
        vec![("upload", forms::upload::upload_form),]
    )
}
