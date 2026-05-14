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
use crate::core::{RUMResult, RUMVec, RUMVecDeque};
use crate::strings::{rumtk_format, RUMArrayConversions, RUMString};
pub use bytes::{BufMut, Bytes as RUMBuffer, BytesMut as RUMBufferMut};
use clap::builder::TypedValueParser;
use rand::{distr::Alphanumeric, RngExt};
use tokio::io::AsyncReadExt;

pub const DEFAULT_BUFFER_CHUNK_SIZE: usize = 1024;
pub const DEFAULT_BUFFER_ITEM_COUNT: usize = 1024;

pub struct RUMSliceSplitIter<'a, 'b> {
    pub remainder: &'a [u8],
    pub pattern: &'b [u8],
    pub last: usize,
    pub pattern_length: usize,
}

pub struct RUMSliceEnumerateIter<'a, 'b> {
    pub remainder: &'a [u8],
    pub pattern: &'b [u8],
    pub cummulative: usize,
    pub last: usize,
    pub pattern_length: usize,
}

pub struct RUMBufferSplitIter<'a> {
    pub remainder: RUMBuffer,
    pub pattern: &'a [u8],
    pub pattern_length: usize,
    pub last: usize,
}

pub trait RUMByteSliceSplitIterTrait {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

pub trait RUMByteSliceEnumeratorIterTrait {
    type Item;
    fn next(&mut self) -> Option<(usize, Self::Item)>;
}

pub trait RUMBufferSplitIterTrait {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

pub trait RUMByteSliceIteratorExt<'a, 'b> {
    fn split_fast(&'a self, pattern: &'b [u8]) -> RUMSliceSplitIter<'a, 'b>;
    fn enumerate_fast(&'a self, pattern: &'b [u8]) -> RUMSliceEnumerateIter<'a, 'b>;
}

pub trait RUMBufferIteratorExt<'a> {
    fn split_fast(self, pattern: &'a [u8]) -> RUMBufferSplitIter<'a>;
}

impl<'a, 'b> Iterator for RUMSliceSplitIter<'a, 'b> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        self.last = buffer_find(self.remainder, self.pattern);

        if self.remainder.len() > 0 {
            let r = Some(&self.remainder[..self.last]);
            let next = self.last + self.pattern_length;
            if next <= self.remainder.len() {
                self.remainder = &self.remainder[self.last + self.pattern_length..];
            } else {
                self.remainder = &self.remainder[self.last..];
            }
            r
        } else {
            None
        }
    }
}

impl<'a, 'b> Iterator for RUMSliceEnumerateIter<'a, 'b> {
    type Item = (usize, &'a [u8]);

    fn next(&mut self) -> Option<Self::Item> {
        self.last = buffer_find(self.remainder, self.pattern);
        self.cummulative += self.last;

        if self.remainder.len() > 0 {
            let r = Some((self.cummulative, &self.remainder[..self.last]));
            self.remainder = &self.remainder[self.last + self.pattern.len()..];
            r
        } else {
            None
        }
    }
}

impl<'a> Iterator for RUMBufferSplitIter<'a> {
    type Item = RUMBuffer;

    fn next(&mut self) -> Option<Self::Item> {
        self.last = buffer_find(&self.remainder, self.pattern);

        if self.remainder.len() > 0 {
            let v = self.remainder.split_to(self.last);
            if self.remainder.len() > self.pattern_length {
                let _ = self.remainder.split_to(self.pattern_length);
            }
            Some(v)
        } else {
            None
        }
    }
}

impl<'a, 'b> RUMByteSliceIteratorExt<'a, 'b> for &[u8] {
    fn split_fast(&'a self, pattern: &'b [u8]) -> RUMSliceSplitIter<'a, 'b> {
        RUMSliceSplitIter {
            pattern_length: pattern.len(),
            remainder: self.clone(),
            pattern: pattern.clone(),
            last: 0,
        }
    }

