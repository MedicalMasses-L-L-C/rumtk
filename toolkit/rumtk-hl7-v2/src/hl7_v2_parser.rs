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

///
/// The V2 Parser module will contain a simple and lightweight message parser that will generate a
/// structure following the message structure in the HL7 Specifications.
/// The V2Message type here will provide a basic interface for navigating through the mapped
/// segments and fields.
/// From here, we will then write a schema driven interpreter module (see other source files in
/// crate). That interpreter will try to generate a message structure using the specified HL7
/// types. That structure will be exportable to JSON and (maybe) XML.
///
/// [Conformance](https://v2.hl7.org/conformance/HL7v2_Conformance_Methodology_R1_O1_Ballot_Revised_D9_-_September_2019_Introduction.html#:~:text=The%20base%20HL7%20v2%20standard,message%20definition%20is%20called%20profiling.)
///
/// [Product Brief](https://www.hl7.org/implement/standards/product_brief.cfm?product_id=185)
///

pub mod v2_parser {
    use pyo3::prelude::*;
    use std::io::BufRead;

    pub use crate::hl7_v2_base_types::v2_primitives::{
        V2DateTime, V2ParserCharacters, V2PrimitiveCasting, V2Result, V2SearchIndex, V2String,
    };
    pub use crate::hl7_v2_constants::{
        V2_DELETE_FIELD, V2_EMPTY_STRING, V2_MSHEADER_PATTERN, V2_SEGMENT_DESC, V2_SEGMENT_IDS,
        V2_SEGMENT_TERMINATOR,
    };
    use crate::hl7_v2_constants::{V2_MSHEADER_PATTERN_STR, V2_SEGMENT_TERMINATORS};
    use pyo3::exceptions::PyValueError;
    use rumtk_core::buffers::{buffer_replace, buffer_replace_in_place, buffer_slice_trim, buffer_split_fast, buffer_to_str, buffer_to_string, buffer_trim, RUMBufferIteratorExt, RUMByteSliceIteratorExt, DEFAULT_CPU_CACHE_LINE_SIZE, DEFAULT_CPU_PAGE_SIZE};
    use rumtk_core::cache::{new_cache, LazyRUMCache};
    use rumtk_core::base::{clamp_index, RUMError};
    use rumtk_core::base::{RUMResult, RUMVecDeque};
    use rumtk_core::rumtk_cache_fetch;
    use rumtk_core::scripting::python_utils::RUMPyResult;
    use rumtk_core::serde::compatibility::{RUMSerializableBuffer, RUMSerializableOrderedMap};
    use rumtk_core::serde::json::{RUMDeJson, RUMSerJson};
    pub use rumtk_core::strings::{
        rumtk_format, try_decode_with, unescape_string, AsStr, RUMString, RUMStringConversions,
    };
    use rumtk_core::strings::{string_to_buffer, AsString};
    use rumtk_core::types::{RUMBuffer, RUMBufferMut, RUMOrderedMap};
    use std::ops::{Index, IndexMut};
    use std::str::Chars;
    use std::sync::Arc;
    /**************************** Globals ***************************************/

    static mut search_cache: LazyRUMCache<RUMString, V2SearchIndex> = new_cache();

    /**************************** Helpers ***************************************/
    fn compile_search_index(search_pattern: &str) -> RUMResult<V2SearchIndex> {
        Ok(V2SearchIndex::from(search_pattern))
    }

    /**************************** Types *****************************************/
    ///
    /// V2Component.
    /// All V2Components contain the field's component data as a UTF-8 string.
    /// You can request a conversion to an atomic type via the as_* family of methods.
    ///
    /// ## Per Section 2.5.3.1
    ///
    /// ```text
    /// A field SHALL exist in one of three population states in an HL7 message:
    ///
    /// **Populated.** (Synonyms: valued, non-blank, not blank, not empty.) The sending system sends a value
    /// in the field. For example, if a sending system includes medical record number, that would be
    /// communicated as |1234567^^^MR^KP-CA|.
    ///
    /// **Not populated.** (Synonyms: unpopulated, not valued, unvalued, blank, empty, not present, missing.)
    /// The sending system does not supply a value for the field. The Sender might or might not have a value
    /// for the field. The receiving system can make no inference regarding the absence of an element value if
    /// there is not a conformance profile governing the implementation. However, if there is a Conformance
    /// Message Profile in effect, then special rules apply; see section 2.B, "Conformance Using Message
    /// Profiles".
    ///
    /// **Null. HL7 v2.x does not have an explicit concept for null values.**
    ///
    /// **Populated with Delete Indicator:** Any existing value for the corresponding data base element in the
    /// receiving application SHOULD be deleted. This is symbolically communicated as two double-quotes
    /// between the delimiters (i.e., |""|).Employing consecutive double quote characters as the only content of
    /// a field for other purposes is prohibited.
    /// ```
    ///
    #[derive(Default, Debug, RUMSerJson, RUMDeJson, PartialEq, Clone)]
    pub struct V2Component {
        component: RUMSerializableBuffer,
    }

