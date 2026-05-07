/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2024  Luis M. Santos, M.D. <lsantos@medicalmasses.com>
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
use crate::core::{is_unique, RUMResult, RUMVec};
use crate::types::RUMBuffer;
use base64::prelude::*;
use chardetng::EncodingDetector;
use encoding_rs::Encoding;
use std::cmp::min;
pub use std::format as rumtk_format;
use unicode_segmentation::UnicodeSegmentation;
/**************************** Constants**************************************/
const ESCAPED_STRING_WINDOW: usize = 6;
const ASCII_ESCAPE_CHAR: char = '\\';
const MIN_ASCII_READABLE: char = ' ';
const MAX_ASCII_READABLE: char = '~';
pub const EMPTY_STRING: &str = "";
pub static EMPTY_RUMSTRING: RUMString = RUMString::default();
pub const DOT_STR: &str = ".";
pub const EMPTY_STRING_OPTION: Option<&str> = Some("");
pub const READABLE_ASCII: &str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

/**************************** Types *****************************************/
pub type RUMString = String;
pub type EscapeException<'a> = (&'a str, &'a str);
pub type EscapeExceptions<'a> = &'a [EscapeException<'a>];
pub type StringReplacementPair<'a> = [(&'a str, &'a str)];
pub type Grapheme<'a> = &'a str;
pub type GraphemeStringView<'a> = RUMVec<Grapheme<'a>>;
pub type GraphemePattern<'a> = &'a [Grapheme<'a>];
pub type GraphemeSlice<'b, 'a> = &'b [Grapheme<'a>];
pub type GraphemePatternPair<'a> = (GraphemePattern<'a>, GraphemePattern<'a>);

///
/// The equivalent to a `stringview` but at the grapheme level. Meaning, we can use this view to
/// iterate through a string at the full `UTF8` implementation
///
#[derive(Default, Debug, PartialEq, Clone)]
pub struct GraphemeStr<'a> {
    view: GraphemeStringView<'a>,
    start: usize,
    end: usize,
}

impl<'a> GraphemeStr<'a> {
    pub fn from(string: &'a str) -> Self {
        let view = string.graphemes(true).collect::<GraphemeStringView>();
        Self::from_view(view)
    }

    pub fn from_view(view: GraphemeStringView<'a>) -> Self {
        let start = 0;
        let end = view.len();
        Self { view, start, end }
    }

    pub fn at(&self, index: usize) -> Grapheme<'a> {
        self.view[index]
    }

    pub fn trim(&self, pattern: &GraphemePatternPair<'a>) -> Self {
        let (left_pattern, right_pattern) = pattern;
        self.trim_left(left_pattern).trim_right(right_pattern)
    }

    pub fn trim_left(&self, pattern: &GraphemePattern<'a>) -> Self {
        let new_offset = self.find(pattern, self.start);
        Self {
            view: self.view.clone(),
            start: new_offset,
            end: self.end,
        }
    }

    pub fn trim_right(&self, pattern: &GraphemePattern<'a>) -> Self {
        let new_offset = self.rfind(pattern, self.end);
        Self {
            view: self.view.clone(),
            start: self.start,
            end: new_offset,
        }
    }

    pub fn splice(&self, skip_pattern: &GraphemePatternPair<'a>) -> Self {
        let (left_pattern, right_pattern) = skip_pattern;
        let mut new_view = GraphemeStringView::with_capacity(self.end - self.start);
        let mut offset = self.start;
        let l_pattern_s = left_pattern.len();

        while offset < self.end {
            let target_s = self.find(left_pattern, offset) + l_pattern_s;
            for i in offset..target_s {
                new_view.push(self.view[i]);
            }
            offset = self.find(right_pattern, target_s);
        }

        GraphemeStr::from_view(new_view)
    }

    pub fn find(&self, pattern: &GraphemePattern<'a>, offset: usize) -> usize {
        let pattern_s = pattern.len();
        let mut new_offset = offset;
        let mut pattern_end = new_offset + pattern_s;

        while new_offset < self.end && pattern_end < self.end {
            if self.view[new_offset..pattern_end] == **pattern {
                break;
            }

            new_offset += 1;
            pattern_end = new_offset + pattern_s;
        }

        new_offset
    }

    pub fn rfind(&self, pattern: &GraphemePattern<'a>, offset: usize) -> usize {
        let pattern_s = pattern.len();
        let mut new_offset = offset;
        while new_offset > self.start {
            if self.view[new_offset - pattern_s..new_offset] == **pattern {
                break;
            }

            new_offset -= 1;
        }

        new_offset
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn get_graphemes(&self) -> GraphemeSlice<'_, 'a> {
        &self.view[self.start..self.end]
    }

    pub fn truncate(&self, size: usize) -> Self {
        let end = min(size, self.end);
        Self {
            view: self.view.clone(),
            start: self.start,
            end,
        }
    }

    pub fn is_unique(&self) -> bool {
        is_unique(&self.view)
    }
}

