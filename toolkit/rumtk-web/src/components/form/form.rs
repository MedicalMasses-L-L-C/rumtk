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
use crate::defaults::{
    DEFAULT_NO_TEXT, PARAMS_ENDPOINT, PARAMS_TARGET, PARAMS_TITLE, SECTION_ENDPOINTS,
};
use crate::utils::defaults::{
    DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_MODULE, PARAMS_TYPE, SECTION_MODULES,
};
use crate::utils::types::{HTMLResult, RUMString, SharedAppState, URLParams, URLPath};
use crate::{
    rumtk_web_get_conf, rumtk_web_get_form, rumtk_web_get_text_item, rumtk_web_render_component,
    rumtk_web_render_html,
};
use askama::Template;

#[derive(Template, Debug)]
#[template(
    source = "
        <div id='form-{{typ}}-box'>
            {% if custom_css_enabled %}
                <link href='/static/components/form/form.css' rel='stylesheet'>
            {% endif %}
            {% if !module.is_empty() %}
                <script type='module' id='form-script' src='/static/js/forms/form_{{typ}}.js'>
                </script>
            {% endif %}
            <form id='form-{{typ}}' class='f18 centered form-default-container gap-10 form-{{css_class}}-container' role='form' hx-encoding='multipart/form-data' hx-post='{{endpoint}}' aria-label='{{typ}} form' hx-swap='innerHTML' hx-target='#form-{{typ}}-box'>
                {% for element in elements %}
                    {{ element|safe }}
                {% endfor %}
            </form>
            <script>
                htmx.on('#form-{{typ}}', 'htmx:xhr:progress', function(evt) {
                  let progressValue = evt.detail.loaded/evt.detail.total * 100;
                  let progressElement = htmx.find('#progress');

                  {% if auto_hide_progress %}
                  progressElement.hidden = false;
                  if (progressValue >= 100) {
                     progressElement.hidden = true;
                  }
                  {% endif %}

                  progressElement.setAttribute('value', progressValue);
                });
            </script>
        </div>
    ",
    ext = "html"
)]
struct Form<'a> {
    typ: RUMString,
    title: RUMString,
    module: RUMString,
    endpoint: RUMString,
    elements: &'a [RUMString],
    css_class: RUMString,
    custom_css_enabled: bool,
    auto_hide_progress: bool,
}

pub fn form(_path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let typ = rumtk_web_get_text_item!(params, PARAMS_TYPE, DEFAULT_TEXT_ITEM);
    let title = rumtk_web_get_text_item!(params, PARAMS_TITLE, DEFAULT_NO_TEXT);
    let module = rumtk_web_get_text_item!(params, PARAMS_MODULE, typ);
    let endpoint = rumtk_web_get_text_item!(params, PARAMS_ENDPOINT, typ);
    let auto_hide_progress = rumtk_web_get_text_item!(params, PARAMS_TARGET, "progress_hidden");
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let title_elem = match title.is_empty() {
        true => RUMString::default(),
        false => rumtk_web_render_component!("title", [("type", title)], state.clone()),
    };

    let module_store = rumtk_web_get_conf!(state, SECTION_MODULES);
    let module_name = rumtk_web_get_text_item!(&module_store, module, DEFAULT_NO_TEXT);

    let endpoint_store = rumtk_web_get_conf!(state, SECTION_ENDPOINTS);
    let endpoint_url = rumtk_web_get_text_item!(&endpoint_store, endpoint, DEFAULT_NO_TEXT);

    let custom_css_enabled = state.read().expect("Lock failure").config.custom_css;

    let elements = rumtk_web_get_form!(typ);

    rumtk_web_render_html!(Form {
        typ: RUMString::from(typ),
        title: title_elem,
        module: RUMString::from(module_name),
        endpoint: RUMString::from(endpoint_url),
        elements: elements.iter().as_ref(),
        css_class: RUMString::from(css_class),
        custom_css_enabled,
        auto_hide_progress: auto_hide_progress == "progress_hidden",
    })
}
