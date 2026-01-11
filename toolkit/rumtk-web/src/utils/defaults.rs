use crate::utils::types::TextMap;
use phf_macros::phf_ordered_map;

/*
   TextMap
*/
pub static DEFAULT_TEXT_MAP: TextMap = phf_ordered_map!();

/*
   IP
*/
pub const DEFAULT_LOCAL_LISTENING_ADDRESS: &str = "127.0.0.1:3000";
pub const DEFAULT_OUTBOUND_LISTENING_ADDRESS: &str = "0.0.0.0:3000";

/*
   Misc
*/
pub const DEFAULT_TEXT_ITEM: &str = "default";
pub const DEFAULT_CONTACT_ITEM: &str = "company";
pub const DEFAULT_NO_TEXT: &str = "";

/*
    Options
*/
pub const OPT_INVERTED_DIRECTION: &str = "inverted";

/*
   Params
*/
pub const PARAMS_TITLE: &str = "title";
pub const PARAMS_TYPE: &str = "type";
pub const PARAMS_CSS_CLASS: &str = "class";
pub const PARAMS_SOCIAL_LIST: &str = "social_list";
pub const PARAMS_ITEM: &str = "item";
pub const PARAMS_INVERTED: &str = "inverted";
pub const PARAMS_SECTION: &str = "section";
pub const PARAMS_FUNCTION: &str = "function";
pub const PARAMS_TARGET: &str = "target";
pub const PARAMS_SIZE: &str = "size";
pub const PARAMS_CONTENTS: &str = "contents";

/*
   CONF SECTIONS
*/
pub const SECTION_TEXT: &str = "text";
pub const SECTION_PERSONNEL: &str = "personnel";
pub const SECTION_CONTACT: &str = "contact";
pub const SECTION_TITLES: &str = "titles";
pub const SECTION_API: &str = "api";
pub const SECTION_SOCIALS: &str = "socials";
pub const SECTION_SERVICES: &str = "services";
pub const SECTION_PRODUCTS: &str = "products";
pub const SECTION_LINKS: &str = "links";

/*
   LANGUAGES
*/
pub const LANG_EN: &str = "en";
pub const LANG_ES: &str = "es";

/*
   Icon
*/
pub const DEFAULT_ICON_STYLE: &str = "fa-solid";

/*
   Robots.txt
*/
pub const DEFAULT_ROBOT_TXT: &str = r"
User-agent: *
Disallow: /static/
";