impl ToString for GraphemeStr<'_> {
    fn to_string(&self) -> String {
        let mut new_string = String::with_capacity(self.len());

        for grapheme in self.view[self.start..self.end].iter() {
            new_string.push_str(grapheme);
        }

        new_string
    }
}

impl RUMStringConversions for GraphemeStr<'_> {}

/**************************** Traits ****************************************/

pub trait StringLike {
    fn with_capacity(capacity: usize) -> Self;
    fn push_str(&mut self, string: &str);
}

pub trait AsStr {
    fn as_str(&self) -> &str;
    fn as_grapheme_str(&self) -> GraphemeStr {
        GraphemeStr::from(self.as_str())
    }
}

pub trait RUMStringConversions: ToString {
    #[inline(always)]
    fn to_raw(&self) -> RUMVec<u8> {
        self.to_string().as_bytes().to_vec()
    }

    #[inline(always)]
    fn to_buffer(&self) -> RUMBuffer {
        string_to_buffer(self.to_string().as_str())
    }
}

pub trait StringUtils: AsStr + RUMStringConversions {
    #[inline(always)]
    fn duplicate(&self, count: usize) -> RUMString {
        let mut duplicated = RUMString::with_capacity(count);
        for i in 0..count {
            duplicated += &self.as_str();
        }
        duplicated
    }

    fn truncate(&self, count: usize) -> RUMString {
        self.as_grapheme_str().truncate(count).to_string()
    }
}

impl AsStr for String {
    fn as_str(&self) -> &str {
        self.as_str()
    }
}

impl RUMStringConversions for RUMString {}
impl StringUtils for RUMString {}

impl RUMStringConversions for str {}

impl AsStr for str {
    fn as_str(&self) -> &str {
        self
    }
}

impl StringUtils for str {}

impl RUMStringConversions for char {}

pub trait RUMArrayConversions {
    fn to_string(&self) -> RUMResult<RUMString>;
}

impl RUMArrayConversions for Vec<u8> {
    #[inline(always)]
    fn to_string(&self) -> RUMResult<RUMString> {
        match RUMString::from_utf8(self.to_owned()) {
            Ok(s) => Ok(s),
            Err(e) => Err(rumtk_format!("Failure to parse incoming UTF-8 string: {}", e))
        }
    }
}

impl RUMArrayConversions for &[u8] {
    #[inline(always)]
    fn to_string(&self) -> RUMResult<RUMString> {
        match RUMString::from_utf8(self.to_vec()) {
            Ok(s) => Ok(s),
            Err(e) => Err(rumtk_format!("Failure to parse incoming UTF-8 string: {}", e))
        }
    }
}

/**************************** Helpers ***************************************/

pub fn count_tokens_ignoring_pattern(vector: &Vec<&str>, string_token: &RUMString) -> usize {
    let mut count: usize = 0;
    for tok in vector.iter() {
        if string_token != tok {
            count += 1;
        }
    }
    count
}

