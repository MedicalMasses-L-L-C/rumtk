#!/usr/bin/env python3
"""Generate Rust V2 table definitions from the cleaned HL7 v2.9 tables."""

from __future__ import annotations

import argparse
import re
from dataclasses import dataclass
from pathlib import Path


TABLE_TITLE = re.compile(r"Table\s*(\d{4})\s*Coded\s+Content", re.IGNORECASE)
TABLE_FILE = re.compile(r"v29_table_(\d{4})\.txt$")


@dataclass(frozen=True)
class Row:
    value: str
    display_name: str
    definition: str
    comment_usage_note: str
    status: str


@dataclass(frozen=True)
class Table:
    number: str
    rows: tuple[Row, ...]


def spaced_word(label: str) -> re.Pattern[str]:
    """Match a label even when PDF text extraction spaces its letters apart."""
    return re.compile(r"\s*".join(map(re.escape, label)), re.IGNORECASE)


FIELD_LABELS = (
    spaced_word("Value"),
    spaced_word("Display Name"),
    spaced_word("Definition"),
    spaced_word("Comment/"),
    spaced_word("Status"),
)


def find_header(lines: list[str], path: Path) -> tuple[int, list[int]]:
    for index, line in enumerate(lines[:8]):
        matches = [pattern.search(line) for pattern in FIELD_LABELS]
        if all(match is not None for match in matches):
            starts = [match.start() for match in matches if match is not None]
            if starts == sorted(starts):
                return index, starts
    raise ValueError(f"{path}: could not find the five-column header")


def cell(line: str, start: int, end: int | None = None) -> str:
    return line[start:end].strip() if end is not None else line[start:].strip()


def finish_row(current: list[list[str]]) -> Row:
    values = [
        re.sub(r"[ \t]{2,}", " ", " ".join(part for part in value if part))
        for value in current
    ]
    return Row(*values)


def parse_table(path: Path) -> Table:
    match = TABLE_FILE.fullmatch(path.name)
    if match is None:
        raise ValueError(f"unexpected table filename: {path.name}")
    number = match.group(1)
    lines = path.read_text(encoding="utf-8").splitlines()
    title = next((TABLE_TITLE.search(line) for line in lines[:2]), None)
    if title is None or title.group(1) != number:
        raise ValueError(f"{path}: title does not contain table {number}")
    header_index, starts = find_header(lines, path)
    rows: list[Row] = []
    current: list[list[str]] | None = None
    row_starts = starts

    for line in lines[header_index + 1 :]:
        # The extracted PDF text occasionally repeats the column header in the
        # middle of a table, sometimes after a real row on the same line.
        header_match = re.search(r"\bValue\s+Display\s+Name\b", line)
        if header_match is not None:
            line = line[: header_match.start()]
        normalized = re.sub(r"\s+", "", line).lower()
        if not line.strip():
            continue
        if "valuedisplaynamedefinition" in normalized:
            continue
        if normalized == "usagenote":
            continue

        first = re.search(r"\S+", line)
        starts_row = first is not None and first.start() <= 8
        if starts_row:
            # Keep the value column anchored to the actual code.  The display
            # column may independently move when the PDF extractor compresses
            # or expands the spacing after a short value.
            value_shift = first.start() - starts[0]
            display_start = first.end()
            while display_start < len(line) and line[display_start].isspace():
                display_start += 1
            if display_start >= starts[2] - 10:
                display_start = starts[1] + value_shift
            row_starts = [
                first.start(),
                display_start,
                *[start + value_shift for start in starts[2:]],
            ]

        fields = [
            cell(
                line,
                start,
                row_starts[index + 1] if index + 1 < len(row_starts) else None,
            )
            for index, start in enumerate(row_starts)
        ]
        if starts_row:
            if current is not None:
                rows.append(finish_row(current))
            current = [[field] for field in fields]
        elif current is not None:
            for index, field in enumerate(fields):
                if field:
                    current[index].append(field)

    if current is not None:
        rows.append(finish_row(current))
    if not rows:
        raise ValueError(f"{path}: no data rows found")
    return Table(number, tuple(rows))


