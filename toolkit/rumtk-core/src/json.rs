/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D.
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

pub mod serialization {
    pub use crate::types::{RUMDeserialize, RUMDeserializer, RUMSerialize, RUMSerializer};
    pub use serde_json::{from_str, to_string, to_string_pretty};

    ///
    /// Serialization macro which will take an object instance decorated with [Serialize] trait
    /// from serde and return the JSON string representation.
    ///
    /// You can pass up to two parameters. The first parameter is the serializable object instance.
    /// The second parameter is a boolean indicating whether to pretty print. Omit the second
    /// parameter if not debugging to save on bytes transferred around.
    ///
    /// # Examples
    /// ## Pretty Print
    /// ```
    /// use rumtk_core::json::serialization::{RUMSerialize};
    /// use rumtk_core::strings::RUMString;
    /// use rumtk_core::rumtk_serialize;
    ///
    /// #[derive(RUMSerialize)]
    /// struct MyStruct {
    ///     hello: RUMString
    /// }
    ///
    /// let hw = MyStruct{hello: RUMString::from("World")};
    /// let hw_str = rumtk_serialize!(&hw, true).unwrap();
    ///
    /// assert!(hw_str.len() > 0, "Empty JSON string generated from the test struct!");
    ///
    /// ```
    ///
    /// ## Default
    /// ```
    /// use rumtk_core::json::serialization::{RUMSerialize};
    /// use rumtk_core::strings::RUMString;
    /// use rumtk_core::rumtk_serialize;
    ///
    /// #[derive(RUMSerialize)]
    /// struct MyStruct {
    ///     hello: RUMString
    /// }
    ///
    /// let hw = MyStruct{hello: RUMString::from("World")};
    /// let hw_str = rumtk_serialize!(&hw).unwrap();
    ///
    /// assert!(hw_str.len() > 0, "Empty JSON string generated from the test struct!");
    ///
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_serialize {
        ( $object:expr ) => {{
            use $crate::json::serialization::{to_string, to_string_pretty};
            use $crate::strings::rumtk_format;

            match to_string(&$object) {
                Ok(s) => Ok(s),
                Err(e) => Err(rumtk_format!("Failed to serialize object because of {}", e)),
            }
        }};
        ( $object:expr, $pretty:expr ) => {{
            use $crate::json::serialization::{to_string, to_string_pretty};
            use $crate::strings::rumtk_format;

            match $pretty {
                true => match to_string_pretty(&$object) {
                    Ok(s) => Ok(s),
                    Err(e) => Err(rumtk_format!("Failed to serialize object because of {}", e)),
                },
                false => match to_string(&$object) {
                    Ok(s) => Ok(s),
                    Err(e) => Err(rumtk_format!("Failed to serialize object because of {}", e)),
                },
            }
        }};
    }

    ///
    /// Deserialization macro which will take a JSON string representation and return an instance
    /// of the specified type.
    ///
    /// Pass the json string to deserialize. You will need to specify the expected type that will
    /// be generated.
    ///
    /// # Example
    ///
    /// ```
    /// use rumtk_core::json::serialization::{RUMSerialize, RUMDeserialize};
    /// use rumtk_core::strings::RUMString;
    /// use rumtk_core::{rumtk_serialize, rumtk_deserialize};
    ///
    /// #[derive(RUMSerialize, RUMDeserialize, PartialEq)]
    /// struct MyStruct {
    ///     hello: RUMString
    /// }
    ///
    /// let hw = MyStruct{hello: RUMString::from("World")};
    /// let hw_str = rumtk_serialize!(&hw, true).unwrap();
    /// let new_hw: MyStruct = rumtk_deserialize!(&hw_str).unwrap();
    ///
    /// assert!(
    ///    new_hw == hw,
    ///    "Deserialized JSON does not match the expected value!"
    /// );
    ///
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_deserialize {
        ( $string:expr ) => {{
            use $crate::json::serialization::from_str;

            from_str(&$string)
        }};
    }
}
