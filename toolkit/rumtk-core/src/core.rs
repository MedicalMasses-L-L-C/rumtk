/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D. <lsantos@medicalmasses.com>
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
use crate::strings::rumtk_format;
use crate::strings::RUMString;
use crate::types::RUMBuffer;
use rand::{distr::Alphanumeric, RngExt};

pub const DEFAULT_BUFFER_CHUNK_SIZE: usize = 1024;
pub const DEFAULT_BUFFER_ITEM_COUNT: usize = 1024;

pub type RUMError = RUMString;

///
/// Type used for propagating error messages.
///
pub type RUMResult<T> = Result<T, RUMError>;

pub type RUMVec<T> = Vec<T>;

pub fn is_unique<T: std::cmp::Eq + std::hash::Hash>(data: &Vec<T>) -> bool {
    let mut keys = ahash::AHashSet::with_capacity(data.len());
    for itm in data {
        if !keys.insert(itm) {
            return false;
        }
    }
    true
}

pub fn is_unique_bytes(data: &[u8]) -> bool {
    let mut items = ahash::AHashSet::with_capacity(data.len());
    for i in 0..data.len() {
        if !items.insert(data[i]) {
            return false;
        }
    }
    true
}

///
/// Take a requested index and the maximum size of the item container.
/// Check if the index is valid and return an error if it is.
/// The purpose of this function is to enable handling of out of bounds without triggering a panic.
/// Also, add negative indices like Python does when doing a reverse search!
///
/// * If the index is 0, return Error
/// * If the index is below 0, return the max - index iff max - index > 0
/// * If the index is bigger than the defined max, return Error.
/// * Otherwise, return the given index.
///
/// # Examples
///
/// ## Min
/// ```
/// use ::rumtk_core::core::clamp_index;
/// use ::rumtk_core::strings::rumtk_format;
/// let max: isize = 5;
/// let i: isize = 1;
/// let result = clamp_index(&i, &max).unwrap();
/// assert_eq!(&1, &result, "{}", rumtk_format!("Expected to receive 0 but got {}", &result))
/// ```
///
/// ## Max
/// ```
/// use ::rumtk_core::core::clamp_index;
/// use ::rumtk_core::strings::rumtk_format;
/// let max: isize = 5;
/// let i: isize = 5;
/// let result = clamp_index(&i, &max).unwrap();
/// assert_eq!(&5, &result, "{}", rumtk_format!("Expected to receive 0 but got {}", &result))
/// ```
///
/// ## Valid
/// ```
/// use ::rumtk_core::core::clamp_index;
/// use ::rumtk_core::strings::rumtk_format;
/// let max: isize = 5;
/// let i: isize = 5;
/// let result = clamp_index(&i, &max).unwrap();
/// assert_eq!(&5, &result, "{}", rumtk_format!("Expected to receive 0 but got {}", &result))
/// ```
///
/// ## Valid Negative Index (reverse lookup)
/// ```
/// use ::rumtk_core::core::clamp_index;
/// use ::rumtk_core::strings::rumtk_format;
/// let max: isize = 5;
/// let i: isize = -1;
/// let result = clamp_index(&i, &max).unwrap();
/// assert_eq!(&5, &result, "{}", rumtk_format!("Expected to receive 0 but got {}", &result))
/// ```
#[inline(always)]
pub fn clamp_index(given_indx: &isize, max_size: &isize) -> RUMResult<usize> {
    let neg_max_indx = *max_size * -1;
    if *given_indx == 0 {
        return Err(rumtk_format!(
            "Index {} is invalid! Use 1-indexed values if using positive indices.",
            given_indx
        ));
    }

    if *given_indx >= neg_max_indx && *given_indx < 0 {
        return Ok((max_size + given_indx + 1) as usize);
    }

    if *given_indx > 0 && given_indx <= max_size {
        return Ok(*given_indx as usize);
    }

    Err(rumtk_format!(
        "Index {} is outside {} < x < {} boundary!",
        given_indx,
        neg_max_indx,
        max_size
    ))
}

///
/// Convert slice of `&[u8]` to [RUMBuffer].
///
/// ## Example
/// ```
/// use rumtk_core::core::slice_to_buffer;
/// use rumtk_core::types::RUMBuffer;
///
/// const expected: &str = "Hello World!";
/// let buffer = RUMBuffer::from_static(expected.as_bytes());
/// let result = slice_to_buffer(expected.as_bytes());
///
/// assert_eq!(result, buffer, "Slice to RUMBuffer conversion failed!");
/// ```
///
pub fn slice_to_buffer(buffer: &[u8]) -> RUMBuffer {
    RUMBuffer::copy_from_slice(buffer)
}