///
/// Implements decoding this string from its auto-detected encoding to UTF-8.
/// Failing that we assume the string was encoded in UTF-8 and return a copy.
///
/// Note => Decoding is facilitated via the crates chardet-ng and encoding_rs.
///
pub fn try_decode(src: &[u8]) -> RUMResult<RUMString> {
    let mut detector = EncodingDetector::new();
    detector.feed(&src, true);
    let encoding = detector.guess(None, true);
    decode(src, encoding)
}

///
/// Implements decoding this string from a specific encoding to UTF-8.
///
/// Note => Decoding is facilitated via the crates chardet-ng and encoding_rs.
///
pub fn try_decode_with(src: &[u8], encoding_name: &str) -> RUMResult<RUMString> {
    let encoding = match Encoding::for_label(encoding_name.as_bytes()) {
        Some(v) => v,
        None => return Ok(EMPTY_RUMSTRING.clone()),
    };
    decode(src, encoding)
}

///
/// Implements decoding of input with encoder.
///
/// Note => Decoding is facilitated via the crate encoding_rs.
///
fn decode(src: &[u8], encoding: &'static Encoding) -> RUMResult<RUMString> {
    Ok(match encoding.decode_without_bom_handling_and_without_replacement(&src) {
        Some(res) => RUMString::from(res),
        None => src.to_string()?,
    })
}

///
/// This function will scan through an escaped string and unescape any escaped characters.
/// We collect these characters as a byte vector.
/// Finally, we do a decode pass on the vector to re-encode the bytes **hopefully right** into a
/// valid UTF-8 string.
///
/// This function focuses on reverting the result of [escape], whose output is meant for HL7.
///
pub fn unescape_string(escaped_str: &str) -> RUMResult<RUMString> {
    let graphemes = escaped_str.graphemes(true).collect::<Vec<&str>>();
    let str_size = graphemes.len();
    let mut result: Vec<u8> = Vec::with_capacity(escaped_str.len());
    let mut i = 0;
    while i < str_size {
        let seq_start = graphemes[i];
        match seq_start {
            "\\" => {
                let escape_seq = get_grapheme_string(&graphemes, " ", i);
                let mut c = match unescape(&escape_seq) {
                    Ok(c) => c,
                    Err(_why) => Vec::from(escape_seq.as_bytes()),
                };
                result.append(&mut c);
                i += &escape_seq.as_grapheme_str().len();
            }
            _ => {
                result.append(&mut Vec::from(seq_start.as_bytes()));
                i += 1;
            }
        }
    }
    Ok(try_decode(result.as_slice())?)
}

///
/// Get the grapheme block and concatenate it into a newly allocated [`RUMString`].
///
pub fn get_grapheme_string<'a>(
    graphemes: &Vec<&'a str>,
    end_grapheme: &str,
    start_index: usize,
) -> RUMString {
    get_grapheme_collection(graphemes, end_grapheme, start_index).join("")
}

///
/// Return vector of graphemes from starting spot up until we find the end grapheme.
///
/// Because a grapheme may take more than one codepoint characters, these have to be treated as
/// references to strings.
///
pub fn get_grapheme_collection<'a>(
    graphemes: &Vec<&'a str>,
    end_grapheme: &str,
    start_index: usize,
) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::new();
    for grapheme in graphemes.iter().skip(start_index) {
        let item = *grapheme;
        if item == end_grapheme {
            break;
        }
        result.push(item);
    }
    result
}