def rust_string(value: str) -> str:
    return '"' + value.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n") + '"'


def unique_rows(rows: tuple[Row, ...]) -> tuple[Row, ...]:
    seen: set[str] = set()
    result: list[Row] = []
    for row in rows:
        if row.value not in seen:
            seen.add(row.value)
            result.append(row)
    return tuple(result)


def generate(input_dir: Path, output_dir: Path) -> tuple[int, int]:
    paths = sorted(
        path
        for path in input_dir.glob("v29_table_*.txt")
        if re.fullmatch(r"v29_table_\d+\.txt", path.name)
    )
    if not paths:
        raise ValueError(f"no v29 table files found in {input_dir}")
    tables = [parse_table(path) for path in paths]
    numbers = [table.number for table in tables]
    if len(numbers) != len(set(numbers)):
        raise ValueError("duplicate table numbers")

    output_dir.mkdir(parents=True, exist_ok=True)
    (output_dir / "mod.rs").write_text(
        "pub mod tables;\npub mod helpers;\npub mod v2_tables;\n", encoding="utf-8"
    )
    (output_dir / "v2_tables.rs").write_text(
        """/// One coded value from an HL7 v2 coded-content table.\n#[derive(Debug, PartialEq, Eq)]\npub struct V2TableRow {\n    pub value: &'static str,\n    pub display_name: &'static str,\n    pub definition: &'static str,\n    pub comment_usage_note: &'static str,\n    pub status: &'static str,\n}\n\n/// An HL7 v2 coded-content table.\n#[derive(Debug, PartialEq, Eq)]\npub struct V2Table {\n    pub number: u16,\n    pub rows: phf::Map<&'static str, V2TableRow>,\n}\n\n""",
        encoding="utf-8",
    )
    lines = [
        "use phf::phf_map;",
        "",
        "use super::v2_tables::{V2Table, V2TableRow};",
        "",
    ]
    generated_rows = 0
    for table in tables:
        rows = unique_rows(table.rows)
        generated_rows += len(rows)
        lines.append(f"pub static TABLE_{table.number}: V2Table = V2Table {{")
        lines.append(f"    number: {int(table.number)},")
        lines.append("    rows: phf_map! {")
        for row in rows:
            lines.append(
                f"        {rust_string(row.value)} => V2TableRow {{ "
                f"value: {rust_string(row.value)}, "
                f"display_name: {rust_string(row.display_name)}, "
                f"definition: {rust_string(row.definition)}, "
                f"comment_usage_note: {rust_string(row.comment_usage_note)}, "
                f"status: {rust_string(row.status)} "
                "},"
            )
        lines.extend(["    },", "};", ""])
    (output_dir / "tables.rs").write_text("\n".join(lines), encoding="utf-8")
    helper_lines = [
        "use super::tables;",
        "use super::v2_tables::V2Table;",
        "",
        "/// Returns the HL7 v2 table identified by its table number.",
        "#[inline]",
        "pub fn get_table(table_number: usize) -> Option<&'static V2Table> {",
        "    match table_number {",
    ]
    helper_lines.extend(
        f"        {int(table.number)} => Some(&constants::TABLE_{table.number}),"
        for table in tables
    )
    helper_lines.extend(["        _ => None,", "    }", "}", ""])
    (output_dir / "helpers.rs").write_text("\n".join(helper_lines), encoding="utf-8")
    return len(tables), generated_rows


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input-dir", type=Path, default=Path(__file__).with_name("output"))
    parser.add_argument("--output-dir", type=Path, default=Path(__file__).with_name("rust"))
    args = parser.parse_args()
    tables, rows = generate(args.input_dir, args.output_dir)
    print(f"Generated {tables} tables and {rows} rows in {args.output_dir}")


if __name__ == "__main__":
    main()