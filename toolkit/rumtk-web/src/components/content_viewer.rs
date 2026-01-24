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
use crate::defaults::PARAMS_TYPE;
use crate::utils::defaults::{
    DEFAULT_TEXT_ITEM, FORM_DATA_TYPE_HTML, FORM_DATA_TYPE_PDF, PARAMS_CONTENTS, PARAMS_CSS_CLASS,
};
use crate::utils::types::{HTMLResult, RUMString, SharedAppState, URLParams, URLPath};
use crate::{rumtk_web_get_text_item, rumtk_web_render_html};
use askama::Template;
use rumtk_core::rumtk_generate_id;
use rumtk_core::strings::RUMStringConversions;

#[derive(Template, Debug)]
#[template(
    source = "
        {% if custom_css_enabled %}
            <link href='/static/components/content_viewer.css' rel='stylesheet'>
        {% endif %}
        {% if typ == FORM_DATA_TYPE_PDF %}
            <object data='{{contents}}' type='application/pdf' class='f18 contact-card-{{ css_class }}-container'>
                <!-- Fallback content for browsers that cannot display the PDF inline -->
                <p>It appears your browser doesn't have a PDF plugin. No problem, you can
                <a href='{{contents}}'>click here to download the PDF file.</a></p>
            </object>
        {% else if typ == FORM_DATA_TYPE_HTML %}
            <div id='div-{{id}}' class='f18 contact-card-{{ css_class }}-container'></div>
            <script>
                const contentDiv = document.getElementById('div-{{id}}');
                 // Set the innerHTML to a string containing HTML markup
                 contentDiv.innerHTML = '{{contents|safe}}';
            </script>
        {% else %}
            <pre id='div-{{id}}' class='f18 contact-card-{{ css_class }}-container'>
                {{contents}}
            </pre>
        {% endif %}
        <div class='div-{{css_class}}'>{{contents|safe}}</div>
    ",
    ext = "html"
)]
pub struct ContentViewer<'a> {
    id: RUMString,
    typ: &'a str,
    contents: &'a str,
    css_class: &'a str,
    custom_css_enabled: bool,
}

pub fn content_viewer(
    _path_components: URLPath,
    params: URLParams,
    state: SharedAppState,
) -> HTMLResult {
    let id = rumtk_generate_id!().to_rumstring();
    let typ = rumtk_web_get_text_item!(params, PARAMS_TYPE, DEFAULT_TEXT_ITEM);
    let contents = rumtk_web_get_text_item!(params, PARAMS_CONTENTS, DEFAULT_TEXT_ITEM);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let custom_css_enabled = state.read().expect("Lock failure").config.custom_css;

    rumtk_web_render_html!(ContentViewer {
        id,
        typ,
        contents,
        css_class,
        custom_css_enabled
    })
}