///
/// Turn escaped character sequence into the equivalent UTF-8 character
/// This function accepts \o, \x and \u formats.
/// This function will also attempt to unescape the common C style control characters.
/// Anything else needs to be expressed as hex or octal patterns with the formats above.
///
/// If I did this right, I should get the "raw" byte sequence out of the escaped string.
/// We can then use the bytes and attempt a decode() to figure out the string encoding and
/// get the correct conversion to UTF-8. **Fingers crossed**
///
pub fn unescape(escaped_str: &str) -> Result<Vec<u8>, RUMString> {
    let lower_case = escaped_str.to_lowercase();
    let mut bytes: Vec<u8> = Vec::with_capacity(3);
    match &lower_case[0..2] {
        // Hex notation case. Assume we are getting xxyy bytes
        "\\x" => {
            let byte_str = number_to_char_unchecked(&hex_to_number(&lower_case[2..6])?);
            bytes.append(&mut byte_str.as_bytes().to_vec());
        }
        // Unicode notation case, we need to do an extra step or we will lose key bytes.
        "\\u" => {
            let byte_str = number_to_char_unchecked(&hex_to_number(&lower_case[2..6])?);
            bytes.append(&mut byte_str.as_bytes().to_vec());
        }
        // Single byte notation case
        "\\c" => {
            let byte_str = number_to_char_unchecked(&hex_to_number(&lower_case[2..6])?);
            bytes.append(&mut byte_str.as_bytes().to_vec());
        }
        // Unicode notation case
        "\\o" => {
            let byte_str = number_to_char_unchecked(&octal_to_number(&lower_case[2..6])?);
            bytes.append(&mut byte_str.as_bytes().to_vec());
        }
        // Multibyte notation case
        "\\m" => match lower_case.as_grapheme_str().len() {
            8 => {
                bytes.push(hex_to_byte(&lower_case[2..4])?);
                bytes.push(hex_to_byte(&lower_case[4..6])?);
                bytes.push(hex_to_byte(&lower_case[6..8])?);
            }
            6 => {
                bytes.push(hex_to_byte(&lower_case[2..4])?);
                bytes.push(hex_to_byte(&lower_case[4..6])?);
            }
            _ => {
                return Err(rumtk_format!(
                    "Unknown multibyte sequence. Cannot decode {}",
                    lower_case
                ))
            }
        },
        // Custom encoding
        "\\z" => bytes.append(&mut lower_case.as_bytes().to_vec()),
        // Single byte codes.
        _ => bytes.push(unescape_control_byte(&lower_case[0..2])?),
    }
    Ok(bytes)
}

///
/// Unescape basic character
/// We use pattern matching to map the basic escape character to its corresponding integer value.
///
fn unescape_control(escaped_str: &str) -> Result<char, RUMString> {
    match escaped_str {
        // Common control sequences
        "\\t" => Ok('\t'),
        "\\b" => Ok('\x08'),
        "\\n" => Ok('\n'),
        "\\r" => Ok('\r'),
        "\\f" => Ok('\x14'),
        "\\s" => Ok('\x20'),
        "\\\\" => Ok(ASCII_ESCAPE_CHAR),
        "\\'" => Ok('\''),
        "\\\"" => Ok('"'),
        "\\0" => Ok('\0'),
        "\\v" => Ok('\x0B'),
        "\\a" => Ok('\x07'),
        // Control sequences by
        _ => Err(rumtk_format!(
            "Unknown escape sequence? Sequence: {}!",
            escaped_str
        )),
    }
}

///
/// Unescape basic character
/// We use pattern matching to map the basic escape character to its corresponding integer value.
///
fn unescape_control_byte(escaped_str: &str) -> Result<u8, RUMString> {
    match escaped_str {
        // Common control sequences
        "\\t" => Ok(9),   // Tab/Character Tabulation
        "\\b" => Ok(8),   // Backspace
        "\\n" => Ok(10),  // New line/ Line Feed character
        "\\r" => Ok(13),  // Carriage Return character
        "\\f" => Ok(12),  // Form Feed
        "\\s" => Ok(32),  // Space
        "\\\\" => Ok(27), // Escape
        "\\'" => Ok(39),  // Single quote
        "\\\"" => Ok(34), // Double quote
        "\\0" => Ok(0),   // Null character
        "\\v" => Ok(11),  // Vertical Tab/Line Tabulation
        "\\a" => Ok(7),   // Alert bell
        // Control sequences by hex
        //Err(rumtk_format!("Unknown escape sequence? Sequence: {}!", escaped_str))
        _ => hex_to_byte(escaped_str),
    }
}

