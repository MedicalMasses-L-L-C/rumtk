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
use crate::base::RUMResult;
use crate::strings::{rumtk_format, RUMString};
pub use nanoserde::{DeBin as RUMDeBin, DeJson as RUMDeJson, SerBin as RUMSerBin, SerJson as RUMSerJson};

#[inline(always)]
    pub fn from_json<T>(input: &str) -> RUMResult<T>
    where
        T: RUMDeJson
    {
        match T::deserialize_json(input) {
            Ok(value) => Ok(value),
            Err(e) => Err(rumtk_format!("Failed to deserialize object because of {}", e))
        }
    }

    #[inline(always)]
    pub fn to_json<T>(input: &T) -> RUMString
    where
        T: RUMSerJson
    {
        input.serialize_json()
    }

    ///
    /// Serialization macro which will take an object instance decorated with [Serialize] trait
    /// from serde and return the JSON string representation.
    ///
    /// You can pass up to two parameters. The first parameter is the serializable object instance.
    /// The second parameter is a boolean indicating whether to pretty print. Omit the second
    /// parameter if not debugging to save on bytes transferred around.
    ///
    /// # Examples
    ///
    /// ## Default
    /// ```
    /// use rumtk_core::serde::json::{RUMSerJson};
    /// use rumtk_core::strings::RUMString;
    /// use rumtk_core::rumtk_serialize;
    ///
    /// #[derive(RUMSerJson)]
    /// struct MyStruct {
    ///     hello: RUMString
    /// }
    ///
    /// let hw = MyStruct{hello: RUMString::from("World")};
    /// let hw_str = rumtk_serialize!(&hw);
    ///
    /// assert!(hw_str.len() > 0, "Empty JSON string generated from the test struct!");
    ///
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_serialize {
        ( $object:expr ) => {{
            use $crate::serde::json::{to_json};

            to_json($object)
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
    /// use rumtk_core::serde::json::{RUMSerJson, RUMDeJson};
    /// use rumtk_core::strings::RUMString;
    /// use rumtk_core::{rumtk_serialize, rumtk_deserialize};
    ///
    /// #[derive(RUMSerJson, RUMDeJson, PartialEq)]
    /// struct MyStruct {
    ///     hello: RUMString
    /// }
    ///
    /// let hw = MyStruct{hello: RUMString::from("World")};
    /// let hw_str = rumtk_serialize!(&hw);
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
            use $crate::serde::json::from_json;

            from_json($string)
        }};
    }
