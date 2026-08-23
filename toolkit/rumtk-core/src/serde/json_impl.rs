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
pub use crate::serde::json::*;
pub use crate::types::RUMOrderedMap;
use std::mem::ManuallyDrop;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct RUMSerializableManualDrop<T>(pub ManuallyDrop<T>);

impl<T> RUMSerializableManualDrop<T> {
    pub fn new(v: T) -> Self {
        RUMSerializableManualDrop(ManuallyDrop::new(v))
    }

    pub fn inner(&self) -> &T {
        &self.0
    }
}

impl<T> RUMSerJson for RUMSerializableManualDrop<T>
where
    T: RUMSerJson + Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: RUMJsonSerializer,
    {
        self.inner().serialize(serializer)
    }
}

impl<'a, T: RUMDeJson<'a>> RUMDeJson<'a> for RUMSerializableManualDrop<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D>::Error>
    where
        D: RUMJsonDeserializer<'a>,
    {
        let escaped_val = T::deserialize(deserializer)?;
        Ok(RUMSerializableManualDrop(ManuallyDrop::new(escaped_val)))
    }
}