///
/// Turn hex string to number (u32)
///
fn hex_to_number(hex_str: &str) -> Result<u32, RUMString> {
    match u32::from_str_radix(&hex_str, 16) {
        Ok(result) => Ok(result),
        Err(val) => Err(rumtk_format!(
            "Failed to parse string with error {}! Input string {} \
        is not hex string!",
            val,
            hex_str
        )),
    }
}

///
/// Turn hex string to byte (u8)
///
fn hex_to_byte(hex_str: &str) -> Result<u8, RUMString> {
    match u8::from_str_radix(&hex_str, 16) {
        Ok(result) => Ok(result),
        Err(val) => Err(rumtk_format!(
            "Failed to parse string with error {}! Input string {} \
        is not hex string!",
            val,
            hex_str
        )),
    }
}

///
/// Turn octal string to number (u32)
///
fn octal_to_number(hoctal_str: &str) -> Result<u32, RUMString> {
    match u32::from_str_radix(&hoctal_str, 8) {
        Ok(result) => Ok(result),
        Err(val) => Err(rumtk_format!(
            "Failed to parse string with error {}! Input string {} \
        is not an octal string!",
            val,
            hoctal_str
        )),
    }
}

///
/// Turn octal string to byte (u32)
///
fn octal_to_byte(hoctal_str: &str) -> Result<u8, RUMString> {
    match u8::from_str_radix(&hoctal_str, 8) {
        Ok(result) => Ok(result),
        Err(val) => Err(rumtk_format!(
            "Failed to parse string with error {}! Input string {} \
        is not an octal string!",
            val,
            hoctal_str
        )),
    }
}

///
/// Turn number to UTF-8 char
///
fn number_to_char(num: &u32) -> Result<RUMString, RUMString> {
    match char::from_u32(*num) {
        Some(result) => Ok(result.to_string()),
        None => Err(rumtk_format!(
            "Failed to cast number to character! Number {}",
            num
        )),
    }
}

///
/// Turn number to UTF-8 char. Normally, calling from_u32 checks if the value is a valid character.
/// This version uses the less safe from_u32_unchecked() function because we want to get the bytes
/// and deal with validity at a higher layer.
///
fn number_to_char_unchecked(num: &u32) -> RUMString {
    unsafe { char::from_u32_unchecked(*num).to_string() }
}

///
/// Turn UTF-8 character into escaped character sequence as expected in HL7
///
/// # Example
/// ```
///  use rumtk_core::strings::{escape};
///  let message = "I ❤ my wife!";
///  let escaped_message = escape(&message);
///  assert_eq!("I \\u2764 my wife!", &escaped_message, "Did not get expected escaped string! Got {}!", &escaped_message);
///```
///
pub fn escape(unescaped_str: &str) -> RUMString {
    basic_escape(unescaped_str, &vec![("{", ""), ("}", "")])
}

///
/// Escape UTF-8 characters in UTF-8 string that are beyond ascii range
///
/// # Example
/// ```
///  use rumtk_core::strings::basic_escape;
///  let message = "I ❤ my wife!";
///  let escaped_message = basic_escape(&message, &vec![]);
///  assert_eq!("I \\u{2764} my wife!", &escaped_message, "Did not get expected escaped string! Got {}!", &escaped_message);
///```
pub fn basic_escape(unescaped_str: &str, except: EscapeExceptions) -> RUMString {
    let escaped = is_escaped_str(unescaped_str);
    if !escaped {
        let mut escaped_str = unescaped_str.escape_default().to_string();
        for (from, to) in except {
            escaped_str = escaped_str.replace(from, to);
        }
        return escaped_str.to_string();
    }
    unescaped_str.to_string()
}

