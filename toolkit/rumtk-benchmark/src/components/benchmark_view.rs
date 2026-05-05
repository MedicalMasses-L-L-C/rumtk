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
use rumtk_core::rumtk_deserialize;
use rumtk_core::strings::RUMString;
use rumtk_web::defaults::{DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, PARAMS_CONTENTS, PARAMS_CSS_CLASS, PARAMS_ID};
use rumtk_web::{rumtk_web_check_on_job, rumtk_web_get_text_item, rumtk_web_render_component, rumtk_web_render_markdown, rumtk_web_render_template};
use rumtk_web::{HTMLResult, RUMWebTemplate, SharedAppState, URLParams, URLPath};

#[derive(RUMWebTemplate, Debug)]
#[template(
    source = "
        <div class='f24 benchmark-view-{{css_class}} md'>{{data|safe}}</div>
    ",
    ext = "html"
)]
pub struct BenchmarkReportView<'a> {
    data: &'a str,
    css_class: &'a str,
}

pub fn benchmark_view(_path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let job_id = rumtk_web_get_text_item!(params, PARAMS_ID, DEFAULT_NO_TEXT);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let job_result = match rumtk_web_check_on_job!("benchmark_view", job_id, state) {
        Some(result) => result?.to_string(),
        None => RUMString::default()
    };
    
    let data = rumtk_web_render_component!("container", [(PARAMS_CONTENTS, job_result)], state)?.to_string();

    rumtk_web_render_template!(BenchmarkReportView {
            data: data.as_str(),
            css_class
        }
    )
}
