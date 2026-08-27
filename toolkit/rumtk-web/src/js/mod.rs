/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2026  Luis M. Santos, M.D. <lsantos@medicalmasses.com>
 * Copyright (C) 2026  MedicalMasses L.L.C. <contact@medicalmasses.com>
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
use rumtk_core::base::RUMResult;
use rumtk_core::strings::{rumtk_format, RUMString};
use std::{fs, path};

mod default_library;

pub const DEFAULT_OUT_JS_DIR: &str = "./static/js";

fn select_from_library(item_name: &str) -> &'static str {
    match item_name {
        "file_cache" => default_library::JS_FILE_CACHE,
        "goto" => default_library::JS_GOTO,
        _ => "",
    }
}

pub fn rumtk_web_js_get_item(item_name: &str) -> RUMResult<RUMString> {
    match fs::create_dir_all(DEFAULT_OUT_JS_DIR) {
        Ok(_) => (),
        Err(e) => return Err(rumtk_format!("Failed to create JS directory: {} => because {}", DEFAULT_OUT_JS_DIR, e)),
    };
    let path = path::Path::new(DEFAULT_OUT_JS_DIR)
        .join(item_name)
        .with_extension("js");
    let out_path = match path
        .to_str()
    {
        Some(path) => path.to_string(),
        None => return Err(rumtk_format!("Could not create path to JS file {}!", item_name)),
    };
    match fs::exists(&out_path) {
        Ok(result) => match result {
            true => Ok(out_path),
            false => match fs::write(&out_path, select_from_library(item_name)) {
                Ok(_) => Ok(out_path),
                Err(e) => Err(rumtk_format!("Failed to write JS file: {} => because {}", out_path, e)),
            },
        },
        Err(e) => Err(rumtk_format!("Failed to generate JS file: {} => because {}", out_path, e)),
    }
}
