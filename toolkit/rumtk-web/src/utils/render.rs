/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2026  Luis M. Santos, M.D.
 * Copyright (C) 2026  MedicalMasses L.L.C.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
 */
use crate::utils::types::HTMLResult;
use axum::response::Html;
use rumtk_core::strings::{rumtk_format, RUMStringConversions};

pub fn rumtk_web_html_render<T: askama::Template>(template: T) -> HTMLResult {
    let result = template.render();
    match result {
        Ok(html) => Ok(Html(html.to_rumstring())),
        Err(e) => {
            let tn = std::any::type_name::<T>();
            Err(rumtk_format!("Template {tn} render failed: {e:?}"))
        }
    }
}

#[macro_export]
macro_rules! rumtk_web_render_component {
    ( $component_fxn:expr ) => {{
        match $component_fxn() {
            Ok(x) => x.0,
            Err(e) => RUMString::default(),
        }
    }};
    ( $component_fxn:expr, $app_state:expr ) => {{
        match $component_fxn($app_state.clone()) {
            Ok(x) => x.0,
            Err(e) => RUMString::default(),
        }
    }};
    ( $component:expr, $params:expr, $app_state:expr, $components:expr ) => {{
        use $crate::components::div::div;
        use $crate::rumtk_web_params_map;
        use $crate::utils::types::ComponentFunction;
        let component = match $components.get($component) {
            Some(x) => x,
            None => &(div as ComponentFunction),
        };

        match component(&[], &rumtk_web_params_map!($params), $app_state.clone()) {
            Ok(x) => x.0,
            _ => RUMString::default(),
        }
    }};
}

#[macro_export]
macro_rules! rumtk_web_render_html {
    ( $component:expr ) => {{
        use crate::utils::{rumtk_web_html_render, types::HTMLResult};

        let closure = || -> HTMLResult { rumtk_web_html_render($component) };

        closure()
    }};
}

///
///
/// If using raw strings, do not leave an extra line. The first input must have characters or you will get <pre><code> blocks regardless of what you do.
///
#[macro_export]
macro_rules! rumtk_web_render_markdown {
    ( $md:expr ) => {{
        use pulldown_cmark::{Options, Parser};
        use $crate::utils::types::RUMString;

        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_MATH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_WIKILINKS);

        let input = RUMString::from($md);
        let parser = Parser::new_ext(&input, options);
        let mut html_output = RUMString::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);
        println!("{}", &html_output);

        html_output
    }};
}
