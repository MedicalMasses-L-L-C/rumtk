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
use crate::defaults::{DEFAULT_NO_TEXT, PARAMS_ENDPOINT};
use crate::utils::defaults::{
    DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_MODULE, PARAMS_TYPE, SECTION_MODULES,
};
use crate::utils::types::{HTMLResult, RUMString, SharedAppConf, URLParams, URLPath};
use crate::{
    rumtk_web_get_conf, rumtk_web_get_form, rumtk_web_get_text_item, rumtk_web_render_html,
};
use askama::Template;

#[derive(Template, Debug)]
#[template(
    source = "
        {% if custom_css_enabled %}
            <link href='/static/components/form/form.css' rel='stylesheet'>
        {% endif %}
        {% if !module.is_empty() %}
            <script type='module' id='form-script' src='/static/js/forms/{{typ}}.js'>
            </script>
        {% endif %}
        <script>
            htmx.on('#form', 'htmx:xhr:progress', function(evt) {
              htmx.find('#progress').setAttribute('value', evt.detail.loaded/evt.detail.total * 100)
            });
        </script>
        <form id='form-{{typ}}' class='f18 centered form-default-container gap-10' class='form-{{css_class}}-container' hx-encoding='multipart/form-data' hx-post='{{endpoint}}' >
            {% for element in elements %}
                {{ element|safe }}
            {% endfor %}
        </form>
    ",
    ext = "html"
)]
struct Form<'a> {
    typ: RUMString,
    module: RUMString,
    endpoint: RUMString,
    elements: &'a [RUMString],
    css_class: RUMString,
    custom_css_enabled: bool,
}

pub fn form(_path_components: URLPath, params: URLParams, state: SharedAppConf) -> HTMLResult {
    let typ = rumtk_web_get_text_item!(params, PARAMS_TYPE, DEFAULT_TEXT_ITEM);
    let module = rumtk_web_get_text_item!(params, PARAMS_MODULE, typ);
    let module = rumtk_web_get_text_item!(params, PARAMS_ENDPOINT, typ);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let module_store = rumtk_web_get_conf!(state, SECTION_MODULES);
    let module_name = rumtk_web_get_text_item!(&module_store, module, DEFAULT_NO_TEXT);

    let custom_css_enabled = state.read().expect("Lock failure").custom_css;

    let elements = rumtk_web_get_form!(typ);

    rumtk_web_render_html!(Form {
        typ: RUMString::from(typ),
        module: RUMString::from(module_name),
        elements: elements.iter().as_ref(),
        css_class: RUMString::from(css_class),
        custom_css_enabled
    })
}
