/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D.
 * Copyright (C) 2025  Nick Stephenson
 * Copyright (C) 2025  Ethan Dixon
 * Copyright (C) 2025  MedicalMasses L.L.C.
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
use crate::utils::packaging::{minify_asset, Asset};
use rumtk_core::hash::has_same_hash;
use rumtk_core::strings::{RUMString, RUMStringConversions};
use std::{fs, path};

mod animations;
mod basic;
mod components;
mod default;
mod fonts;
mod forms;
mod gap;
mod index;
mod layout;
mod theme;

pub const DEFAULT_OUT_CSS_DIR: &str = "./static/css";
pub const DEFAULT_OUT_CSS: &str = "bundle.min.css";

pub fn bundle_css(sources: &Vec<String>, out_dir: &str, out_file: &str) {
    let mut css: RUMString = RUMString::default();

    css += theme::THEME;
    css += index::BODY;
    css += basic::BASIC_CSS;
    css += default::DEFAULT_CSS;
    css += fonts::FONTS_CSS;
    css += gap::GAP_CSS;
    css += animations::ANIMATIONS_CSS;
    css += forms::FORM_CSS;
    css += components::LIST_CSS;
    css += layout::LAYOUT_CSS;

    for source in sources {
        let css_data = fs::read_to_string(source).unwrap_or_default();
        css += &css_data;
    }

    fs::create_dir_all(out_dir).unwrap_or_default();

    let out_path = path::Path::new(out_dir)
        .join(out_file)
        .with_extension("css")
        .to_str()
        .expect("Could not create path to CSS file!")
        .to_string();

    let minified = minify_asset(Asset::CSS(&css))
        .expect("Failed to minify the CSS contents!")
        .to_rumstring();

    let file_exists = fs::exists(&out_path).unwrap_or_default();
    let skip_write_css = file_exists
        && has_same_hash(
            &minified,
            &fs::read_to_string(&out_path)
                .unwrap_or_default()
                .to_rumstring(),
        );

    if !skip_write_css {
        println!("Generated minified CSS file!");
        fs::write(&out_path, minified).expect("Failed to write to CSS file!");
    }
}

pub fn collect_css_sources(root: &str, depth: u8) -> Vec<String> {
    let mut files = Vec::<String>::new();

    let dirs = match fs::read_dir(root) {
        Ok(dirs) => dirs,
        Err(_) => return files,
    };

    for dir_entry in dirs {
        let dir = dir_entry.unwrap();
        let dir_name = dir.file_name().into_string().unwrap();
        let dir_path = dir.path().to_str().unwrap().to_string();
        if dir_name.ends_with(".css") && dir_name != DEFAULT_OUT_CSS {
            files.push(dir_path.clone());
        }

        if depth == 255 {
            return files;
        }

        if dir.file_type().unwrap().is_dir() {
            files.extend(collect_css_sources(&dir_path, depth + 1));
        }
    }

    files
}

#[macro_export]
macro_rules! rumtk_web_compile_css_bundle {
    (  ) => {{
        use $crate::css::{bundle_css, collect_css_sources};
        use $crate::css::{DEFAULT_OUT_CSS, DEFAULT_OUT_CSS_DIR};
        let sources = collect_css_sources(DEFAULT_OUT_CSS_DIR, 0);
        bundle_css(&sources, DEFAULT_OUT_CSS_DIR, DEFAULT_OUT_CSS);
    }};
    ( $static_dir_path:expr ) => {{
        use $crate::css::{bundle_css, collect_css_sources};
        use $crate::css::{DEFAULT_OUT_CSS, DEFAULT_OUT_CSS_DIR};
        let sources = collect_css_sources($static_dir_path, 0);
        bundle_css(&sources, DEFAULT_OUT_CSS_DIR, DEFAULT_OUT_CSS);
    }};
}
