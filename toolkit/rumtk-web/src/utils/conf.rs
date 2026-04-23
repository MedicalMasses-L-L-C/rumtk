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
use crate::jobs::{Job, JobID};
use crate::utils::defaults::DEFAULT_TEXT_ITEM;
use crate::utils::types::RUMString;
use askama::PrimitiveType;
use axum::extract::State;
use phf::OrderedMap;
pub use phf_macros::phf_ordered_map as rumtk_create_const_ordered_map;
use rumtk_core::net::tcp::SafeLock;
use rumtk_core::pipelines::pipeline_types::RUMCommandLine;
use rumtk_core::strings::RUMStringConversions;
use rumtk_core::types::{RUMDeserialize, RUMSerialize, RUMID};
use rumtk_core::types::{RUMHashMap, RUMOrderedMap};
use rumtk_core::{rumtk_generate_id, rumtk_new_lock};

pub type TextMap = RUMOrderedMap<RUMString, RUMString>;
pub type NestedTextMap = RUMOrderedMap<RUMString, TextMap>;
pub type NestedNestedTextMap = RUMOrderedMap<RUMString, NestedTextMap>;
pub type RootNestedNestedTextMap = RUMOrderedMap<RUMString, NestedNestedTextMap>;

pub type ConstTextMap = OrderedMap<&'static str, &'static str>;
pub type ConstNestedTextMap = OrderedMap<&'static str, &'static ConstTextMap>;
pub type ConstNestedNestedTextMap = OrderedMap<&'static str, &'static ConstNestedTextMap>;

pub type PipelineGroup = RUMHashMap<RUMString, RUMCommandLine>;

#[derive(RUMSerialize, RUMDeserialize, PartialEq, Debug, Clone, Default)]
pub struct HeaderConf {
    pub logo_source: Option<RUMString>,
    pub logo_size: RUMString,
    pub disable_navlinks: bool,
    pub disable_logo: bool,
}

#[derive(RUMSerialize, RUMDeserialize, PartialEq, Debug, Clone, Default)]
pub struct FooterConf {
    pub socials_list: RUMString,
    pub disable_contact_button: bool,
}

#[derive(RUMSerialize, RUMDeserialize, PartialEq, Debug, Clone, Default)]
pub struct PipelineConf {
    pub setup_settings: TextMap,
    pub categories: Option<RUMHashMap<RUMString, PipelineGroup>>
}

impl PipelineConf {
    pub fn get_pipeline_category(&self, pipeline_category: &str) -> Option<&PipelineGroup> {
        match self.categories {
            Some(ref categories) => {
                match categories.get(pipeline_category) {
                    Some(pipelines) => Some(pipelines),
                    None => None
                }
            }
            None => None,
        }
    }
    pub fn get_pipeline(&self, pipeline_category: &str, pipeline_name: &str) -> RUMCommandLine {
        match self.get_pipeline_category(pipeline_category) {
            Some(group) => match group.get(pipeline_name) {
                Some(pipeline) => pipeline.to_owned(),
                None => RUMCommandLine::new()
            },
            None => RUMCommandLine::new()
        }
    }
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
    pipelines: PipelineConf,
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