    impl V2Component {
        pub fn new() -> Self {
            Self {
                component: RUMSerializableBuffer(RUMBuffer::new()),
            }
        }

        ///
        /// Constructs HL7 V2 Component.
        /// ### Per Section 2.7
        /// Added support for unescaping escaped strings.
        /// Support is limited to control sequences and hex/unicode character sequences.
        /// Advanced ANSI Escape sequences are not supported at this layer.
        /// We let the receiving application further handle the advanced ANSI escape sequences as
        /// it best sees fit.
        ///
        /// ## Section 2.7.3
        ///
        /// Note => People have already created the conversion tables for the different encodings
        /// but auto detection of encoding is not 100% reliable. Care should be taken when using
        /// the resulting string.
        ///
        /// ## Single-byte character sets:
        /// ```text
        ///-      \C2842\ISO-IR6 G0 (ISO 646 : ASCII)
        ///-      \C2D41\ISO-IR100 (ISO 8859 : Latin Alphabet 1)
        ///-      \C2D42\ISO-IR101 (ISO 8859 : Latin Alphabet 2)
        ///-      \C2D43\ISO-IR109 (ISO 8859 : Latin Alphabet 3)
        ///-      \C2D44\ISO-IR110 (ISO 8859 : Latin Alphabet 4)
        ///-      \C2D4C\ISO-IR144 (ISO 8859 : Cyrillic)
        ///-      \C2D47\ISO-IR127 (ISO 8859 : Arabic)
        ///-      \C2D46\ISO-IR126 (ISO 8859 : Greek)
        ///-      \C2D48\ISO-IR138 (ISO 8859 : Hebrew)
        ///-      \C2D4D\ISO-IR148 (ISO 8859 : Latin Alphabet 5)
        ///-      \C284A\ISO-IR14 (JIS X 0201 -1976: Romaji)
        ///-      \C2949\ISO-IR13 (JIS X 0201 : Katakana)
        /// ```
        /// ## Multi-byte codes:
        /// ```text
        ///-      \M2442\ISO-IR87 (JIS X 0208 : Kanji, hiragana and katakana)
        ///-      \M242844\ISO-IR159 (JIS X 0212 : Supplementary Kanji)
        /// ```
        /// We grab the ASCII string.
        /// Cast it to bytes while unescaping any escape sequences.
        /// Guess the encoding of the bytes.
        /// Decode back to UTF-8.
        /// If all things go right, the UTF-8 string should be a faithful representative of the
        /// intended string per section 2.7 of the standard.
        ///
        /// Will not support 2.7.8 Local encodings (\Zxxyy) until needed in the wild.
        ///
        fn from(component: RUMBuffer) -> Self {
            Self {
                component: RUMSerializableBuffer(component)
            }
        }

        pub fn to_string(&self) -> V2String {
            buffer_to_string(&self.component.0).unwrap_or_default()
        }

        pub fn is_empty(&self) -> bool {
            self.component.0 == ""
        }

        pub fn is_delete(&self) -> bool {
            self.component.0 == V2_DELETE_FIELD
        }

        pub fn as_datetime(&self) -> RUMResult<V2DateTime> {
            Ok(V2DateTime::from_str(buffer_to_str(&self.component.0)?)?)
        }

        pub fn as_bool(&self) -> bool {
            self.as_str().parse::<bool>().unwrap()
        }

        pub fn as_integer(&self) -> i64 {
            self.as_str().parse::<i64>().unwrap()
        }

        pub fn as_float(&self) -> f64 {
            self.as_str().parse::<f64>().unwrap()
        }
    }

    impl AsStr for V2Component {
        fn as_str(&self) -> &str {
            buffer_to_str(&self.component.0).unwrap_or_default()
        }
    }

    impl V2PrimitiveCasting for V2Component {}

