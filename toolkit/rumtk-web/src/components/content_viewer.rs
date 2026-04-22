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
use crate::defaults::{DEFAULT_NO_TEXT, PARAMS_ID, PARAMS_TYPE};
use crate::utils::defaults::{
    DEFAULT_TEXT_ITEM, FORM_DATA_TYPE_HTML, FORM_DATA_TYPE_PDF, PARAMS_CONTENTS, PARAMS_CSS_CLASS,
};
use crate::utils::types::{HTMLResult, SharedAppState, URLParams, URLPath};
use crate::{
    rumtk_web_get_config, rumtk_web_get_text_item, rumtk_web_render_template, RUMWebTemplate,
};
use rumtk_core::rumtk_generate_id;

#[derive(RUMWebTemplate, Debug)]
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
    ",
    ext = "html"
)]
pub struct ContentViewer<'a> {
    id: &'a str,
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
    let default_id = rumtk_generate_id!();
    let id = rumtk_web_get_text_item!(params, PARAMS_ID, default_id.as_str());
    let typ = rumtk_web_get_text_item!(params, PARAMS_TYPE, DEFAULT_TEXT_ITEM);
    let contents = rumtk_web_get_text_item!(params, PARAMS_CONTENTS, DEFAULT_NO_TEXT);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let custom_css_enabled = rumtk_web_get_config!(state).custom_css;

    rumtk_web_render_template!(ContentViewer {
        id,
        typ,
        contents,
        css_class,
        custom_css_enabled
    })
}
