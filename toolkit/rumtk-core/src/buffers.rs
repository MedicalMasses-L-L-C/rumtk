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
use crate::core::{RUMResult, RUMVec};
use crate::strings::{rumtk_format, RUMArrayConversions, RUMString};
pub use bytes::{BufMut, Bytes as RUMBuffer, BytesMut as RUMBufferMut};
use clap::builder::TypedValueParser;
use rand::{distr::Alphanumeric, RngExt};
use tokio::io::AsyncReadExt;

pub const DEFAULT_BUFFER_CHUNK_SIZE: usize = 1024;
pub const DEFAULT_BUFFER_ITEM_COUNT: usize = 1024;

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

///
/// Convert buffer to string.
///
/// ## Example
/// ```
/// use rumtk_core::strings::buffer_to_string;
/// use rumtk_core::types::RUMBuffer;
///
/// const expected: &str = "Hello World!";
/// let buffer = RUMBuffer::from_static(expected.as_bytes());
/// let result = buffer_to_string(&buffer).unwrap();
///
/// assert_eq!(result, expected, "Buffer to RUMString conversion failed!");
/// ```
///
pub fn buffer_to_string(buffer: &[u8]) -> RUMResult<RUMString> {
    match buffer.to_string() {
        Ok(string) => Ok(string),
        Err(e) => Err(rumtk_format!("Failure to parse incoming UTF-8 string: {}", e)),
    }
}

pub fn buffer_to_str(buffer: &[u8]) -> RUMResult<&str> {
    match std::str::from_utf8(buffer) {
        Ok(string) => Ok(string),
        Err(e) => Err(rumtk_format!("Failure to parse incoming UTF-8 string: {}", e)),
    }
}

pub fn buffer_find(buffer: &[u8], pattern: &[u8], offset: usize) -> usize {
    let buffer_length = buffer.len();
    let pattern_length = pattern.len();

    for i in offset..buffer_length {
        if (i + pattern_length) <= buffer_length
        {
            let mut matches = true;
            for j in 0..pattern_length {
                matches = matches && buffer[i + j] == pattern[j]
            }

            if matches {
                return i;
            }
        }
    }

    usize::MAX
}

pub fn buffer_pad(buffer: &[u8], pad: u8, target_length: usize) -> RUMBuffer {
    let buffer_length = buffer.len();
    let pad_length = target_length - buffer_length;
    let s = buffer_length + pad_length;
    let mut slice = RUMBufferMut::with_capacity(s);

    slice.put(buffer);

    for _ in buffer_length..s {
        slice.put_u8(pad);
    }

    slice.freeze()
}

pub fn buffer_replace<'a>(buffer: &RUMBuffer, pattern: &[u8], replacement: &[u8]) -> RUMBuffer {
    let mut start = buffer_find(buffer.as_slice(), pattern, 0);
    let mut last = 0;
    let mut new_buffer =  RUMBufferMut::with_capacity(buffer.len());

    while start < buffer.len() {
        new_buffer.put(&buffer[last ..start]);
        last = start;

        new_buffer.put(replacement);
        start = buffer_find(buffer.as_slice(), pattern, start + replacement.len());
    }

    new_buffer.freeze()
}

pub fn buffer_has_pattern(buffer: &[u8], pattern: &[u8]) -> bool {
    buffer_find(buffer, pattern, 0) != usize::MAX
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

