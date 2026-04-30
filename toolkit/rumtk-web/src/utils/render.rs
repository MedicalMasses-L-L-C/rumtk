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
use crate::{RUMWebData, RUMWebRedirect, RUMWebTemplate};
use pulldown_cmark::{Options, Parser};
use rumtk_core::core::RUMResult;
use rumtk_core::search::rumtk_search::{string_replace_all_matches, string_search_list};
use rumtk_core::strings::{
    rumtk_format, AsStr, GraphemePattern, GraphemePatternPair, RUMString, RUMStringConversions,
};
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

const TEMPLATE_NEWLINE_COMPONENT_PATTERN: GraphemePatternPair<'static> = (&["<"], &[">"]);
const TEMPLATE_NEWLINE_COMPONENT_INNER_PATTERN: GraphemePatternPair<'static> =
    (&[">", "\n"], &["<"]);
const TEMPLATE_MIDDLE_REGEX: &str = ">\\s+<";
const TEMPLATE_MIDDLE_REPLACEMENT: &str = "><";

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
/// use rumtk_web::testdata::data::{TRIMMED_HTML_RENDER, UNTRIMMED_HTML_RENDER};
///
/// let expected = String::from(TRIMMED_HTML_RENDER);
/// let input = String::from(UNTRIMMED_HTML_RENDER);
/// let filtered = rumtk_web_trim_rendered_html(input).unwrap();
///
/// assert_eq!(filtered, expected, "Template render trim failed!");
/// ```
///
pub fn rumtk_web_trim_rendered_html(html: String) -> RUMResult<String> {
    let filtered = html.as_grapheme_str()
        .trim(&TEMPLATE_NEWLINE_COMPONENT_PATTERN)
        .trim(&TEMPLATE_NEWLINE_COMPONENT_PATTERN)
        .to_string();
    string_replace_all_matches(filtered.as_str(), TEMPLATE_MIDDLE_REGEX, TEMPLATE_MIDDLE_REPLACEMENT)
}

pub fn rumtk_web_post_process(html: String, url: RUMWebRedirect) -> HTMLResult {
    let filtered = rumtk_web_trim_rendered_html(html)?;
    Ok(url.into_web_response(Some(filtered)))
}

///
/// Render the given component template into an `HTML Body response` or a `URL Redirect response`.
/// If you provide the [RUMWebRedirect] in the `url` parameter configured for redirection, then we
/// return the redirection as the response. Otherwise, we render the HTML and save it in the response.
///
/// ## Example
/// ```
/// use rumtk_web::{HTMLBody, RUMString, RUMWebRedirect, RUMWebResponse};
/// use rumtk_web::RUMWebTemplate;
/// use rumtk_web::rumtk_web_render;
///
/// #[derive(RUMWebTemplate)]
/// #[template(
///     source = "<div></div>",
///     ext = "html"
/// )]
/// struct Div { }
///
/// let result = rumtk_web_render(Div{}, RUMWebRedirect::None).unwrap();
/// let expected = RUMWebResponse::into_get_response("<div></div>");
///
/// assert_eq!(result, expected, "Test Div template rendered improperly!");
/// ```
///
pub fn rumtk_web_render<T: RUMWebTemplate>(template: T, url: RUMWebRedirect) -> HTMLResult {
    let result = template.render();
    match result {
        Ok(html) => {
            rumtk_web_post_process(html, url)
        }
        Err(e) => {
            let tn = std::any::type_name::<T>();
            Err(rumtk_format!("Template {tn} render failed: {e:?}"))
        }
    }
}

pub fn rumtk_web_render_contents(elements: &[RUMString]) -> HTMLResult {
    rumtk_web_render(ContentBlock { elements }, RUMWebRedirect::None)
}

pub fn rumtk_web_redirect(url: RUMWebRedirect) -> HTMLResult {
    Ok(url.into_web_response(Some(String::default())))
}

