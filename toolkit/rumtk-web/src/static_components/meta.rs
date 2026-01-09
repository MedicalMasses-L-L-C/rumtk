use crate::mm_render_html;
use crate::utils::types::{HTMLResult, MMString, SharedAppState};
use askama::Template;

#[derive(Template)]
#[template(
    source = "
            <meta charset='UTF-8'>
            <meta http-equiv='Content-Type' content='text/html; charset=utf-8' />
            <meta name='viewport' content='width=device-width, initial-scale=1.0' />
            <meta http-equiv='X-UA-Compatible' content='IE=edge,chrome=1'/>
            <meta name='description' content='{{description}}'>
            <title>{{title}}</title>
            <link rel='icon' type='image/png' href='/static/img/icon.png'>
    ",
    ext = "html"
)]
pub struct Meta {
    title: MMString,
    description: MMString,
}

pub fn meta(state: SharedAppState) -> HTMLResult {
    let owned_state = state.lock().expect("Lock failure");

    mm_render_html!(Meta {
        title: owned_state.title.clone(),
        description: owned_state.description.clone()
    })
}
