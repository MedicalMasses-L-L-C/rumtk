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
use crate::defaults::{
    DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_ENDPOINT, PARAMS_MODULE, PARAMS_TARGET, PARAMS_TITLE, PARAMS_TYPE, SECTION_ENDPOINTS, SECTION_MODULES,
};
use crate::utils::types::{HTMLResult, RUMString, SharedAppState, URLParams, URLPath};
use crate::{
    rumtk_web_get_config, rumtk_web_get_config_section, rumtk_web_get_form, rumtk_web_get_text_item,
    rumtk_web_render_component, rumtk_web_render_template, RUMWebTemplate,
};

#[derive(RUMWebTemplate, Debug)]
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
            {% if !title.is_empty() %}
                {{title|safe}}
            {% endif %}
            <form id='form-{{typ}}' class='f18 centered form-default-container gap-10 form-{{css_class}}-container' role='form' hx-encoding='multipart/form-data' hx-post='{{endpoint}}' aria-label='{{typ}} form' hx-swap='outerHTML' hx-target='#form-{{typ}}-box'>
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
    typ: &'a str,
    title: &'a str,
    module: &'a str,
    endpoint: &'a str,
    elements: &'a [RUMString],
    css_class: &'a str,
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
        true => DEFAULT_NO_TEXT,
        false => &rumtk_web_render_component!("title", [(PARAMS_TYPE, title)], state)?.to_rumstring(),
    };

    let module_store = rumtk_web_get_config_section!(state, SECTION_MODULES);
    let module_name = rumtk_web_get_text_item!(&module_store, module, DEFAULT_NO_TEXT);

    let endpoint_store = rumtk_web_get_config_section!(state, SECTION_ENDPOINTS);
    let endpoint_url = rumtk_web_get_text_item!(&endpoint_store, endpoint, endpoint);

    let custom_css_enabled = rumtk_web_get_config!(state).custom_css;

    let elements = rumtk_web_get_form!(typ)?;

    rumtk_web_render_template!(Form {
        typ,
        title: title_elem,
        module: module_name,
        endpoint: endpoint_url,
        elements: elements.iter().as_ref(),
        css_class,
        custom_css_enabled,
        auto_hide_progress: auto_hide_progress == "progress_hidden",
    })
}
