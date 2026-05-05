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

pub mod rumtk_search {
    use crate::cache::{new_cache, LazyRUMCache};
    use crate::core::RUMResult;
    use crate::rumtk_cache_fetch;
    use crate::strings::{rumtk_format, RUMString};
    use crate::types::RUMHashMap;
    use regex::Regex;
    use std::fmt::Debug;
    use std::str::FromStr;
    /**************************** Globals **************************************/
    static mut re_cache: RegexCache = new_cache();
    /**************************** Constants**************************************/
    const DEFAULT_REGEX_CACHE_PAGE_SIZE: usize = 10;
    /**************************** Types *****************************************/
    pub type RegexCache = LazyRUMCache<RUMString, Regex>;
    pub type SearchGroups = RUMHashMap<RUMString, RUMString>;
    pub type CapturedList = Vec<RUMString>;

    /**************************** Traits ****************************************/

    /**************************** Helpers ***************************************/
    fn compile_regex(expr: &str) -> RUMResult<Regex> {
        match Regex::new(expr) {
            Ok(regex) => Ok(regex),
            Err(e) => Err(rumtk_format!("Invalid regex => {}", e))
        }
    }

    ///
    /// Finds all of the named regex captures and generates a hash table with the results assorted
    /// into key-value pairs. The keys are the names found in the regex expression. The value is
    /// the match corresponding to the named capture.
    ///
    /// This function returns an instance of SearchGroup which is the hash map.
    ///
    pub fn string_search_named_captures(input: &str, expr: &str, default: &str) -> RUMResult<SearchGroups> {
        let key = RUMString::from(expr);
        let re: Regex = rumtk_cache_fetch!(&raw mut re_cache, &key, || {compile_regex(expr)})?;
        let names: Vec<&str> = re
            .capture_names()
            .skip(1)
            .map(|x| x.unwrap_or(""))
            .collect();
        let mut clean_names: Vec<&str> = Vec::with_capacity(names.len());
        let mut groups = SearchGroups::with_capacity(DEFAULT_REGEX_CACHE_PAGE_SIZE);

        for name in &names {
            if !name.is_empty() {
                clean_names.push(name);
            }
        }

        if clean_names.is_empty() {
            return Ok(groups);
        }

        for name in &clean_names {
            groups.insert(RUMString::from(name.to_string()), RUMString::from(default));
        }

        for cap in re.captures_iter(input).map(|c| c) {
            for name in &clean_names {
                let val = cap.name(name).map_or("", |s| s.as_str());
                if !val.is_empty() {
                    groups.insert(RUMString::from(name.to_string()), RUMString::from(val));
                }
            }
        }

        Ok(groups)
    }

    ///
    /// Finds all of the regex captures regardless of name status and compile them into a list
    /// of strings. Elsewhere, this provides a simple way to iterate through the contents that
    /// were inside a group \(\).
    ///
    /// This function returns an instance of CapturedList which is the list of strings.
    ///
    pub fn string_search_all_captures(input: &str, expr: &str, default: &str) -> RUMResult<CapturedList> {
        let key = RUMString::from(expr);
        let re: Regex = rumtk_cache_fetch!(&raw mut re_cache, &key, || {compile_regex(expr)})?;
        let mut capture_list = CapturedList::with_capacity(DEFAULT_REGEX_CACHE_PAGE_SIZE);

        for caps in re.captures_iter(input) {
            for c in caps.iter().skip(1) {
                let c_str = c.unwrap().as_str();
                capture_list.push(RUMString::from(c_str));
            }
        }

        Ok(capture_list)
    }

    ///
    /// Given a string input and a compiled RegEx, look for all matches and put them in a string
    /// list for easy iteration/access.
    ///
    pub fn string_list(input: &str, re: &Regex) -> CapturedList {
        let mut list: Vec<RUMString> = Vec::with_capacity(DEFAULT_REGEX_CACHE_PAGE_SIZE);
        for itm in re.find_iter(input) {
            list.push(RUMString::from(itm.as_str()));
        }
        list
    }

    ///
    /// Given a string input and a RegEx string,
    /// ```text
    ///     - Compile the regex if not done so already.
    ///     - Do a string search for all regex matches.
    ///     - Collapse/join the matches into a single output string using join_pattern as the join fragment.
    /// ```
    /// Use \" \" in join_pattern if you wish to have spaces in between matches.
    ///
    pub fn string_search(input: &str, expr: &str, join_pattern: &str) -> RUMResult<RUMString> {
        Ok(string_search_list(input, expr)?.join(join_pattern))
    }

    ///
    /// Search for pattern and return all matches.
    ///
    pub fn string_search_list(input: &str, expr: &str) -> RUMResult<CapturedList> {
        let key = RUMString::from(expr);
        let re: Regex = rumtk_cache_fetch!(&raw mut re_cache, &key, || {compile_regex(expr)})?;
        Ok(string_list(input, &re))
    }

    ///
    /// Given a string input and a set of RegEx patterns, find the target value and return it as
    /// the given target type `T`.
    ///
    /// ```
    /// use rumtk_core::search::rumtk_search::string_find_value;
    ///
    /// let haystack = "Range (min \\xe2\\x80\\xa6 max):     0.6 ms \\xe2\\x80\\xa6   2.9 ms    1273 runs";
    /// let patterns = ["\\d+ runs", "\\d+"];
    /// let expected = 1273;
    /// let result = string_find_value::<usize>(haystack, &patterns);
    ///
    /// assert_eq!(result, Ok(expected), "Did not find the needle in the haystack or returned the wrong type!");
    /// ```
    /// Use \" \" in join_pattern if you wish to have spaces in between matches.
    ///
    pub fn string_find_value<T: Default + FromStr>(input: &str, patterns: &[&str]) -> RUMResult<T> {
        let mut haystack = input;
        let mut needle = RUMString::default();
        let mut result = T::default();

        for expr in patterns {
            needle = string_search(haystack, expr, " ")?;
            haystack = &needle;
        }

        result = needle.trim().parse::<T>().unwrap_or_default();
        Ok(result)
    }

    ///
    /// Search for pattern and replace all matches.
    ///
    pub fn string_replace_all_matches(input: &str, expr: &str, replacement: &str) -> RUMResult<String> {
        let matches = string_search_list(input, expr)?;
        let mut result = String::from(input);

        for pattern in matches.iter() {
            result = result.as_str().replace(pattern.as_str(), replacement.as_str());
        }

        Ok(result)
    }
}
