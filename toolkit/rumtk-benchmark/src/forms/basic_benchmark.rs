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
use rumtk_web::components::form::{props::InputProps, FormElementBuilder, FormElements};
use rumtk_web::rumtk_web_get_pipelines;
use rumtk_web::SharedAppState;

pub fn basic_benchmark(builder: FormElementBuilder, state: &SharedAppState) -> FormElements {
    let benchmark_items = rumtk_web_get_pipelines!(state).get_available_pipeline_names().iter().map(|s| s.as_str()).collect::<Vec<_>>().join(",");
    let template_items = rumtk_web_get_pipelines!(state).get_available_data_templates().iter().map(|s| s.as_str()).collect::<Vec<_>>().join(",");
    vec![
        builder(
            "label",
            "Select utility to benchmark!",
            InputProps {
                id: Some("basic_select_label"),
                name: None,
                for_element: Some("basic_choice"),
                typ: Some("text"),
                value: None,
                max: None,
                placeholder: Some("text"),
                pattern: None,
                accept: None,
                alt: None,
                aria_label: Some("Utility Selection for Basic Benchmark"),
                event_handlers: None,
                max_length: None,
                min_length: None,
                autocapitalize: false,
                autocomplete: false,
                autocorrect: false,
                autofocus: false,
                disabled: false,
                hidden: false,
                required: false,
                multiple: false,
            },
            "f18"
        ),
        builder(
            "select",
            benchmark_items.as_str(),
            InputProps {
                id: Some("basic_choice"),
                name: Some("basic_choice"),
                for_element: None,
                typ: Some("text"),
                value: None,
                max: None,
                placeholder: Some("text"),
                pattern: None,
                accept: None,
                alt: None,
                aria_label: Some("Utility Selection for Basic Benchmark"),
                event_handlers: None,
                max_length: None,
                min_length: None,
                autocapitalize: false,
                autocomplete: false,
                autocorrect: false,
                autofocus: false,
                disabled: false,
                hidden: false,
                required: false,
                multiple: false,
            },
            "f18"
        ),
        builder(
            "label",
            "Select message profile!",
            InputProps {
                id: Some("basic_template_label"),
                name: None,
                for_element: Some("basic_template"),
                typ: Some("text"),
                value: None,
                max: None,
                placeholder: Some("text"),
                pattern: None,
                accept: None,
                alt: None,
                aria_label: Some("Select message profile!"),
                event_handlers: None,
                max_length: None,
                min_length: None,
                autocapitalize: false,
                autocomplete: false,
                autocorrect: false,
                autofocus: false,
                disabled: false,
                hidden: false,
                required: false,
                multiple: false,
            },
            "f18"
        ),
        builder(
            "select",
            template_items.as_str(),
            InputProps {
                id: Some("basic_template"),
                name: Some("basic_template"),
                for_element: None,
                typ: Some("text"),
                value: None,
                max: None,
                placeholder: Some("text"),
                pattern: None,
                accept: None,
                alt: None,
                aria_label: Some("Benchmark Template Selection"),
                event_handlers: None,
                max_length: None,
                min_length: None,
                autocapitalize: false,
                autocomplete: false,
                autocorrect: false,
                autofocus: false,
                disabled: false,
                hidden: false,
                required: false,
                multiple: false,
            },
            "f18"
        ),
        builder(
            "input",
            "",
            InputProps {
                id: Some("basic_start"),
                name: None,
                for_element: None,
                typ: Some("submit"),
                value: Some("Start"),
                max: None,
                placeholder: None,
                pattern: None,
                accept: None,
                alt: None,
                aria_label: Some("Start Basic Benchmark"),
                event_handlers: None,
                max_length: None,
                min_length: None,
                autocapitalize: false,
                autocomplete: false,
                autocorrect: false,
                autofocus: false,
                disabled: false,
                hidden: false,
                required: false,
                multiple: false,
            },
            "f18"
        )
    ]
}