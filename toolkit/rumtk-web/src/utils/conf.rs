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
use crate::jobs::{Job, JobID};
use crate::utils::defaults::DEFAULT_TEXT_ITEM;
use crate::utils::types::RUMString;
use axum::extract::State;
use phf::OrderedMap;
pub use phf_macros::phf_ordered_map as rumtk_create_const_ordered_map;
use rumtk_core::rumtk_generate_id;
use rumtk_core::strings::RUMStringConversions;
use rumtk_core::types::{RUMDeserialize, RUMDeserializer, RUMSerialize, RUMSerializer, RUMID};
use rumtk_core::types::{RUMHashMap, RUMOrderedMap};
use std::sync::{Arc, RwLock};

pub type TextMap = RUMOrderedMap<RUMString, RUMString>;
pub type NestedTextMap = RUMOrderedMap<RUMString, TextMap>;
pub type NestedNestedTextMap = RUMOrderedMap<RUMString, NestedTextMap>;
pub type RootNestedNestedTextMap = RUMOrderedMap<RUMString, NestedNestedTextMap>;

pub type ConstTextMap = OrderedMap<&'static str, &'static str>;
pub type ConstNestedTextMap = OrderedMap<&'static str, &'static ConstTextMap>;
pub type ConstNestedNestedTextMap = OrderedMap<&'static str, &'static ConstNestedTextMap>;

#[derive(RUMSerialize, RUMDeserialize, PartialEq, Debug, Clone, Default)]
pub struct HeaderConf {
    pub logo_size: RUMString,
    pub disable_navlinks: bool,
    pub disable_logo: bool,
}

#[derive(RUMSerialize, RUMDeserialize, PartialEq, Debug, Clone, Default)]
pub struct FooterConf {
    pub socials_list: RUMString,
    pub disable_contact_button: bool,
}

///
/// This is a core structure in a web project using the RUMTK framework. This structure contains
/// a series of fields that represent the web app initial state or configuration. The idea is that
/// the web app can come bundled with a JSON config file following this structure which we can load
/// at runtime. The settings will dictate a few key project behaviors such as properly labeling
/// some components with the company name or use the correct language text.
///
#[derive(RUMSerialize, RUMDeserialize, PartialEq, Debug, Clone, Default)]
pub struct AppConf {
    pub title: RUMString,
    pub description: RUMString,
    pub company: RUMString,
    pub copyright: RUMString,
    pub lang: RUMString,
    pub theme: RUMString,
    pub custom_css: bool,
    pub header_conf: HeaderConf,
    pub footer_conf: FooterConf,

    strings: RootNestedNestedTextMap,
    config: NestedNestedTextMap,
    //pub opts: TextMap,
}

impl AppConf {
    pub fn update_site_info(
        &mut self,
        title: RUMString,
        description: RUMString,
        company: RUMString,
        copyright: RUMString,
    ) {
        if !title.is_empty() {
            self.title = title;
        }
        if !company.is_empty() {
            self.company = company;
        }
        if !description.is_empty() {
            self.description = description;
        }
        if !copyright.is_empty() {
            self.copyright = copyright;
        }
    }

    pub fn get_text(&self, item: &str) -> NestedTextMap {
        match self.strings.get(&self.lang) {
            Some(l) => match l.get(item) {
                Some(i) => i.clone(),
                None => NestedTextMap::default(),
            },
            None => NestedTextMap::default(),
        }
    }

    pub fn get_conf(&self, section: &str) -> TextMap {
        match self.config.get(&self.lang) {
            Some(l) => match l.get(section) {
                Some(i) => i.clone(),
                None => match l.get(DEFAULT_TEXT_ITEM) {
                    Some(i) => i.clone(),
                    None => TextMap::default(),
                },
            },
            None => TextMap::default(),
        }
    }
}

pub type ClipboardID = RUMString;
///
/// Main internal structure for holding the initial app configuration ([AppConf](crate::utils::AppConf)),
/// the `clipboard` containing dynamically generated state ([NestedTextMap](crate::utils::NestedTextMap)),
/// and the `jobs` field containing
///
pub struct AppState {
    config: AppConf,
    clipboard: NestedTextMap,
    jobs: RUMHashMap<RUMID, Job>,
}

pub type SafeAppState = Arc<RwLock<AppState>>;

impl AppState {
    pub fn new() -> AppState {
        AppState {
            config: AppConf::default(),
            clipboard: NestedTextMap::default(),
            jobs: RUMHashMap::default(),
        }
    }

