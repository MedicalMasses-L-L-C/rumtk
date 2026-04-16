/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D. <lsantos@medicalmasses.com>
 * Copyright (C) 2025  Ethan Dixon
 * Copyright (C) 2025  MedicalMasses L.L.C. <contact@medicalmasses.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::types::HTMLResult;
use crate::{RUMWebRedirect, RUMWebTemplate};
use pulldown_cmark::{Options, Parser};
use rumtk_core::strings::{rumtk_format, RUMString, RUMStringConversions};
use std::sync::OnceLock;

pub static MARKDOWN_OPTIONS: OnceLock<Options> = OnceLock::new();

pub static MARKDOWN_OPTIONS_INIT: fn() -> Options = || -> Options {
    let mut options = Options::empty();

    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_MATH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_WIKILINKS);

    options
};

const TEMPLATE_NEWLINE_PATTERN: &str = "\n   ";
const TEMPLATE_NEWLINE_REPLACEMENT: &str = "    ";

#[derive(RUMWebTemplate)]
#[template(
    source = "
        {% for element in elements %}
           {{ element|safe }}
        {% endfor %}
    ",
    ext = "html"
)]
struct ContentBlock<'a> {
    elements: &'a [RUMString],
}

///
/// This function trims excess newlines and whitespacing outside tag block (e.g. `<div></div>`). The
/// idea is to cleanup the rendered template which picks up extra characters due to the way string
/// literals work in proc macros.
///
/// This is not meant to be used as a sanitization function!
///
/// This function consumes the input string!!!!!
///
/// ## Example
/// ```
/// use rumtk_web::rumtk_web_trim_rendered_html;
///
/// let expected = String::from("<div class='div-default'>default</div>");
/// let input = String::from("\n        \n           \n        \n        <div class='div-default'>default</div>\n    \n        \n    ");
/// let filtered = rumtk_web_trim_rendered_html(input);
///
/// assert_eq!(filtered, expected, "Template render trim failed!");
/// ```
///
pub fn rumtk_web_trim_rendered_html(html: String) -> String {
    let filtered = html
        .as_str()
        .replace(TEMPLATE_NEWLINE_PATTERN, TEMPLATE_NEWLINE_REPLACEMENT);
    filtered.as_str().trim().to_string()
}

pub fn rumtk_web_render_html<T: RUMWebTemplate>(template: T, url: RUMWebRedirect) -> HTMLResult {
    let result = template.render();
    match result {
        Ok(html) => {
            let filtered = rumtk_web_trim_rendered_html(html);
            Ok(url.into_web_response(Some(filtered)))
        }
        Err(e) => {
            let tn = std::any::type_name::<T>();
            Err(rumtk_format!("Template {tn} render failed: {e:?}"))
        }
    }
}

pub fn rumtk_web_render_contents(elements: &[RUMString]) -> HTMLResult {
    rumtk_web_render_html(ContentBlock { elements }, RUMWebRedirect::None)
}

pub fn rumtk_web_redirect(url: RUMWebRedirect) -> HTMLResult {
    Ok(url.into_web_response(Some(String::default())))
}

#[macro_export]
macro_rules! rumtk_web_render_component {
    ( $component_fxn:expr ) => {{
        use rumtk_core::strings::{RUMString, RUMStringConversions};
        match $component_fxn() {
            Ok(x) => x.to_rumstring(),
            _ => RUMString::default(),
        }
    }};
    ( $component_fxn:expr, $app_state:expr ) => {{
        use rumtk_core::strings::{RUMString, RUMStringConversions};
        match $component_fxn($app_state.clone()) {
            Ok(x) => x.to_rumstring(),
            _ => RUMString::default(),
        }
    }};
    ( $component:expr, $params:expr, $app_state:expr ) => {{
        rumtk_web_render_component!($component, &[""], $params, $app_state)
    }};
    ( $component:expr, $path:expr, $params:expr, $app_state:expr ) => {{
        use rumtk_core::strings::{RUMString, RUMStringConversions};
        use $crate::{rumtk_web_get_component, rumtk_web_params_map};

        let component = rumtk_web_get_component!($component);

        match component($path, &rumtk_web_params_map!($params), $app_state.clone()) {
            Ok(x) => x.to_rumstring(),
            _ => RUMString::default(),
        }
    }};
}

