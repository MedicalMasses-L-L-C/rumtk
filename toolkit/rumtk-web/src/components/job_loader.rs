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
use crate::components::html::Container;
use crate::components::loader::{loader, Loader};
use crate::defaults::{DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_ID};
use crate::{rumtk_web_check_on_job, rumtk_web_get_text_item, rumtk_web_params_string, ComponentResult, RUMWebTemplateSafe, DEFAULT_HTMX_CHECK_TICKS, PARAMS_TICKS};
use crate::{RUMWebTemplate, SharedAppState, URLParams, URLPath};
use rumtk_core::strings::RUMString;

#[derive(RUMWebTemplate, Debug)]
#[template(
    source = "
        <div id='loader-{{job_id}}' class='centered container-default job-loader-{{css_class}}-container'>
            {% if let Some(loader) = loader %}
                <div class='centered' hx-get='/jobs/{{params_string}}' hx-trigger='every {{ticks}}' hx-swap='outerHTML' hx-target='#loader-{{job_id}}'>
                    {{loader}}
                </div>
            {% elif let Some(result) = results %}
                {{result}}
            {% endif %}
        </div>
    ",
    ext = "html"
)]
pub struct JobLoader {
    job_id: RUMString,
    ticks: RUMString,
    results: Option<Container>,
    loader: Option<Loader>,
    params_string: RUMString,
    css_class: RUMString,
}

impl RUMWebTemplateSafe for JobLoader {}

pub fn job_loader<'a>(_path_components: URLPath<'a, 'a>, params: URLParams<'a>, state: SharedAppState) -> ComponentResult<JobLoader> {
    let job_id = rumtk_web_get_text_item!(params, PARAMS_ID, DEFAULT_NO_TEXT).to_string();
    let ticks = rumtk_web_get_text_item!(params, PARAMS_TICKS, DEFAULT_HTMX_CHECK_TICKS).to_string();
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM).to_string();
    let params_string = rumtk_web_params_string!(&params);

    match rumtk_web_check_on_job!(&job_id, state) {
        Some(r) => {
            Ok(JobLoader {
                job_id,
                ticks,
                results: Some(r),
                loader: None,
                params_string,
                css_class
            })
        },
        None => {
            let loader = loader(_path_components, params, state)?;

            Ok(JobLoader {
                job_id,
                ticks,
                results: None,
                loader: Some(loader),
                params_string,
                css_class
            })
        }
    }
}
