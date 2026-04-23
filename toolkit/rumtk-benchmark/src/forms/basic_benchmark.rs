use rumtk_core::strings::CompactStringExt;
use rumtk_web::components::form::{props::InputProps, FormElementBuilder, FormElements};
use rumtk_web::rumtk_web_get_pipeline;
use rumtk_web::SharedAppState;

pub fn basic_benchmark(builder: FormElementBuilder, state: &SharedAppState) -> FormElements {
    let benchmark_items = rumtk_web_get_pipeline!(state).get_available_pipeline_names("basic").join_compact(",");
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