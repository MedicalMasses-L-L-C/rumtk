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
use rumtk_core::strings::{rumtk_format, CompactStringExt, RUMString};

type EventHandler<'a> = (&'a str, &'a str);
type EventHandlers<'a> = Vec<EventHandler<'a>>;

#[derive(Debug, Clone, Default)]
pub struct InputProps<'a> {
    pub id: Option<&'a str>,
    pub name: Option<&'a str>,
    pub typ: Option<&'a str>,
    pub value: Option<&'a str>,
    pub max: Option<&'a str>,
    pub placeholder: Option<&'a str>,
    pub pattern: Option<&'a str>,
    pub accept: Option<&'a str>,
    pub alt: Option<&'a str>,
    pub aria_label: Option<&'a str>,
    pub for_element: Option<&'a str>,
    pub event_handlers: Option<EventHandlers<'a>>,
    pub max_length: Option<usize>,
    pub min_length: Option<usize>,
    pub autocapitalize: bool,
    pub autocomplete: bool,
    pub autocorrect: bool,
    pub autofocus: bool,
    pub disabled: bool,
    pub hidden: bool,
    pub required: bool,
    pub multiple: bool,
}

impl InputProps<'_> {
    fn get_handler_string(&self, handlers: &EventHandlers) -> RUMString {
        let mut handler_string = RUMString::default();

        for (handler_name, handler_function) in handlers {
            handler_string += &rumtk_format!(" {}={:?}", handler_name, handler_function);
        }

        handler_string.trim().to_string()
    }

    pub fn to_rumstring(&self) -> RUMString {
        let default_text = RUMString::default();
        let mut options = vec![
            match &self.id {
                Some(id) => rumtk_format!("id={:?}", id),
                None => default_text.clone(),
            },
            match &self.name {
                Some(name) => rumtk_format!("name={:?}", name),
                None => default_text.clone(),
            },
            match &self.for_element {
                Some(name) => rumtk_format!("for={:?}", name),
                None => default_text.clone(),
            },
            match &self.typ {
                Some(typ) => rumtk_format!("type={:?}", typ),
                None => default_text.clone(),
            },
            match &self.value {
                Some(val) => rumtk_format!("value={:?}", val),
                None => default_text.clone(),
            },
            match &self.max {
                Some(val) => rumtk_format!("max={:?}", val),
                None => default_text.clone(),
            },
            match &self.placeholder {
                Some(placeholder) => rumtk_format!("placeholder={:?}", placeholder),
                None => default_text.clone(),
            },
            match &self.pattern {
                Some(pattern) => rumtk_format!("pattern={:?}", pattern),
                None => default_text.clone(),
            },
            match &self.accept {
                Some(accept) => rumtk_format!("accept={:?}", accept),
                None => default_text.clone(),
            },
            match &self.alt {
                Some(alt) => rumtk_format!("alt={:?}", alt),
                None => default_text.clone(),
            },
            match &self.aria_label {
                Some(aria_label) => rumtk_format!("aria-label={:?}", aria_label),
                None => default_text.clone(),
            },
            match &self.event_handlers {
                Some(handlers) => self.get_handler_string(handlers),
                None => default_text.clone(),
            },
            match self.max_length {
                Some(max_length) => rumtk_format!("maxlength={:?}", max_length),
                None => default_text.clone(),
            },
            match self.min_length {
                Some(min_length) => rumtk_format!("minlength={:?}", min_length),
                None => default_text.clone(),
            },
            match self.autocapitalize {
                true => rumtk_format!("autocapitalize"),
                false => default_text.clone(),
            },
            match self.autocomplete {
                true => rumtk_format!("autocomplete"),
                false => default_text.clone(),
            },
            match self.autocorrect {
                true => rumtk_format!("autocorrect"),
                false => default_text.clone(),
            },
            match self.autofocus {
                true => rumtk_format!("autofocus"),
                false => default_text.clone(),
            },
            match self.disabled {
                true => rumtk_format!("disabled"),
                false => default_text.clone(),
            },
            match self.hidden {
                true => rumtk_format!("hidden"),
                false => default_text.clone(),
            },
            match self.required {
                true => rumtk_format!("required"),
                false => default_text.clone(),
            },
        ];
        options
            .into_iter()
            .filter(|itm| !itm.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn to_string(&self) -> String {
        self.to_string().to_string()
    }
}