    pub fn new_safe() -> SafeAppState {
        Arc::new(RwLock::new(AppState::new()))
    }

    pub fn from_safe(conf: AppConf) -> SafeAppState {
        Arc::new(RwLock::new(AppState::from(conf)))
    }

    pub fn get_config(&self) -> &AppConf {
        &self.config
    }

    pub fn get_config_mut(&mut self) -> &mut AppConf {
        &mut self.config
    }

    pub fn has_clipboard(&self, id: &ClipboardID) -> bool {
        self.clipboard.contains_key(id)
    }

    pub fn has_job(&self, id: &JobID) -> bool {
        self.jobs.contains_key(id)
    }

    pub fn push_job_result(&mut self, id: &JobID, job: Job) {
        self.jobs.insert(id.clone(), job);
    }

    pub fn push_to_clipboard(&mut self, data: TextMap) -> ClipboardID {
        let clipboard_id = rumtk_generate_id!().to_rumstring();
        self.clipboard.insert(clipboard_id.clone(), data);
        clipboard_id
    }

    pub fn request_clipboard_slice(&mut self) -> ClipboardID {
        let clipboard_id = rumtk_generate_id!().to_rumstring();
        self.clipboard
            .insert(clipboard_id.clone(), TextMap::default());
        clipboard_id
    }

    pub fn pop_job(&mut self, id: &RUMID) -> Option<Job> {
        self.jobs.remove(id)
    }

    pub fn pop_clipboard(&mut self, id: &ClipboardID) -> Option<TextMap> {
        self.clipboard.shift_remove(id)
    }
}

impl From<AppConf> for AppState {
    fn from(config: AppConf) -> Self {
        AppState {
            config,
            clipboard: NestedTextMap::default(),
            jobs: RUMHashMap::default(),
        }
    }
}

pub type SharedAppState = Arc<RwLock<AppState>>;
pub type RouterAppState = State<Arc<RwLock<AppState>>>;

#[macro_export]
macro_rules! rumtk_web_load_conf {
    ( $args:expr ) => {{
        rumtk_web_load_conf!($args, "./app.json")
    }};
    ( $args:expr, $path:expr ) => {{
        use rumtk_core::rumtk_deserialize;
        use rumtk_core::strings::RUMStringConversions;
        use rumtk_core::types::RUMHashMap;
        use std::fs;

        use $crate::rumtk_web_save_conf;
        use $crate::utils::{AppConf, AppState, TextMap};

        let json = match fs::read_to_string($path) {
            Ok(json) => json,
            Err(err) => rumtk_web_save_conf!($path),
        };

        let mut conf: AppConf = match rumtk_deserialize!(json) {
            Ok(conf) => conf,
            Err(err) => panic!(
                "The App config file in {} does not meet the expected structure. \
                    See the documentation for more information. Error: {}\n{}",
                $path, err, json
            ),
        };
        conf.update_site_info(
            $args.title.clone(),
            $args.description.clone(),
            $args.company.clone(),
            $args.copyright.clone(),
        );
        AppState::from_safe(conf)
    }};
}

#[macro_export]
macro_rules! rumtk_web_save_conf {
    ( $path:expr ) => {{
        use rumtk_core::rumtk_serialize;
        use rumtk_core::strings::RUMStringConversions;
        use std::fs;
        use $crate::utils::AppConf;

        let json = rumtk_serialize!(AppConf::default(), true)?;
        fs::write($path, &json);
        json
    }};
}

#[macro_export]
macro_rules! rumtk_web_get_string {
    ( $conf:expr, $item:expr ) => {{
        let owned_state = $conf.read().expect("Lock failure");
        owned_state.get_config().get_text($item)
    }};
}

#[macro_export]
macro_rules! rumtk_web_get_conf {
    ( $conf:expr, $item:expr ) => {{
        let owned_state = $conf.read().expect("Lock failure");
        owned_state.get_config().get_conf($item)
    }};
}

/*
   Default non static data to minimize allocations.
*/
pub const DEFAULT_TEXT: fn() -> RUMString = || RUMString::default();
pub const DEFAULT_TEXTMAP: fn() -> TextMap = || TextMap::default();
pub const DEFAULT_NESTEDTEXTMAP: fn() -> NestedTextMap = || NestedTextMap::default();
pub const DEFAULT_NESTEDNESTEDTEXTMAP: fn() -> NestedNestedTextMap =
    || NestedNestedTextMap::default();