    pub fn get_pipelines(&self) -> &PipelineConf {
        &self.pipelines
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

    pub fn get_section(&self, section: &str) -> TextMap {
        match self.config.get(&self.lang) {
            Some(l) => match l.get(section) {
                Some(i) => i.clone(),
                None => self.get_default_item(section),
            },
            None => self.get_default_item(section),
        }
    }

    pub fn get_default_item(&self, section: &str) -> TextMap {
        match self.config.get(DEFAULT_TEXT_ITEM) {
            Some(l) => match l.get(section) {
                Some(i) => i.clone(),
                None => TextMap::default(),
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
#[derive(Default, Debug, Clone)]
pub struct AppState {
    config: AppConf,
    clipboard: NestedTextMap,
    jobs: RUMHashMap<RUMID, Job>,
}

pub type SharedAppState = SafeLock<AppState>;

impl AppState {
    pub fn new() -> AppState {
        AppState {
            config: AppConf::default(),
            clipboard: NestedTextMap::default(),
            jobs: RUMHashMap::default(),
        }
    }

    pub fn new_safe() -> SharedAppState {
        rumtk_new_lock!(AppState::new())
    }

    pub fn from_safe(conf: AppConf) -> SharedAppState {
        rumtk_new_lock!(AppState::from(conf))
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

pub type RouterAppState = State<SharedAppState>;

///
/// Load the configuration for this app at the specified path. By default, we look into
/// [DEFAULT_APP_CONFIG](crate::utils::defaults::DEFAULT_APP_CONFIG) as the location of the configuration.
///
/// ## Example
/// ```
/// use std::fs;
/// use rumtk_core::rumtk_new_lock;
/// use rumtk_web::{rumtk_web_save_conf, rumtk_web_load_conf, rumtk_web_get_config};
/// use rumtk_web::{AppConf};
/// use rumtk_core::strings::RUMString;
///
/// #[derive(Default)]
/// struct Args {
///     title: RUMString,
///     description: RUMString,
///     company: RUMString,
///     copyright: RUMString,
///     css_source_dir: RUMString,
///     ip: RUMString,
///     upload_limit: usize,
///     threads: usize,
///     skip_default_css: bool,
/// }
///
/// let path = "./test_conf.json";
///
/// if fs::exists(&path).unwrap() {
///     fs::remove_file(&path).unwrap();
/// }
///
/// rumtk_web_save_conf!(&path);
/// let app_state = rumtk_web_load_conf!(Args::default(), &path);
/// let config = rumtk_web_get_config!(app_state).clone();
///
/// if fs::exists(&path).unwrap() {
///     fs::remove_file(&path).unwrap();
/// }
///
/// assert_eq!(config, AppConf::default(), "Configuration was not loaded properly!");
/// ```
///
#[macro_export]
macro_rules! rumtk_web_load_conf {
    ( $args:expr ) => {{
        use $crate::defaults::{DEFAULT_APP_CONFIG};
        rumtk_web_load_conf!($args, DEFAULT_APP_CONFIG)
    }};
    ( $args:expr, $path:expr ) => {{
        use rumtk_core::rumtk_deserialize;
        use rumtk_core::strings::RUMStringConversions;
        use rumtk_core::types::RUMHashMap;
        use $crate::AppConf;
        use std::fs;

        use $crate::rumtk_web_save_conf;
        use $crate::utils::{AppState, TextMap};

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

///
/// Serializes [AppConf] default contents and saves it to a file on disk at a specified path or relative to
/// the current working directory. This is done to pre-craft a default configuration skeleton so
/// a consumer of the framework can simply update that file before testing and shipping to production.
///
/// By default, we generate the skeleton in [DEFAULT_APP_CONFIG](crate::utils::defaults::DEFAULT_APP_CONFIG).
///
/// ## Example
/// ```
/// use std::fs;
/// use rumtk_core::rumtk_new_lock;
/// use rumtk_web::rumtk_web_save_conf;
/// use rumtk_core::strings::RUMString;
///
/// let path = "./test_conf.json";
///
/// if fs::exists(&path).unwrap() {
///     fs::remove_file(&path).unwrap();
/// }
///
/// assert!(!fs::exists(&path).unwrap(), "File was not deleted as expected!");
///
/// rumtk_web_save_conf!(&path);
///
/// assert!(fs::exists(&path).unwrap(), "File was not created as expected!");
///
/// if fs::exists(&path).unwrap() {
///     fs::remove_file(&path).unwrap();
/// }
/// ```
///
#[macro_export]
macro_rules! rumtk_web_save_conf {
    (  ) => {{
        $crate::utils::defaults::DEFAULT_APP_CONFIG;
        rumtk_web_save_conf!(DEFAULT_APP_CONFIG)
    }};
    ( $path:expr ) => {{
        use rumtk_core::rumtk_serialize;
        use rumtk_core::strings::RUMStringConversions;
        use std::fs;
        use $crate::utils::AppConf;

        let json = rumtk_serialize!(AppConf::default(), true).unwrap_or_default();
        fs::write($path, &json);
        json
    }};
}

///
/// Retrieve a configuration ([AppConf]) static string. These are strings driven by the app designer's
/// generated configuration.
///
#[macro_export]
macro_rules! rumtk_web_get_config_string {
    ( $conf:expr, $item:expr ) => {{
        use $crate::rumtk_web_get_config;
        use $crate::AppConf;
        rumtk_web_get_config!($conf).get_text($item)
    }};
}

///
/// Retrieve a configuration ([AppConf]) item. These are strings driven by the app designer's
/// generated configuration. Unlike [rumtk_web_get_config_string](crate::rumtk_web_get_config_string), the item
/// retrieved here is separate from the strings section.
///
#[macro_export]
macro_rules! rumtk_web_get_config_section {
    ( $conf:expr, $item:expr ) => {{
        use $crate::rumtk_web_get_config;
        use $crate::AppConf;
        rumtk_web_get_config!($conf).get_section($item)
    }};
}

///
/// Retrieve access to a named pipeline as defined by the app configuration.
///
/// ## Example
/// ```
/// use rumtk_core::rumtk_new_lock;
/// use rumtk_web::{AppState};
/// use rumtk_web::defaults::DEFAULT_TEXT_ITEM;
/// use rumtk_web::{rumtk_web_get_pipeline};
///
/// let state = rumtk_new_lock!(AppState::new());
///
/// let pipeline = rumtk_web_get_pipeline!(state).get_pipeline(DEFAULT_TEXT_ITEM, DEFAULT_TEXT_ITEM);
///
/// assert_eq!(pipeline, vec![], "Pipeline field in the configuration was not empty!");
/// ```
///
#[macro_export]
macro_rules! rumtk_web_get_pipeline {
    ( $conf:expr ) => {{
        use $crate::rumtk_web_get_config;
        use $crate::AppConf;
        rumtk_web_get_config!($conf).get_pipelines()
    }};
}

///
/// Get field state from the configuration section of the [SharedAppState] object. The configuration
/// is of type [AppConf].
///
/// ## Example
/// ```
/// use rumtk_core::rumtk_new_lock;
/// use rumtk_web::{AppState};
/// use rumtk_web::{rumtk_web_set_config, rumtk_web_get_config};
///
/// let state = rumtk_new_lock!(AppState::new());
///
/// let new_lang = rumtk_web_get_config!(state).lang.clone();
///
/// assert_eq!(new_lang, "", "Language field in the configuration was not empty!");
/// ```
///
#[macro_export]
macro_rules! rumtk_web_get_config {
    ( $state:expr ) => {{
        use rumtk_core::{rumtk_lock_read};
        rumtk_lock_read!($state.clone()).get_config()
    }};
}

///
/// Set field or state in the configuration section of the [SharedAppState] object. The configuration
/// is of type [AppConf].
///
/// ## Example
/// ```
/// use rumtk_core::rumtk_new_lock;
/// use rumtk_core::strings::RUMString;
/// use rumtk_web::{AppState};
/// use rumtk_web::{rumtk_web_set_config, rumtk_web_get_config};
///
/// let state = rumtk_new_lock!(AppState::new());
/// let lang = RUMString::from("en");
///
/// rumtk_web_set_config!(state).lang = RUMString::from(lang.clone());
///
/// let new_lang = rumtk_web_get_config!(state).lang.clone();
///
/// assert_eq!(new_lang, lang, "Changing the language field in the configuration was not successful!");
/// ```
///
#[macro_export]
macro_rules! rumtk_web_set_config {
    ( $state:expr ) => {{
        use rumtk_core::rumtk_lock_write;
        rumtk_lock_write!($state.clone()).get_config_mut()
    }};
}

///
/// Facility for modifying the state in an instance of [SharedAppState].
///
/// ## Example
/// ```
/// use rumtk_core::rumtk_new_lock;
/// use rumtk_core::strings::RUMString;
/// use rumtk_web::{AppState, ClipboardID, SharedAppState};
/// use rumtk_web::rumtk_web_modify_state;
///
/// let state = rumtk_new_lock!(AppState::new());
/// let clipboard_id = ClipboardID::new("");
///
/// let item_list = rumtk_web_modify_state!(state).pop_clipboard(&clipboard_id);
///
/// assert_eq!(item_list, None, "A non empty item list was retrieved from the app state.");
/// ```
///
#[macro_export]
macro_rules! rumtk_web_modify_state {
    ( $state:expr ) => {{
        use rumtk_core::rumtk_lock_write;
        rumtk_lock_write!($state.clone())
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
