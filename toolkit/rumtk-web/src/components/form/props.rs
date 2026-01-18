/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D.
 * Copyright (C) 2025  Nick Stephenson
 * Copyright (C) 2025  Ethan Dixon
 * Copyright (C) 2025  MedicalMasses L.L.C.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
 */
use rumtk_core::strings::{rumtk_format, RUMString};

type EventHandler<'a> = (&'a str, &'a str);
type EventHandlers<'a> = Vec<EventHandler<'a>>;

#[derive(Debug, Clone, Default)]
pub struct InputProps<'a> {
    pub name: Option<RUMString>,
    pub typ: Option<RUMString>,
    pub value: Option<RUMString>,
    pub placeholder: Option<RUMString>,
    pub pattern: Option<RUMString>,
    pub event_handlers: Option<EventHandlers<'a>>,
    pub max_length: Option<usize>,
    pub min_length: Option<usize>,
    pub autocapitalize: bool,
    pub autocomplete: bool,
    pub autocorrect: bool,
    pub autofocus: bool,
    pub disabled: bool,
    pub required: bool,
}

impl InputProps<'_> {
    fn get_handler_string(&self, handlers: &EventHandlers) -> RUMString {
        let mut handler_string = RUMString::default();

        for (handler_name, handler_function) in handlers {
            handler_string += &rumtk_format!(" {}={:?}", handler_name, handler_function);
        }

        handler_string
    }

    pub fn to_rumstring(&self) -> RUMString {
        let default_text = RUMString::default();
        rumtk_format!(
            "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
            match &self.name {
                Some(name) => rumtk_format!("id={:?}", name),
                None => default_text.clone(),
            },
            match &self.name {
                Some(name) => rumtk_format!("name={:?}", name),
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
            match &self.placeholder {
                Some(placeholder) => rumtk_format!("placeholder={:?}", placeholder),
                None => default_text.clone(),
            },
            match &self.pattern {
                Some(pattern) => rumtk_format!("pattern={:?}", pattern),
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
            match self.required {
                true => rumtk_format!("required"),
                false => default_text.clone(),
            },
        )
    }

    pub fn to_string(&self) -> String {
        self.to_rumstring().to_string()
    }
}