///
/// Generates a new random buffer using the `rand` crate and wrapped inside a [RUMBuffer](RUMBuffer).
/// 
/// The buffer size can be adjusted via the turbofish method => `new_random_buffer::<10>()`.
///
/// ## Example
///
/// ```
/// use rumtk_core::core::{new_random_buffer, DEFAULT_BUFFER_CHUNK_SIZE};
///
/// let buffer = new_random_buffer::<DEFAULT_BUFFER_CHUNK_SIZE>();
///
/// assert_eq!(buffer.is_empty(), false, "Function returned an empty random buffer which was unexpected!");
/// assert_eq!(buffer.len(), DEFAULT_BUFFER_CHUNK_SIZE, "The new random buffer does not have the expected size!");
/// ```
///
pub fn new_random_buffer<const N: usize>() -> [u8; N] {
    let mut buffer = [0u8; N];
    rand::fill(&mut buffer);
    buffer
}

///
/// Generates a new random buffer using the `rand` crate and wrapped inside a [RUMBuffer](RUMBuffer).
///
/// The buffer size can be adjusted via the turbofish method => `new_random_buffer::<10>()`.
///
/// ## Example
///
/// ```
/// use rumtk_core::core::{new_random_buffer, DEFAULT_BUFFER_CHUNK_SIZE};
///
/// let buffer = new_random_buffer::<DEFAULT_BUFFER_CHUNK_SIZE>();
///
/// assert_eq!(buffer.is_empty(), false, "Function returned an empty random buffer which was unexpected!");
/// assert_eq!(buffer.len(), DEFAULT_BUFFER_CHUNK_SIZE, "The new random buffer does not have the expected size!");
/// ```
///
pub fn new_random_rumbuffer<const N: usize>() -> RUMBuffer {
    slice_to_buffer(&new_random_buffer::<N>())
}

///
/// Generates a new random string using the `rand` crate and wrapped inside a [RUMString](RUMString).
///
/// The buffer size can be adjusted via the turbofish method => `new_random_string_buffer::<10>()`.
///
/// ## Example
///
/// ```
/// use rumtk_core::core::{new_random_string_buffer, DEFAULT_BUFFER_CHUNK_SIZE};
///
/// let buffer = new_random_string_buffer::<DEFAULT_BUFFER_CHUNK_SIZE>();
///
/// assert_eq!(buffer.is_empty(), false, "Function returned an empty random buffer which was unexpected!");
/// assert_eq!(buffer.len(), DEFAULT_BUFFER_CHUNK_SIZE, "The new random buffer does not have the expected size!");
/// ```
///
pub fn new_random_string_buffer<const N: usize>() -> RUMString {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(N) // Length of the string
        .map(char::from)
        .collect()
}

///
/// Generates a new random set of [RUMString] using the `rand` crate.
///
/// The buffer size for each item can be adjusted via the turbofish method => `new_random_string_set::<10>()`.
///
/// ## Example
///
/// ```
/// use rumtk_core::core::{new_random_string_set, DEFAULT_BUFFER_CHUNK_SIZE};
///const item_count: usize = 5;
/// 
/// let buffer = new_random_string_set::<DEFAULT_BUFFER_CHUNK_SIZE>(item_count);
///
/// assert_eq!(buffer.is_empty(), false, "Function returned an empty random buffer which was unexpected!");
/// assert_eq!(buffer.len(), item_count, "The new random buffer does not have the expected item count!");
/// assert_eq!(buffer.get(0).unwrap().len(), DEFAULT_BUFFER_CHUNK_SIZE, "The new random buffer does not have the expected size!");
/// ```
///
pub fn new_random_string_set<const N: usize>(item_count: usize) -> RUMVec<RUMString> {
    let mut set = RUMVec::<RUMString>::with_capacity(item_count);
    
    for _ in 0..item_count {
        set.push(new_random_string_buffer::<N>())
    }
    
    set
}

pub fn split_buffer(mut input: RUMBuffer, separator: u8) -> RUMVec<RUMBuffer> {
    let mut item_list = RUMVec::<RUMBuffer>::with_capacity(100);
    for mut i in 0..input.len() {
        if input[i] == separator {
            let component = input.split_to(i);
            item_list.push(component);

            // Let's consume the separator character so it does not show in any buffers.
            i += 1;
            let _ = input.split_to(i);
        }
    }

    item_list
}