    pub type ComponentList = Vec<V2Component>;

    ///
    /// A field is a collection of items separated by the field separation character.
    ///
    /// ## Example
    ///
    /// PID5 in
    /// `PID|||3064985^^^^SR^~ML288^^^^PI^||CHILD^BABEE^^^^^^||20180911|F||2106-3^^^^^|22 YOUNGER LAND^^JUNEAU^WI^53039^^^^WI027^^||(920)386-5555^PRN^PH^^^920^3865555^^|||||||||2186-5^^^^^|||||||`
    /// is `CHILD^BABEE^^^^^^`
    ///
    /// ## Per Section 2.5.3
    ///
    /// ```text
    /// A field is a string of characters. Fields for use within HL7 segments are defined by HL7. A
    /// comprehensive data dictionary of all HL7 fields is provided in Appendix A.
    ///```
    ///
    #[derive(Default, Debug, RUMSerJson, RUMDeJson, PartialEq, Clone)]
    pub struct V2Field {
        components: ComponentList,
    }

    impl V2Field {
        pub fn new() -> Self {
            Self {
                components: vec![V2Component::new()]
            }
        }

        pub fn from(field: RUMBuffer, parser_chars: &V2ParserCharacters) -> Self {
            let mut component_list: ComponentList = ComponentList::with_capacity(5);

            for c in field.split_fast(&[parser_chars.component_separator]) {
                component_list.push(V2Component::from(c))
            }

            Self {
                components: component_list,
            }
        }

        pub fn to_string(&self, parser_chars: &V2ParserCharacters) -> V2String {
            let mut components: Vec<&str> = Vec::with_capacity(self.components.len());
            for component in self.components.iter() {
                components.push(component.as_str())
            }
            components.join(&parser_chars.component_separator.as_string())
        }

        pub fn len(&self) -> usize {
            self.components.len()
        }

        pub fn get(&self, indx: isize) -> V2Result<&V2Component> {
            let component_indx = clamp_index(&indx, &(self.components.len() as isize))? - 1;
            match self.components.get(component_indx) {
                Some(component) => Ok(component),
                None => Err(rumtk_format!("Component at index {} not found!", indx)),
            }
        }

        pub fn get_mut(&mut self, indx: isize) -> V2Result<&mut V2Component> {
            let component_indx = clamp_index(&indx, &(self.components.len() as isize))? - 1;
            match self.components.get_mut(component_indx) {
                Some(component) => Ok(component),
                None => Err(rumtk_format!("Component at index {} not found!", indx)),
            }
        }
    }

    impl Index<isize> for V2Field {
        type Output = V2Component;
        fn index(&self, indx: isize) -> &V2Component {
            self.get(indx).unwrap()
        }
    }

