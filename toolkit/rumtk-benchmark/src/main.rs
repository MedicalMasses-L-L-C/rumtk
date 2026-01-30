mod forms;
mod pages;

use rumtk_web::rumtk_web_run_app;

fn main() {
    rumtk_web_run_app!(
        vec![
            ("index", pages::index::index),
            ("other", pages::other::other),
        ],
        vec![],
        vec![("upload", forms::upload::upload_form),]
    )
}