#[macro_export]
macro_rules! rumtk_web_render_html {
    ( $page:expr ) => {{
        use $crate::utils::{rumtk_web_render_html, RUMWebRedirect};

        rumtk_web_render_html($page, RUMWebRedirect::None)
    }};
    ( $page:expr, $redirect_url:expr ) => {{
        use $crate::utils::rumtk_web_render_html;

        rumtk_web_render_html($page, $redirect_url)
    }};
}

///
/// Generates the HTML page as prescribed by the input `page` function of type [HTMLResult].
///
/// ## Example
/// ```
/// use rumtk_core::strings::RUMString;
/// use rumtk_web::pages::index::index;
/// use rumtk_web::{rumtk_web_render_component, rumtk_web_render_page_contents, SharedAppState};
///
/// let app_state = SharedAppState::default();
/// let mydiv = rumtk_web_render_component!("div", [("type", "story")], app_state.clone());
///
/// let expected_page = RUMString::new("<div class='div-default'>default</div>");
/// let page_response = rumtk_web_render_page_contents!(
///     &vec![
///         mydiv
///     ]
/// ).expect("Page rendered!");
/// let rendered_page = page_response.to_rumstring();
///
/// assert_eq!(rendered_page, expected_page, "Page was not rendered properly!")
/// ```
///
#[macro_export]
macro_rules! rumtk_web_render_page_contents {
    ( $page_elements:expr ) => {{
        use $crate::utils::rumtk_web_render_contents;

        rumtk_web_render_contents($page_elements)
    }};
}

///
/// Generate redirect response automatically instead of actually rendering an HTML page.
///
/// ## Examples
///
/// ### Temporary Redirect
/// ```
/// use rumtk_web::RUMStringConversions;
/// use rumtk_web::utils::response::RUMWebRedirect;
/// use rumtk_web::rumtk_web_render_redirect;
///
/// let url = "http://localhost/redirected";
/// let redirect = rumtk_web_render_redirect!(RUMWebRedirect::RedirectTemporary(url.to_rumstring()));
///
/// let result = redirect.expect("Failed to create the redirect response!").get_url();
///
/// assert_eq!(result, url, "Url in Response object does not match the expected!");
///
/// ```
///
#[macro_export]
macro_rules! rumtk_web_render_redirect {
    ( $url:expr ) => {{
        use $crate::utils::rumtk_web_redirect;

        rumtk_web_redirect($url)
    }};
}

///
///
/// If using raw strings, do not leave an extra line. The first input must have characters, or you
/// will get <pre><code> blocks regardless of what you do.
///
/// ## Example
/// ```
/// use rumtk_web::rumtk_web_render_markdown;
///
/// let md = r###"
///**Hello World**
/// "###;
/// let expected_html = "<p><strong>Hello World</strong></p>\n";
///
/// let result = rumtk_web_render_markdown!(md);
///
/// assert_eq!(result, expected_html, "The rendered markdown does not match the expected HTML!");
/// ```
///
#[macro_export]
macro_rules! rumtk_web_render_markdown {
    ( $md:expr ) => {{
        use pulldown_cmark::{Options, Parser};
        use rumtk_core::strings::RUMStringConversions;
        use $crate::utils::render::{MARKDOWN_OPTIONS, MARKDOWN_OPTIONS_INIT};

        let mut options = MARKDOWN_OPTIONS.get_or_init(MARKDOWN_OPTIONS_INIT);

        let input = String::from($md);
        let parser = Parser::new_ext(&input, *options);
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);

        html_output.to_rumstring()
    }};
}