///
/// Render component into an HTML Response Body of type [HTMLResult]. This macro is a bit more complex.
/// Depending on the arguments passed to it, it can
///
/// 1. Call a component function that receives exactly 0 parameters.
/// 2. Call a component function that only receives the [SharedAppState](crate::utils::SharedAppState) handle as its only parameter.
/// 3. Call a component function that can accept the standard set of parameters (`path`, `params`, and `app_state`). However, the Path is set to empty.
/// 4. Call a component function that can accept the standard set of parameters (`path`, `params`, and `app_state`). All of these parameters are passed through to the function.
///
/// The reason for this set of behaviors is that we have standard component functions which are found in [components](crate::components) modules.
/// These functions are of type [ComponentFunction](crate::utils::ComponentFunction) and the expected parameters are as follows:
///
/// 1. `path` => [URLPath](crate::utils::URLPath)
/// 2. `params` => [URLParams](crate::utils::URLParams)
/// 3. `app_state` => [SharedAppState](crate::utils::SharedAppState)
///
/// The component functions are the bread and butter of the framework and are what are expected from consumers of
/// this library. They get registered to an internal `Map` that we use as a sort of `vTable` to dispatch the correct user function.
/// **In this case, the component function parameter for this macro is a stringview type since we perform the lookup automatically!**
///
/// The reason for the other usages is that we also have static components whose only purpose are to define
/// pre-selected items to help make web apps come together in an easy to use package. These include the
/// `htmx` and `fontawesome` imports. Perhaps, we will open up this facility to the user in later iterations of the framework
/// to make it easy to override and include other static assets and maybe for prefetch and optimization purposes.
///
/// ## Examples
///
/// ### Simple Component Render
/// ```
/// use rumtk_web::static_components::css::css;
/// use rumtk_web::rumtk_web_render_component;
///
/// let rendered = rumtk_web_render_component!(css);
/// let expected = "<link rel='stylesheet' href='/static/css/bundle.min.css' onerror='this.onerror=null;this.href='/static/css/bundle.css';' />";
///
/// assert_eq!(rendered, expected, "Commponent rendered improperly!");
/// ```
///
/// ### Component Render with Shared State
/// ```
/// use rumtk_web::SharedAppState;
/// use rumtk_web::static_components::meta::meta;
/// use rumtk_web::utils::testdata::TRIMMED_HTML_RENDER_META;
/// use rumtk_web::rumtk_web_render_component;
///
/// let state = SharedAppState::default();
/// let rendered = rumtk_web_render_component!(meta, state);
///
/// assert_eq!(rendered, TRIMMED_HTML_RENDER_META, "Commponent rendered improperly!");
/// ```
///
/// ### Component Render with Standard Parameters
/// ```
/// use rumtk_web::SharedAppState;
/// use rumtk_web::defaults::PARAMS_TITLE;
/// use rumtk_web::utils::testdata::TRIMMED_HTML_TITLE_RENDER;
/// use rumtk_web::{rumtk_web_render_component, rumtk_web_init_components};
///
/// rumtk_web_init_components!(None);
/// let params = [
///     (PARAMS_TITLE, "Hello World!")
/// ];
/// let state = SharedAppState::default();
/// let rendered = rumtk_web_render_component!("title", params, state).unwrap().to_rumstring();
///
/// assert_eq!(rendered, TRIMMED_HTML_TITLE_RENDER, "Commponent rendered improperly!");
/// ```
///
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
        use $crate::components::div::div;
        use $crate::{rumtk_web_get_component, rumtk_web_params_map};

        let params = rumtk_web_params_map!(&$params);

        match rumtk_web_get_component!($component) {
            Some(component) => component($path, params.get_inner(), $app_state.clone()),
                // This is tricky, but I could not decide if the correct option here was to pass an
                // message or default to a blank div. I chose the div, but if something changes, feel
                // free to reconsider.
            None => div($path, params.get_inner(), $app_state.clone())
        }
    }};
}

#[macro_export]
macro_rules! rumtk_web_render_template {
    ( $page:expr ) => {{
        use $crate::utils::{rumtk_web_render, RUMWebRedirect};

        rumtk_web_render($page, RUMWebRedirect::None)
    }};
    ( $page:expr, $redirect_url:expr ) => {{
        use $crate::utils::rumtk_web_render;

        rumtk_web_render($page, $redirect_url)
    }};
}

#[macro_export]
macro_rules! rumtk_web_post_process_html {
    ( $html:expr ) => {{
        use rumtk_core::strings::{RUMStringConversions};
        use $crate::utils::{rumtk_web_post_process, RUMWebRedirect};

        rumtk_web_post_process($html.to_string(), RUMWebRedirect::None)
    }};
    ( $html:expr, $redirect_url:expr ) => {{
        use rumtk_core::strings::{RUMStringConversions};
        use $crate::utils::rumtk_web_post_process;

        rumtk_web_post_process($html.to_string(), $redirect_url)
    }};
}

///
/// Generates the HTML page as prescribed by the input `page` function of type [HTMLResult].
///
/// ## Example
/// ```
/// use rumtk_core::strings::RUMString;
/// use rumtk_web::defaults::{PARAMS_TYPE};
/// use rumtk_web::pages::index::index;
/// use rumtk_web::{rumtk_web_render_component, rumtk_web_render_page_contents, SharedAppState};
///
/// let app_state = SharedAppState::default();
/// let mydiv = rumtk_web_render_component!("div", [(PARAMS_TYPE, "story")], app_state).unwrap().to_rumstring();
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
