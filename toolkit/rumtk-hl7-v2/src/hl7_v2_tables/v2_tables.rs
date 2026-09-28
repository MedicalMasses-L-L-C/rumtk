
/// One coded value from an HL7 v2 coded-content table.
#[derive(Debug, PartialEq, Eq)]
pub struct V2TableRow {
    pub value: &'static str,
    pub display_name: &'static str,
    pub definition: &'static str,
    pub comment_usage_note: &'static str,
    pub status: &'static str,
}

/// Metadata describing an HL7 v2 coded-content table.
#[derive(Debug, PartialEq, Eq)]
pub struct V2MetadataTable {
    pub table: usize,
    pub description: &'static str,
    pub ttype: &'static str,
    pub steward: &'static str,
    pub where_used: &'static str,
    pub hl7_version: &'static str,
}

/// An HL7 v2 coded-content table.
#[derive(Debug, PartialEq, Eq)]
pub struct V2Table {
    pub number: u16,
    pub metadata: &'static V2MetadataTable,
    pub rows: phf::Map<&'static str, V2TableRow>,
}
