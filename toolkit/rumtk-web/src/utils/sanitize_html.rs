/*
 *     rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 *     This toolkit aims to be reliable, simple, performant, and standards compliant.
 *     Copyright (C) 2026  Luis M. Santos, M.D. <lsantos@medicalmasses.com>
 *     Copyright (C) 2026  MedicalMasses L.L.C. <contact@medicalmasses.com>
 *
 *     This program is free software: you can redistribute it and/or modify
 *     it under the terms of the GNU General Public License as published by
 *     the Free Software Foundation, either version 3 of the License, or
 *     (at your option) any later version.
 *
 *     This program is distributed in the hope that it will be useful,
 *     but WITHOUT ANY WARRANTY; without even the implied warranty of
 *     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *     GNU General Public License for more details.
 *
 *     You should have received a copy of the GNU General Public License
 *     along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use ammonia::Builder;
use rumtk_core::dependencies::maplit::{hashmap, hashset};
use rumtk_core::strings::RUMString;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::iter::IntoIterator;
use std::sync::LazyLock;

type AllowedSet = LazyLock<HashSet<&'static str>>;
type AllowedMap = LazyLock<HashMap<&'static str, HashSet<&'static str>>>;

static CLEAN_CONTENT_TAGS: AllowedSet = LazyLock::new(|| hashset!["script", "style"]);
static CLEAN_CONTENT_TAGS_RELAXED: AllowedSet = LazyLock::new(|| hashset![]);
static ALLOWED_URL_SCHEMES: AllowedSet = LazyLock::new(|| hashset![
    "https", "mailto", "data"
]);
static ALLOWED_URL_SCHEMES_RELAXED: AllowedSet = LazyLock::new(|| hashset![
    "http", "https", "mailto", "data"
]);
const ALLOWED_LINK_POLICY: Option<&str> = Some("noopener noreferrer");
static ALLOWED_TAGS: AllowedSet = LazyLock::new(|| hashset![
    "animate", "video", "object", "img", "noscript", "meta", "link",
    "title", "form", "input", "select", "option", "textarea",
    "button", "label", "fieldset", "legend",

    "svg", "circle", "path", "polygon"
]);
static ALLOWED_TAGS_RELAXED: AllowedSet = LazyLock::new(|| hashset![
    "script", "html", "head", "body", "header", "main", "footer", "style"
]);
static ALLOWED_GENERIC_ATTR: AllowedSet = LazyLock::new(|| hashset![
    "class", "open", "hidden", "alt", "type", "height", "width", "href", "id", "data",
    "action", "formaction",
    "name", "for", "value", "max", "placeholder", "accept", "alt", "pattern",
    "maxlength", "minlength", "autocapitalize",
    "autocomplete", "autocorrect", "autofocus", "disabled", "hidden", "required",
    "content"
]);
static ALLOWED_GENERIC_ATTR_RELAXED: AllowedSet = LazyLock::new(|| hashset![
    "onload", "onerror", "style", "src", "srcset", "sizes", "width", "height",
    "fetchpriority", "defer", "role", "as", "rel", "lang",
    "onload", "onerror", "onclick", "ondblclick", "onmouseover", "onmouseout",
    "onmousedown", "onmouseup", "onwheel", "onkeydown", "onkeyup", "onkeypress",
    "onchange", "onfocus", "onblur", "oninput", "onsubmit", "onreset", "onunload",
    "onresize", "onhashchange", "onplay", "onpause", "onended", "onvolumechange",
    "ontimeupdate"
]);
static ALLOWED_HTMX_ATTR: AllowedSet = LazyLock::new(|| hashset![
    //Core attributes
    "hx-get", "hx-post", "hx-put", "hx-patch", "hx-delete", "hx-on", "hx-push-url",
    "hx-select", "hx-select-oob", "hx-swap", "hx-swap-oob", "hx-target", "hx-trigger",
    "hx-vals",
    //Additional attributes
    "hx-boost", "hx-confirm", "hx-disable", "hx-disable-elt", "hx-disinherit",
    "hx-encoding", "hx-ext", "hx-headers", "hx-history", "hx-history-elt", "hx-include",
    "hx-indicator", "hx-inherit", "hx-params", "hx-preserve", "hx-prompt", "hx-replace-url",
    "hx-request", "hx-sync", "hx-validate", "hx-vars",
]);
static mut ALLOWED_ATTRS: AllowedMap = LazyLock::new(|| hashmap![
            "a" => hashset![
                "href", "hreflang"
            ],
            "bdo" => hashset![
                "dir"
            ],
            "blockquote" => hashset![
                "cite"
            ],
            "col" => hashset![
                "align", "char", "charoff", "span"
            ],
            "colgroup" => hashset![
                "align", "char", "charoff", "span"
            ],
            "del" => hashset![
                "cite", "datetime"
            ],
            "hr" => hashset![
                "align", "size", "width"
            ],
            "img" => hashset![
                "align", "alt", "height", "src", "width"
            ],
            "ins" => hashset![
                "cite", "datetime"
            ],
            "ol" => hashset![
                "start"
            ],
            "q" => hashset![
                "cite"
            ],
            "table" => hashset![
                "align", "char", "charoff", "summary"
            ],
            "tbody" => hashset![
                "align", "char", "charoff"
            ],
            "td" => hashset![
                "align", "char", "charoff", "colspan", "headers", "rowspan"
            ],
            "tfoot" => hashset![
                "align", "char", "charoff"
            ],
            "th" => hashset![
                "align", "char", "charoff", "colspan", "headers", "rowspan", "scope"
            ],
            "thead" => hashset![
                "align", "char", "charoff"
            ],
            "tr" => hashset![
                "align", "char", "charoff"
            ],


            "object" => hashset![
                "data", "type", "img"
            ],
            "path" => hashset![
                "d", "stroke-linecap", "stroke-linejoin"
            ],
            "svg" => hashset![
                "fill", "height", "opacity", "stroke", "stroke-width", "viewbox",
                "width", "version", "baseProfile"
            ],
            "polygon" => hashset![
                "points", "fill", "stroke", "stroke-width"
            ],
            "circle" => hashset![
                "cx", "cy", "r", "fill", "stroke", "stroke-width"
            ],
            "video" => hashset![
                "src", "controls", "autoplay", "loop", "muted", "width", "height"
            ],
            "animate" => hashset![
                "attributeName", "from", "to", "dur", "fill", "begin", "repeatCount"
            ],
]);
static mut ALLOWED_ATTRS_RELAXED: AllowedMap = LazyLock::new(|| hashmap![
            "a" => hashset![
                "href", "hreflang"
            ],
            "bdo" => hashset![
                "dir"
            ],
            "blockquote" => hashset![
                "cite"
            ],
            "col" => hashset![
                "align", "char", "charoff", "span"
            ],
            "colgroup" => hashset![
                "align", "char", "charoff", "span"
            ],
            "del" => hashset![
                "cite", "datetime"
            ],
            "hr" => hashset![
                "align", "size", "width"
            ],
            "img" => hashset![
                "align", "alt", "height", "src", "width"
            ],
            "ins" => hashset![
                "cite", "datetime"
            ],
            "ol" => hashset![
                "start"
            ],
            "q" => hashset![
                "cite"
            ],
            "table" => hashset![
                "align", "char", "charoff", "summary"
            ],
            "tbody" => hashset![
                "align", "char", "charoff"
            ],
            "td" => hashset![
                "align", "char", "charoff", "colspan", "headers", "rowspan"
            ],
            "tfoot" => hashset![
                "align", "char", "charoff"
            ],
            "th" => hashset![
                "align", "char", "charoff", "colspan", "headers", "rowspan", "scope"
            ],
            "thead" => hashset![
                "align", "char", "charoff"
            ],
            "tr" => hashset![
                "align", "char", "charoff"
            ],


            "object" => hashset![
                "data", "type", "img"
            ],
            "style" => hashset![
            ],
            "script" => hashset![
                "src", "integrity", "crossorigin"
            ],
            "input" => hashset![
                "id", "name", "for", "type", "value", "max", "placeholder",
                "accept", "alt", "pattern", "maxlength", "minlength", "autocapitalize",
                "autocomplete", "autocorrect", "autofocus", "disabled", "hidden", "required",
                "onload", "onerror", "onclick", "ondblclick", "onmouseover", "onmouseout",
                "onmousedown", "onmouseup", "onwheel", "onkeydown", "onkeyup", "onkeypress",
                "onchange", "onfocus", "onblur", "oninput", "onsubmit", "onreset", "onunload",
                "onresize", "onhashchange", "onplay", "onpause", "onended", "onvolumechange",
                "ontimeupdate",
            ],
            "select" => hashset![
                "id", "name", "for", "type", "value", "max", "placeholder",
                "accept", "alt", "pattern", "maxlength", "minlength", "autocapitalize",
                "autocomplete", "autocorrect", "autofocus", "disabled", "hidden", "required",
                "onload", "onerror", "onclick", "ondblclick", "onmouseover", "onmouseout",
                "onmousedown", "onmouseup", "onwheel", "onkeydown", "onkeyup", "onkeypress",
                "onchange", "onfocus", "onblur", "oninput", "onsubmit", "onreset", "onunload",
                "onresize", "onhashchange", "onplay", "onpause", "onended", "onvolumechange",
                "ontimeupdate",
            ],
            "path" => hashset![
                "d", "stroke-linecap", "stroke-linejoin"
            ],
            "svg" => hashset![
                "fill", "height", "opacity", "stroke", "stroke-width", "viewbox",
                "width", "version", "baseProfile"
            ],
            "polygon" => hashset![
                "points", "fill", "stroke", "stroke-width"
            ],
            "circle" => hashset![
                "cx", "cy", "r", "fill", "stroke", "stroke-width"
            ],
            "video" => hashset![
                "src", "controls", "autoplay", "loop", "muted", "width", "height"
            ],
            "animate" => hashset![
                "attributeName", "from", "to", "dur", "fill", "begin", "repeatCount"
            ],
]);
static mut STRICT_SANITIZER: LazyLock<Builder> = LazyLock::new(|| unsafe {
        let mut sanitizer = Builder::default();
        default_init_sanitizer(&mut sanitizer, |sanitizer| { });
        sanitizer
    }
);
static mut RELAXED_SANITIZER: LazyLock<Builder> = LazyLock::new(|| unsafe {
        let mut sanitizer = Builder::default();
        default_init_sanitizer(&mut sanitizer, |sanitizer| {
            sanitizer
                .link_rel(None)
                .add_tags((*ALLOWED_TAGS_RELAXED).clone())
                .url_schemes((*ALLOWED_URL_SCHEMES_RELAXED).clone())
                .add_generic_attributes((*ALLOWED_GENERIC_ATTR_RELAXED).clone())
                .clean_content_tags(CLEAN_CONTENT_TAGS_RELAXED.clone())
                ;
            }
        );
        sanitizer
    }
);

#[inline]
unsafe fn default_init_sanitizer(builder: &mut Builder, init_closure: impl FnOnce(&mut Builder)) {
    builder
        .link_rel(ALLOWED_LINK_POLICY)
        .url_schemes(ALLOWED_URL_SCHEMES.clone())
        .add_tags((*ALLOWED_TAGS).clone())
        .tag_attributes((*ALLOWED_ATTRS).clone())
        .add_generic_attributes((*ALLOWED_GENERIC_ATTR).clone())
        .add_generic_attributes((*ALLOWED_HTMX_ATTR).clone())
        .clean_content_tags(CLEAN_CONTENT_TAGS.clone())
        .strip_comments(true);
    init_closure(builder);
}

#[inline]
pub fn select_sanitizer<'a>(relaxed: bool) -> &'a Builder<'static> {
    match relaxed {
        true => unsafe {&*RELAXED_SANITIZER},
        false => unsafe {&*STRICT_SANITIZER},
    }
}

#[inline]
pub fn select_sanitizer_mut<'a>(relaxed: bool) -> &'a mut Builder<'static> {
    match relaxed {
        true => unsafe {&mut *RELAXED_SANITIZER},
        false => unsafe {&mut *STRICT_SANITIZER},
    }
}

#[inline]
pub fn sanitizer_update_attributes<T: 'static + ?Sized + Borrow<str>, I: IntoIterator<Item = &'static T>>(it: I, relaxed: bool) {
    select_sanitizer_mut(relaxed).add_generic_attributes(it);
}

#[inline]
pub fn sanitizer_update_tag_attributes<T: 'static + ?Sized + Borrow<str>, U: 'static + ?Sized + Borrow<str>, I: IntoIterator<Item = &'static T>>(tag: &'static U, it: I, relaxed: bool) {
    select_sanitizer_mut(relaxed).add_tag_attributes(tag, it);
}

#[inline]
pub fn sanitizers_update_attributes<T: 'static + ?Sized + Borrow<str>, I: IntoIterator<Item = &'static T> + Clone>(it: I) {
    select_sanitizer_mut(false).add_generic_attributes(it.clone());
    select_sanitizer_mut(true).add_generic_attributes(it);
}

#[inline]
pub fn sanitizers_update_tag_attributes<T: 'static + ?Sized + Borrow<str>, I: IntoIterator<Item = &'static T> + Clone>(tag: &'static str, it: I) {
    select_sanitizer_mut(false).add_tag_attributes(tag, it.clone());
    select_sanitizer_mut(false).rm_clean_content_tags(&[tag]);
    select_sanitizer_mut(true).add_tag_attributes(tag, it);
    select_sanitizer_mut(true).rm_clean_content_tags(&[tag]);
}

#[inline]
pub fn sanitize_html_strict(html: &str) -> RUMString {
    select_sanitizer(false).clean(html).into()
}

#[inline]
pub fn sanitize_html_relaxed(html: &str) -> RUMString {
    select_sanitizer(true).clean(html).into()
}

#[inline]
pub fn sanitize_html(html: &str, relaxed: bool) -> RUMString {
    select_sanitizer(relaxed).clean(html).into()
}