    impl<'a> IndexMut<isize> for V2Field {
        fn index_mut(&mut self, indx: isize) -> &mut V2Component {
            self.get_mut(indx).unwrap()
        }
    }

    pub type V2FieldGroup = Vec<V2Field>;
    pub type V2FieldList = Vec<V2FieldGroup>;

    ///
    /// A segment comprises of a collection of items separated by the segment separator character.
    /// A segment is one line.
    ///
    /// ## Example
    ///
    /// - MSH|^~\\&|WIR11.3.2^^|WIR^^||WIRPH^^|20200514||VXU^V04^VXU_V04|2020051412382900|P^|2.5.1^^|||ER||||||^CDCPHINVS
    ///
    /// ## Per Section 2.5.2
    /// ```text
    /// A segment is a logical grouping of data fields. Segments of a message MAY be required or optional.
    /// They MAY occur only once in a message or they MAY be allowed to repeat. Each segment is given a
    /// name. For example, the ADT message MAY contain the following segments: Message Header (MSH),
    /// Event Type (EVN), Patient ID (PID), and Patient Visit (PV1).
    /// ```
    ///
    #[derive(Default, Debug, RUMSerJson, RUMDeJson, PartialEq, Clone)]
    pub struct V2Segment {
        name: V2String,
        description: V2String,
        fields: V2FieldList,
    }

    impl V2Segment {
        pub fn from(raw_segment: RUMBuffer, parser_chars: &V2ParserCharacters) -> V2Result<Self> {
            let segment = buffer_trim(&raw_segment);
            let pattern = &[parser_chars.field_separator];
            let mut raw_fields = segment.split_fast(pattern);
            let mut field_list = V2FieldList::new();

            let raw_field = match raw_fields.next() {
                Some(raw_field) => raw_field,
                None => return Err(rumtk_format!("Failed to get first field in segment! The segment is empty?")),
            };

            let segment_name = buffer_to_string(&raw_field[0..3])?;

            for raw_field in raw_fields {
                field_list.push(Self::generate_subfields(raw_field, parser_chars));
            }

            let field_description = V2_SEGMENT_DESC(&segment_name).to_string();

            Ok(V2Segment {
                name: segment_name,
                description: field_description,
                fields: field_list,
            })
        }

        pub fn to_string(&self, parser_chars: &V2ParserCharacters) -> V2String {
            let mut segment: Vec<V2String> = Vec::with_capacity(self.fields.len());
            for field_group in self.fields.iter() {
                let mut fields: Vec<V2String> = Vec::with_capacity(field_group.len());
                for field in field_group {
                    fields.push(field.to_string(parser_chars));
                }
                segment.push(fields.join(&parser_chars.repetition_separator.as_string()));
            }
            rumtk_format!(
                "{}{}{}",
                self.name,
                parser_chars.field_separator.as_string(),
                segment.join(&parser_chars.field_separator.as_string())
            )
        }

        pub fn get(&self, indx: isize) -> V2Result<&V2FieldGroup> {
            let field_indx = clamp_index(&indx, &(self.fields.len() as isize))? - 1;
            match self.fields.get(field_indx) {
                Some(field) => Ok(field),
                None => Err(rumtk_format!("Field number {} not found!", indx)),
            }
        }

        pub fn get_mut(&mut self, indx: isize) -> V2Result<&mut V2FieldGroup> {
            let field_indx = clamp_index(&indx, &(self.fields.len() as isize))? - 1;
            match self.fields.get_mut(field_indx) {
                Some(field) => Ok(field),
                None => Err(rumtk_format!("Field number {} not found!", indx)),
            }
        }

        pub fn len(&self) -> usize {
            self.fields.len()
        }

        fn generate_subfields(field: RUMBuffer, parser_chars: &V2ParserCharacters) -> Vec<V2Field> {
            if field.is_empty() {
                return vec![V2Field::new()];
            }

            let mut field_group = V2FieldGroup::new();
            for subfield in field.split_fast(&[parser_chars.repetition_separator]) {
                field_group.push(V2Field::from(subfield, parser_chars))
            }

            field_group
        }
    }

    impl<'a> Index<isize> for V2Segment {
        type Output = V2FieldGroup;
        fn index(&self, indx: isize) -> &V2FieldGroup {
            self.get(indx).unwrap()
        }
    }

    impl<'a> IndexMut<isize> for V2Segment {
        fn index_mut(&mut self, indx: isize) -> &mut V2FieldGroup {
            self.get_mut(indx).unwrap()
        }
    }

    ///
    /// Segments can be repeating. As such we contain them in groups.
    ///
    /// ## Per Section 2.5.2
    /// ```text
    /// Two or more segments MAY be organized as a logical unit called a segment group. A segment group
    /// MAY be required or optional and might or might not repeat. As of v 2.5, the first segment in a newly
    /// defined segment group will be required to help ensure that unparsable messages will not be
    /// inadvertently defined. This required first segment is known as the anchor segment.
    /// ```
    ///
    pub type V2SegmentGroup = Vec<V2Segment>;

    ///
    /// We collect segment groups in a map thus yielding the core of a message.
    ///
    pub type SegmentMap = RUMOrderedMap<u8, V2SegmentGroup>;
    pub type SerializableSegmentMap = RUMSerializableOrderedMap<u8, V2SegmentGroup>;

    #[derive(Default, Debug, RUMSerJson, RUMDeJson, PartialEq, Clone)]
    pub struct V2Message {
        separators: V2ParserCharacters,
        segment_groups: SerializableSegmentMap,
    }

    impl V2Message {
        ///
        /// Attempts to parse incoming raw HL7 v2 message into an instance of [V2Message](V2Message).
        ///
        /// ## Example
        ///
        /// ```
        /// ```
        ///
        pub fn try_from_buffer(raw_msg: RUMBuffer) -> V2Result<Self> {
            let sanitized = V2Message::sanitize(raw_msg);
            let parse_characters = V2ParserCharacters::from(&sanitized)?;
            let segments = V2Message::extract_segments(sanitized, &parse_characters)?;

            Ok(V2Message {
                separators: parse_characters,
                segment_groups: RUMSerializableOrderedMap(segments),
            })
        }

        ///
        /// Generates a raw V2 message from a [V2Message](V2Message) instance. Do keep in mind that
        /// printing this generated message in Unix will look as if the message was not parsed correctly
        /// but this is an artifact of following the standard and forcing all linefeed characters into
        /// carriage return characters as terminator.
        ///
        pub fn to_string(&self) -> V2String {
            let mut msg: Vec<V2String> = Vec::with_capacity(self.segment_groups.0.len());
            for segment_key in self.segment_groups.0.keys() {
                let segment_group = &self.segment_groups.0[segment_key];
                for segment in segment_group {
                    msg.push(segment.to_string(&self.separators));
                }
            }

            msg.join(&self.separators.segment_terminator.as_string())
        }

        pub fn len(&self) -> usize {
            self.segment_groups.0.len()
        }

        pub fn is_empty(&self) -> bool {
            self.segment_groups.0.is_empty()
        }

        pub fn get(&self, segment_index: &u8, sub_segment: usize) -> V2Result<&V2Segment> {
            let segment_group = self.get_group(segment_index)?;
            let subsegment_indx = sub_segment - 1;
            match segment_group.get(subsegment_indx) {
                Some(segment) => Ok(segment),
                None => Err(rumtk_format!(
                    "Subsegment {} was not found in segment group {}!",
                    subsegment_indx,
                    segment_index
                )),
            }
        }

        pub fn get_mut(
            &mut self,
            segment_index: &u8,
            sub_segment: usize,
        ) -> V2Result<&mut V2Segment> {
            let segment_group = self.get_mut_group(segment_index)?;
            let subsegment_indx = sub_segment - 1;
            match segment_group.get_mut(subsegment_indx) {
                Some(segment) => Ok(segment),
                None => Err(rumtk_format!(
                    "Subsegment {} was not found in segment group {}!",
                    subsegment_indx,
                    segment_index
                )),
            }
        }

        pub fn get_group(&self, segment_index: &u8) -> V2Result<&V2SegmentGroup> {
            match self.segment_groups.0.get(segment_index) {
                Some(segment_group) => Ok(segment_group),
                None => Err(rumtk_format!(
                    "Segment id {} not found in message!",
                    segment_index
                )),
            }
        }

        pub fn get_mut_group(&mut self, segment_index: &u8) -> V2Result<&mut V2SegmentGroup> {
            match self.segment_groups.0.get_mut(segment_index) {
                Some(segment_group) => Ok(segment_group),
                None => Err(rumtk_format!(
                    "Segment id {} not found in message!",
                    segment_index
                )),
            }
        }

        pub fn find_component(&self, search_pattern: &str) -> V2Result<&V2Component> {
            let index = rumtk_cache_fetch!(&mut search_cache, &search_pattern.to_string(), || {compile_search_index(search_pattern)})?;
            let segment = self.get(&index.segment, index.segment_group as usize)?;
            let field = match segment.get(index.field as isize)?.get((index.field_group - 1) as usize) {
                Some(field) => field,
                None => return Err(rumtk_format!("Subfield provided is not 1 indexed or out of bounds. Did you give us a 0 when you meant 1? Got {}!", index.field_group))
            };
            field.get(index.component as isize)
        }

        pub fn find_component_mut(
            &mut self,
            search_pattern: &str,
        ) -> V2Result<&mut V2Component> {
            let index = rumtk_cache_fetch!(&mut search_cache, &search_pattern.to_string(), || {compile_search_index(search_pattern)})?;
            let segment = self.get_mut(&index.segment, index.segment_group as usize)?;
            let mut field = match segment.get_mut(index.field as isize)?.get_mut((index.field_group - 1) as usize) {
                Some(field) => field,
                None => return Err(rumtk_format!("Subfield provided is not 1 indexed or out of bounds. Did you give us a 0 when you meant 1? Got {}!", index.field_group))
            };
            field.get_mut(index.component as isize)
        }

        pub fn is_repeat_segment(&self, segment_index: &u8) -> bool {
            let _segment_group: &V2SegmentGroup = self.get_group(segment_index).unwrap();
            _segment_group.len() > 1
        }

        pub fn segment_exists(&self, segment_index: &u8) -> bool {
            self.segment_groups.0.contains_key(segment_index)
        }

        ///
        /// Sanitizes incoming raw HL7 V2 message. In particular, this method ensures that the message
        /// only contains [V2_SEGMENT_TERMINATOR](V2_SEGMENT_TERMINATOR) as the newline terminator
        /// instead of a mixture of `\r\r`, `\r\n`, and `\n`.
        ///
        /// ## Example
        ///
        /// ```
        /// use rumtk_core::buffers::buffer_to_string;
        /// use rumtk_core::types::RUMBuffer;
        /// use rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Message;
        ///
        /// const RAW_MSG: &str = r"MSH|^~\\&#|NIST EHR^2.16.840.1.113883.3.72.5.22^ISO|NIST EHR Facility^2.16.840.1.113883.3.72.5.23^ISO|NIST Test Lab APP^2.16.840.1.113883.3.72.5.20^ISO|NIST Lab Facility^2.16.840.1.113883.3.72.5.21^ISO|20130211184101-0500||OML^O21^OML_O21|NIST-LOI_9.0_1.1-GU_PRU|T|2.5.1|||AL|AL|||||LOI_Common_Component^LOI BaseProfile^2.16.840.1.113883.9.66^ISO~LOI_GU_Component^LOI GU Profile^2.16.840.1.113883.9.78^ISO~LAB_PRU_Component^LOI PRU Profile^2.16.840.1.113883.9.82^ISO
        /// PID|1||PATID14567^^^NIST MPI&2.16.840.1.113883.3.72.5.30.2&ISO^MR||Hernandez^Maria^^^^^L||19880906|F||2054-5^Black or   African American^HL70005|3248 E  FlorenceAve^^Huntington Park^CA^90255^^H||^^PH^^^323^5825421|||||||||H^Hispanic or Latino^HL70189
        /// ORC|NW|ORD231-1^NIST EHR^2.16.840.1.113883.3.72.5.24^ISO|||||||20130116090021-0800|||134569827^Feller^Hans^^^^^^NPI&2.16.840.1.113883.4.6&ISO^L^^^NPI
        /// OBR|1|ORD231-1^NIST EHR^2.16.840.1.113883.3.72.5.24^ISO||34555-3^Creatinine 24H renal clearance panel^LN^^^^^^CreatinineClearance|||201301151130-0800|201301160912-0800||||||||134569827^Feller^Hans^^^^^^NPI&2.16.840.1.113883.4.6&ISO^L^^^NPI
        /// DG1|1||I10^Essential (primary) hypertension^I10C^^^^^^Hypertension, NOS|||F|||||||||2
        /// DG1|2||O10.93^Unspecified pre-existing hypertension complicating the puerperium^I10C^^^^^^Pregnancy with chronic hypertension|||W|||||||||1
        /// OBX|1|CWE|67471-3^Pregnancy status^LN^1903^Pregnancy status^99USL^2.44^^Isthe patient pregnant?||Y^Yes^HL70136^1^Yes, confirmed less than 12 weeks^99USL^2.5.1^^early pregnancy (pre 12 weeks)||||||O|||20130115|||||||||||||||SCI
        /// OBX|2|NM|3167-4^Volume of   24   hour Urine^LN^1904^Urine Volume of 24 hour collection^99USL^2.44^^Urine Volume 24hour collection||1250|mL^milliliter^UCUM^ml^mililiter^L^1.7^^ml|||||O|||20130116|||||||||||||||SCI
        /// OBX|3|NM|3141-9^Body weight Measured^LN^BWm^Body weight Measured^99USL^2.44^^patient weight measured in kg||59.5|kg^kilogram^UCUM|||||O|||20130116|||||||||||||||SCI
        /// SPM|1|S-2312987-1&NIST EHR&2.16.840.1.113883.3.72.5.24&ISO||276833005^24 hour urine sample (specimen)^SCT^UR24H^24hr Urine^99USL^^^24 hour urine|||||||||||||201301151130-0800^201301160912-0800
        /// SPM|2|S-2312987-2&NIST EHR&2.16.840.1.113883.3.72.5.24&ISO||119297000^Blood Specimen^SCT|||||||||||||201301160912-0800ORC|NW|ORD231-2^NIST EHR^2.16.840.1.113883.3.72.5.24^ISO|||||||20130115102146-0800|||134569827^Feller^Hans^^^^^^NPI&2.16.840.1.113883.4.6&ISO^L^^^NPI
        /// OBR|2|ORD231-2^NIST EHR^2.16.840.1.113883.3.72.5.24^ISO||21482-5^Protein [Mass/volume] in 24 hour Urine^LN^^^^^^24 hour Urine Protein|||201301151130-0800|201301160912-0800||||||||134569827^Feller^Hans^^^^^^NPI&2.16.840.1.113883.4.6&ISO^L^^^NPI
        /// DG1|1||I10^Essential (primary) hypertension^I10C^^^^^^Hypertension, NOS|||F|||||||||2";
        ///
        /// let data = RUMBuffer::from_static(RAW_MSG.as_bytes());
        /// let sanitized = V2Message::sanitize(data);
        ///
        /// assert_eq!(buffer_to_string(&sanitized).unwrap(), RAW_MSG.replace("\n", "\r"), "V2Message's sanitize method removed unintended contents instead of duplicated newlines. Size {} vs. {}", RAW_MSG.len(), sanitized.len());
        /// ```
        ///
        pub fn sanitize(raw_message: RUMBuffer) -> RUMBuffer {
            let mut raw_data = match raw_message.try_into_mut() {
                Ok(mut raw_data) => raw_data,
                Err(mut raw_data) => RUMBufferMut::from_iter(raw_data),
            };
            buffer_replace_in_place(&mut raw_data, &['\n'  as u8], &['\r' as u8]);
            buffer_replace_in_place(&mut raw_data, &['\r' as u8, '\r' as u8], &['\r' as u8, ' ' as u8]);
            buffer_trim(&raw_data.freeze())
        }

        ///
        ///
        pub fn extract_segments(
            msg: RUMBuffer,
            parser_chars: &V2ParserCharacters,
        ) -> V2Result<SegmentMap> {
            let mut segments: SegmentMap = SegmentMap::with_capacity(DEFAULT_CPU_CACHE_LINE_SIZE);

            for segment in msg.split_fast(&[parser_chars.segment_terminator]) {
                if segment.is_empty() {
                    continue;
                }

                let mut segment: V2Segment = V2Segment::from(segment, parser_chars)?;

                if segment.name == V2_MSHEADER_PATTERN_STR {
                    segment.fields[0] = vec![
                        V2Field {
                            components: vec![
                                V2Component::from(parser_chars.to_buffer())
                            ]
                        }
                    ]
                }

                let key = V2_SEGMENT_IDS(&segment.name);

                if !segments.contains_key(&key) {
                    segments.insert(key, V2SegmentGroup::new());
                }

                segments.get_mut(&key).unwrap().push(segment);
            }

            Ok(segments)
        }
    }

    impl<'a> Index<&'_ u8> for V2Message {
        type Output = V2SegmentGroup;
        fn index(&self, segment_index: &u8) -> &V2SegmentGroup {
            self.get_group(segment_index).unwrap()
        }
    }

    impl<'a> IndexMut<&'_ u8> for V2Message {
        fn index_mut(&mut self, segment_index: &u8) -> &mut V2SegmentGroup {
            self.get_mut_group(segment_index).unwrap()
        }
    }

    impl<'a> TryFrom<RUMBuffer> for V2Message {
        type Error = RUMString;
        fn try_from(input: RUMBuffer) -> V2Result<V2Message> {
            V2Message::try_from_buffer(input)
        }
    }

    impl<'a> TryFrom<&RUMBuffer> for V2Message {
        type Error = RUMString;
        fn try_from(input: &RUMBuffer) -> V2Result<V2Message> {
            V2Message::try_from_buffer(input.clone())
        }
    }

    impl<'a> TryFrom<&[u8]> for V2Message {
        type Error = RUMString;
        fn try_from(input: &[u8]) -> V2Result<V2Message> {
            V2Message::try_from_buffer(RUMBuffer::copy_from_slice(input))
        }
    }

    impl<'a> TryFrom<V2String> for V2Message {
        type Error = RUMString;
        fn try_from(input: V2String) -> V2Result<V2Message> {
            V2Message::try_from(string_to_buffer(input.as_str()))
        }
    }

    impl<'a> TryFrom<&V2String> for V2Message {
        type Error = RUMString;
        fn try_from(input: &V2String) -> V2Result<V2Message> {
            V2Message::try_from(input.as_str())
        }
    }

    impl<'a> TryFrom<&str> for V2Message {
        type Error = RUMString;
        fn try_from(input: &str) -> V2Result<V2Message> {
            V2Message::try_from(string_to_buffer(input))
        }
    }

    impl<'a> TryFrom<&&str> for V2Message {
        type Error = RUMString;
        fn try_from(input: &&str) -> V2Result<V2Message> {
            V2Message::try_from(input.as_bytes())
        }
    }
}

