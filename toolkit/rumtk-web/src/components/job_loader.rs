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

use crate::defaults::{DEFAULT_JOB_LOADER_NAME, DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_ELEMENT, PARAMS_ID};
use crate::{rumtk_web_get_text_item, rumtk_web_render_component, rumtk_web_render_template};
use crate::{HTMLResult, RUMWebTemplate, SharedAppState, URLParams, URLPath};

#[derive(RUMWebTemplate, Debug)]
#[template(
    source = "
        <div id='loader-{{job_id}}' class='centered container-default job-loader-{{css_class}}-container'>
            <div class='centered' hx-get='/component/{{element_name}}?id={{job_id}}' hx-trigger='every 2s' hx-swap='outerHTML' hx-target='#loader-{{job_id}}'>
                {{loader|safe}}
            </div>
        </div>
    ",
    ext = "html"
)]
pub struct JobLoader<'a> {
    job_id: &'a str,
    element_name: &'a str,
    loader: &'a str,
    css_class: &'a str,
}

pub fn job_loader(_path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let job_id = rumtk_web_get_text_item!(params, PARAMS_ID, DEFAULT_NO_TEXT);
    let element_name = rumtk_web_get_text_item!(params, PARAMS_ELEMENT, DEFAULT_JOB_LOADER_NAME);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let loader = &rumtk_web_render_component!("loader", [(PARAMS_CSS_CLASS, css_class)], state)?.to_rumstring();

    rumtk_web_render_template!(JobLoader {
            job_id,
            element_name,
            loader,
            css_class
        }
    )
}
