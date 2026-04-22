/*
 *     rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 *     This toolkit aims to be reliable, simple, performant, and standards compliant.
 *     Copyright (C) 2026  Luis M. Santos, M.D.
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
use rumtk_web::defaults::*;
use rumtk_web::rumtk_web_render_component;
use rumtk_web::utils::*;

pub fn index(app_state: SharedAppState) -> RenderedPageComponentsResult {
    let title_intro = rumtk_web_render_component!("title", [(PARAMS_TYPE, "intro")], app_state)?.to_rumstring();
    let text_card_intro = rumtk_web_render_component!("text_card", [(PARAMS_TYPE, "instructions")], app_state)?.to_rumstring();
    let basic_benchmark = rumtk_web_render_component!(
        "form",
        [
            (PARAMS_TYPE, "basic_benchmark"),
            (PARAMS_TITLE, "basic benchmark"),
            (PARAMS_TARGET, "basic_benchmark"),
            (PARAMS_SWAP_MODE, "innerHTML"),
            (PARAMS_ENDPOINT, "/api/benchmarks/basic")
        ],
        app_state
    )?.to_rumstring();

    Ok(vec![title_intro, text_card_intro, basic_benchmark])
}