    fn enumerate_fast(&'a self, pattern: &'b [u8]) -> RUMSliceEnumerateIter<'a, 'b> {
        RUMSliceEnumerateIter {
            pattern_length: pattern.len(),
            remainder: self.clone(),
            pattern: pattern.clone(),
            cummulative: 0,
            last: 0,
        }
    }
}

impl<'a> RUMBufferIteratorExt<'a> for RUMBuffer {
    fn split_fast(self, pattern: &'a [u8]) -> RUMBufferSplitIter<'a> {
        RUMBufferSplitIter {
            pattern_length: pattern.len(),
            remainder: self,
            pattern: pattern.clone(),
            last: 0,
        }
    }
}

///
/// Convert slice of `&[u8]` to [RUMBuffer].
///
/// ## Example
/// ```
/// use rumtk_core::buffers::slice_to_buffer;
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
/// use rumtk_core::buffers::{new_random_buffer, DEFAULT_BUFFER_CHUNK_SIZE};
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
/// use rumtk_core::buffers::{new_random_buffer, DEFAULT_BUFFER_CHUNK_SIZE};
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
/// use rumtk_core::buffers::{new_random_string_buffer, DEFAULT_BUFFER_CHUNK_SIZE};
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
/// use rumtk_core::buffers::{new_random_string_set, DEFAULT_BUFFER_CHUNK_SIZE};
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

pub fn buffer_split_fast(mut input: RUMBuffer, pattern: u8) -> RUMVecDeque<RUMBuffer> {
    if input.is_empty() {
        return RUMVecDeque::new();
    }

    let mut item_list = RUMVecDeque::with_capacity(10);
    let mut offset = buffer_find_byte(input.as_slice(), pattern);

    while offset < input.len() {
        item_list.push_back(input.split_to(offset));
        input.split_to(1);
        offset = buffer_find_byte(input.as_slice(), pattern);
    }
    item_list.push_back(input);

    item_list
}

///
/// Convert buffer to string.
///
/// ## Example
/// ```
/// use rumtk_core::buffers::buffer_to_string;
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

pub fn buffer_count(buffer: &[u8], pattern: u8) -> usize {
    let instances = buffer.iter().filter(|c| *c != &pattern).collect::<Vec<&u8>>();

    instances.len()
}

pub fn buffer_chunk_find(chunk: &[u8], byte: u8) -> usize {
    for j in 0..chunk.len() {
        if chunk[j] == byte {
            return j;
        }
    }

    chunk.len()
}

pub fn buffer_find_byte(buffer: &[u8], byte: u8) -> usize {
    if buffer.is_empty() {
        return buffer.len();
    }

    let iter = buffer.chunks(256);
    for (i, chunk) in iter.enumerate() {
        if chunk.contains(&byte) {
            return (i * 256) + buffer_chunk_find(chunk, byte);
        }
    }

    buffer.len()
}

pub fn buffer_find(buffer: &[u8], pattern: &[u8]) -> usize {
    if buffer.is_empty() {
        return buffer.len();
    }

    let start_pattern_byte = pattern[0];
    let pattern_length = pattern.len();
    let mut working_buffer = buffer;
    let mut cumulative = 0;
    let mut end = 0;

    while (end + pattern_length) < working_buffer.len() {
        working_buffer = &working_buffer[end..];

        if working_buffer[..pattern_length] == *pattern {
            return cumulative;
        } else {
            working_buffer = &working_buffer[pattern_length..];
            cumulative += pattern_length;
        }

        end = buffer_find_byte(&working_buffer, start_pattern_byte);
        cumulative += end;
    }

    buffer.len()
}

pub fn buffer_find_instances<'a>(buffer: &'a [u8], pattern: &[u8]) -> RUMVec<(usize, &'a [u8])> {
    if buffer.is_empty() {
        return RUMVec::new();
    }

    let pattern_length = pattern.len();
    let buffer_length = buffer.len() - pattern_length;
    let mut instances = RUMVec::<(usize, &[u8])>::with_capacity(100);

    let mut cursor = buffer_find(buffer, pattern);
    let mut cumulative = cursor;
    let mut remainder = &buffer[..];

    while cumulative < buffer_length {
        instances.push((cumulative, &remainder[..cursor]));
        let next = cursor + pattern_length;
        if next <= remainder.len() {
            remainder = &remainder[cursor + pattern_length..];
            cursor = buffer_find(remainder, pattern);
            cumulative += cursor;
        } else {
            cumulative += remainder.len();
        }
    }

    instances
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

pub fn buffer_replace_in_place<'a>(buffer: &'a mut [u8], pattern: &[u8], replacement: &[u8]) {
    let replacement_length = replacement.len();
    let mut cursor = buffer_find(&buffer, pattern);
    let mut remainder = buffer;

    while cursor < remainder.len() {
        for i in 0..replacement_length {
            remainder[cursor + i] = replacement[i];
        }

        remainder = &mut remainder[cursor + pattern.len()..];
        cursor = buffer_find(remainder, pattern);
    }
}

pub fn buffer_replace(buffer: &[u8], pattern: &[u8], replacement: &[u8]) -> RUMBuffer {
    let pattern_length = pattern.len();
    let replacement_length = replacement.len();
    let instances = buffer_find_instances(&buffer, pattern);
    let mut new_buffer =  RUMBufferMut::with_capacity(buffer.len() + (instances.len() * (replacement_length)));
    let mut last = 0;

    for (indx, chunk) in instances {
        new_buffer.put(chunk);
        new_buffer.put(replacement);
        last = indx + pattern_length;
    }

    match new_buffer.is_empty() {
        true => RUMBuffer::copy_from_slice(buffer),
        false => {
            new_buffer.put(&buffer[last..]);
            new_buffer.freeze()
        }
    }
}

pub fn buffer_trim(buffer: &RUMBuffer) -> RUMBuffer {
    let trimmed = buffer.trim_ascii();

    if trimmed.len() < buffer.len() {
        RUMBuffer::copy_from_slice(trimmed)
    } else {
        buffer.clone()
    }
}

pub fn buffer_slice_trim(buffer: &[u8]) -> &[u8] {
    buffer.trim_ascii()
}

pub fn buffer_has_pattern(buffer: &[u8], pattern: &[u8]) -> bool {
    buffer_find(buffer, pattern) != buffer.len()
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

