mod pages;

use rumtk_web::rumtk_web_run_app;

fn main() {
    println!("Hello, world!");
    rumtk_web_run_app!(vec![("index", pages::index::index)])
}
