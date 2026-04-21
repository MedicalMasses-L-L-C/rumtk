use rumtk_core::strings::RUMString;
use rumtk_web::components::form::{props::InputProps, FormElementBuilder, FormElements};

pub fn basic_benchmark(builder: FormElementBuilder) -> FormElements {
    vec![
        builder(
            "input",
            "",
            InputProps {
                id: Some(RUMString::from("basic_start")),
                name: None,
                typ: Some(RUMString::from("submit")),
                value: Some(RUMString::from("Start")),
                max: None,
                placeholder: None,
                pattern: None,
                accept: None,
                alt: None,
                aria_label: Some(RUMString::from("Start Basic Benchmark")),
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
            },
            "f18"
        )
    ]
}