///
/// Checks if a given string is fully ASCII or within the ASCII range.
///
/// Remember: all strings are UTF-8 encoded in Rust, but most ASCII strings fit within the UTF-8
/// encoding scheme.
///
pub fn is_ascii_str(unescaped_str: &str) -> bool {
    unescaped_str.is_ascii()
}

///
/// Checks if an input string is already escaped.
/// The idea is to avoid escaping the escaped string thus making it a nightmare to undo the
/// escaping later on.
///
/// Basically, if you were to blindly escape the input string, back slashes keep getting escaped.
/// For example `\r -> \\r -> \\\\r -> ...`.
///
pub fn is_escaped_str(unescaped_str: &str) -> bool {
    if !is_ascii_str(unescaped_str) {
        return false;
    }

    for c in unescaped_str.chars() {
        if !is_printable_char(&c) {
            return false;
        }
    }
    true
}

///
/// Returns whether a character is in the ASCII printable range.
///
pub fn is_printable_char(c: &char) -> bool {
    &MIN_ASCII_READABLE <= c && c <= &MAX_ASCII_READABLE
}

///
/// Removes all non ASCII and all non printable characters from string.
///
pub fn filter_ascii(unescaped_str: &str, closure: fn(char) -> bool) -> RUMString {
    let mut filtered = unescaped_str.to_string();
    filtered.retain(closure);
    filtered
}

///
/// Removes all non ASCII and all non printable characters from string.
///
pub fn filter_non_printable_ascii(unescaped_str: &str) -> RUMString {
    filter_ascii(unescaped_str, |c: char| is_printable_char(&c))
}

///
/// Convert buffer to string.
///
/// ## Example
/// ```
/// use rumtk_core::strings::{buffer_to_string, string_to_buffer};
/// use rumtk_core::types::RUMBuffer;
///
/// const expected: &str = "Hello World!";
/// let buffer = RUMBuffer::from_static(expected.as_bytes());
/// let result = string_to_buffer(expected);
///
/// assert_eq!(result, expected, "str to RUMBuffer conversion failed!");
/// ```
///
pub fn string_to_buffer(data: &str) -> RUMBuffer {
    RUMBuffer::copy_from_slice(data.as_bytes())
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

pub fn buffer_has_pattern(buffer: &[u8], pattern: &[u8]) -> bool {
    let buffer_length = buffer.len();
    let pattern_length = pattern.len();
    let mut has_pattern = false;

    for i in 0..buffer_length {
        if (i + pattern_length) <= buffer_length
        {
            for j in 0..pattern_length {
                has_pattern = has_pattern || buffer[i + j] != pattern[j]
            }

            if !has_pattern {
                break;
            }
        }
    }

    has_pattern
}

///
/// Given a set of keys and replacements, transform the input string.
///
/// ## Example
/// ```
/// use rumtk_core::strings::string_format;
/// use rumtk_core::types::RUMBuffer;
///
/// const expected: &str = "Hello World!";
/// const template: &str = "Hello {}!";
/// let result = string_format(template, &[("{}", "World")]);
///
/// assert_eq!(result.as_str(), expected, "Formatting of string failed!");
/// ```
///
pub fn string_format(input: &str, formatting: &StringReplacementPair) -> RUMString {
    let mut output = String::from(input);

    for item in formatting.iter() {
        output = output.as_str().replace(item.0, item.1);
    }

    output.to_string()
}

///
/// Convenience function for transforming a string into a `base64` encoded string.
///
/// ## Example
/// ```
///
/// ```
///
pub fn string_to_b64(data: &str) -> String {
    BASE64_STANDARD.encode(data)
}

///
/// Convenience function for transforming a `base64` encoded string back to its original form.
///
/// ## Example
/// ```
/// ```
///
pub fn b64_to_string(data: &String) -> RUMResult<RUMVec<u8>> {
    match BASE64_STANDARD.decode(data) {
        Ok(result) => Ok(result),
        Err(e) => Err(rumtk_format!("Failed to decode base64 string: {}", e)),
    }
}