mod forms;
mod pages;

use rumtk_web::{
    rumtk_web_register_app_components, rumtk_web_register_app_switches, rumtk_web_run_app,
    AppComponents,
};

fn main() {
    let app_components = rumtk_web_register_app_components!(
        vec![
            ("index", pages::index::index),
            ("other", pages::other::other),
        ],
        vec![],
        vec![("basic_benchmark", forms::basic_benchmark::basic_benchmark),]
    );

    rumtk_web_run_app!(app_components);
}