pub mod v2_parser_interface {
    /**************************** Macros ***************************************/
    ///
    /// Simple interface for creating an instance of V2Message!
    /// You can pass a string view, a String, a RUMString, or a byte slice as input.
    ///
    /// ## Example
    ///
    /// ```
    ///     use rumtk_hl7_v2::{rumtk_v2_parse_message};
    ///     let pattern = "MSH1.1";
    ///     let hl7_v2_message = "MSH|^~\\&|NISTEHRAPP|NISTEHRFAC|NISTIISAPP|NISTIISFAC|20150625072816.601-0500||VXU^V04^VXU_V04|NIST-IZ-AD-10.1_Send_V04_Z22|P|2.5.1|||ER|AL|||||Z22^CDCPHINVS|NISTEHRFAC|NISTIISFAC\n";
    ///     let message = rumtk_v2_parse_message!(&hl7_v2_message).unwrap();
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_v2_parse_message {
        ( $msg:expr ) => {{
            use $crate::hl7_v2_parser::v2_parser::{V2Message, V2Result};
            V2Message::try_from($msg)
        }};
    }

    ///
    /// Simple interface for searching for a component inside a V2Message.
    /// This macro takes a borrow of a V2Message instance and a string search pattern.
    /// The only search pattern supported at the moment takes the form
    /// **<3-letter segment>(optional, segment_group)<field>\[optional, field_group\].<component>**.
    /// For example, you can search with **PID5.1** or **PID(1)5.1** or **PID(1)5\[1\].1**.
    ///
    /// The optional portions are for when you need to select a specific repeated segment or field.
    ///
    /// All of these indices must be 1-indexed.
    ///
    /// For the main indices, you can use negative values. For example, a -1 means you want to select
    /// the last item. This is applicable for the field and component indices.
    ///
    /// ## Example
    ///
    /// ```
    ///     use rumtk_hl7_v2::{rumtk_v2_parse_message, rumtk_v2_find_component};
    ///     let pattern = "MSH1.1";
    ///     let hl7_v2_message = "MSH|^~\\&|NISTEHRAPP|NISTEHRFAC|NISTIISAPP|NISTIISFAC|20150625072816.601-0500||VXU^V04^VXU_V04|NIST-IZ-AD-10.1_Send_V04_Z22|P|2.5.1|||ER|AL|||||Z22^CDCPHINVS|NISTEHRFAC|NISTIISFAC\n";
    ///     let message = rumtk_v2_parse_message!(&hl7_v2_message).unwrap();
    ///     let component = rumtk_v2_find_component!(message, pattern).unwrap();
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_v2_find_component {
        ( $v2_msg:expr, $v2_search_pattern:expr ) => {{
            use rumtk_core::strings::RUMString;
            use $crate::hl7_v2_parser::v2_parser::{V2Component, V2Result};
            $v2_msg.find_component(&RUMString::from($v2_search_pattern))
        }};
    }

