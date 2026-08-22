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
use crate::components::form::form_node::FormNode;
use crate::components::title::{title, Title};
use crate::defaults::{DEFAULT_HTMX_SWAP_MODE, DEFAULT_NO_TEXT, DEFAULT_PROGRESS_MODE, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_ENDPOINT, PARAMS_MODULE, PARAMS_PROGRESS_MODE, PARAMS_SWAP_MODE, PARAMS_TARGET, PARAMS_TITLE, PARAMS_TYPE, SECTION_ENDPOINTS, SECTION_MODULES};
use crate::utils::types::{RUMString, SharedAppState, URLParams, URLPath};
use crate::{rumtk_web_get_config, rumtk_web_get_config_section, rumtk_web_get_form, rumtk_web_get_text_item, rumtk_web_params_map, ComponentResult, RUMWebTemplate, RUMWebTemplateSafe};

#[derive(RUMWebTemplate, Debug)]
#[template(
    source = "
        <div id='form-{{htmx_target}}-box'>
            {% if custom_css_enabled %}
                <link href='/static/components/form/form.css' rel='stylesheet'>
            {% endif %}
            {% if !module.is_empty() %}
                <script type='module' id='form-script' src='/static/js/forms/form_{{typ}}.js' defer>
                </script>
            {% endif %}
            {{title|safe}}
            <form id='form-{{htmx_target}}' class='f18 centered form-default-contents gap-10 form-{{css_class}}-contents' role='form' hx-encoding='multipart/form-data' hx-post='{{endpoint}}' aria-label='{{typ}} form' hx-swap='{{htmx_swap_mode}}' hx-target='#form-{{htmx_target}}'>
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
pub struct Form {
    typ: RUMString,
    title: Title,
    module: RUMString,
    endpoint: RUMString,
    htmx_target: RUMString,
    htmx_swap_mode: RUMString,
    elements: Vec<FormNode>,
    css_class: RUMString,
    custom_css_enabled: bool,
    auto_hide_progress: bool,
}

impl RUMWebTemplateSafe for Form {}

pub fn form<'a>(_path_components: URLPath<'a, 'a>, params: URLParams<'a>, state: SharedAppState) -> ComponentResult<Form> {
    let typ = rumtk_web_get_text_item!(params, PARAMS_TYPE, DEFAULT_TEXT_ITEM).to_string();
    let title_str = rumtk_web_get_text_item!(params, PARAMS_TITLE, DEFAULT_NO_TEXT);
    let module = rumtk_web_get_text_item!(params, PARAMS_MODULE, &typ);
    let endpoint = rumtk_web_get_text_item!(params, PARAMS_ENDPOINT, &typ);
    let auto_hide_progress = rumtk_web_get_text_item!(params, PARAMS_PROGRESS_MODE, DEFAULT_PROGRESS_MODE);
    let htmx_swap_mode = rumtk_web_get_text_item!(params, PARAMS_SWAP_MODE, DEFAULT_HTMX_SWAP_MODE).to_string();
    let htmx_target = rumtk_web_get_text_item!(params, PARAMS_TARGET, DEFAULT_TEXT_ITEM).to_string();
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM).to_string();

    let title_params = rumtk_web_params_map!(
        [(PARAMS_TYPE, title_str)]
    );
    let title_elem = title(_path_components, title_params.get_inner(), state.clone())?;

    let module_store = rumtk_web_get_config_section!(state, SECTION_MODULES);
    let module_name = rumtk_web_get_text_item!(&module_store, module, DEFAULT_NO_TEXT).to_string();

    let endpoint_store = rumtk_web_get_config_section!(state, SECTION_ENDPOINTS);
    let endpoint_url = rumtk_web_get_text_item!(&endpoint_store, endpoint, endpoint).to_string();

    let custom_css_enabled = rumtk_web_get_config!(state).flags.custom_css;

    let element_results = rumtk_web_get_form!(&typ)?;
    let mut elements = Vec::<FormNode>::with_capacity(element_results.len());

    for result in element_results {
        elements.push(result?);
    }

    Ok(Form {
        typ,
        title: title_elem,
        module: module_name,
        endpoint: endpoint_url,
        htmx_target,
        htmx_swap_mode,
        elements,
        css_class,
        custom_css_enabled,
        auto_hide_progress: auto_hide_progress == DEFAULT_PROGRESS_MODE,
    })
}