    ///
    /// Macro for generating V2 string message out of an instance of [hl7_v2_parser::v2_parser::V2Message].
    /// Basically, this is the opposite operation to [crate::rumtk_v2_parse_message].
    ///
    /// # Example
    /// ```
    ///     use rumtk_hl7_v2::{rumtk_v2_generate_message, rumtk_v2_parse_message};
    ///     let pattern = "MSH1.1";
    ///     let hl7_v2_message = "MSH|^~\\&|NISTEHRAPP|NISTEHRFAC|NISTIISAPP|NISTIISFAC|20150625072816.601-0500||VXU^V04^VXU_V04|NIST-IZ-AD-10.1_Send_V04_Z22|P|2.5.1|||ER|AL|||||Z22^CDCPHINVS|NISTEHRFAC|NISTIISFAC\n";
    ///     let message = rumtk_v2_parse_message!(&hl7_v2_message).unwrap();
    ///     let generated_message_str = rumtk_v2_generate_message!(&message);
    ///     let generated_message = rumtk_v2_parse_message!(&generated_message_str).unwrap();
    ///     assert_eq!(
    ///             &message, &generated_message,
    ///             "Messages are not equal! Expected: {:?} Got: {:?}",
    ///             &message, &generated_message
    ///         );
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_v2_generate_message {
        ( $v2_msg:expr ) => {{
            $v2_msg.to_string()
        }};
    }
}
