#!/usr/bin/env python3
"""Generate Rust HL7 v2 metadata definitions from extracted metadata files."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
from pathlib import Path


METADATA_FILE = re.compile(r"v29_table_(\d{4})_metadata\.txt$")


def rust_string(value: str) -> str:
    """Return a Rust string literal containing value."""
    return json.dumps(value, ensure_ascii=False)


def field_pattern(label: str) -> re.Pattern[str]:
    """Match a field label even when PDF extraction inserted spaces in it."""
    return re.compile(
        r"^\s*" + r"\s*".join(map(re.escape, label)) + r"\b",
        re.IGNORECASE,
    )


FIELD_PATTERNS = {
    "description": field_pattern("Description"),
    "type": field_pattern("Type"),
    "Sseward": field_pattern("Steward"),
    "where_used": field_pattern("where used"),
    "hl7_version": field_pattern("HL7 Version Introduced"),
}


def parse_metadata(path: Path) -> tuple[int, dict[str, str]]:
    """Parse one extracted metadata file."""
    match = METADATA_FILE.fullmatch(path.name)
    if match is None:
        raise ValueError(f"unexpected metadata filename: {path.name}")

    table_match = re.search(r"^\s*Table\s+(\d{4})\s*$", path.read_text().split("\n", 1)[1], re.MULTILINE)
    if table_match is None:
        raise ValueError(f"{path}: missing table number")
    table = int(table_match.group(1))
    if table != int(match.group(1)):
        raise ValueError(f"{path}: filename and table number disagree")

    values = {field: "" for field in FIELD_PATTERNS}
    current: str | None = None
    for line in path.read_text(encoding="utf-8").splitlines()[1:]:
        matched_field = next(
            (field for field, pattern in FIELD_PATTERNS.items() if pattern.match(line)),
            None,
        )
        if matched_field is not None:
            match = FIELD_PATTERNS[matched_field].match(line)
            assert match is not None
            values[matched_field] = line[match.end() :].strip()
            current = matched_field
        elif current == "description" and line.strip():
            values[current] += (" " if values[current] else "") + line.strip()

    return table, values


def generate(input_dir: Path, output: Path) -> int:
    """Generate metadata.rs and return the number of definitions written."""
    entries = [parse_metadata(path) for path in sorted(input_dir.glob("*_metadata.txt"))]
    if not entries:
        raise ValueError(f"no metadata files found in {input_dir}")
    if len({table for table, _ in entries}) != len(entries):
        raise ValueError("duplicate table numbers found")

    lines = [
        "//! HL7 v2 coded-content table metadata.",
        "",
        "use super::v2_tables::V2MetadataTable;",
        "",
    ]
    for table, values in entries:
        name = f"TABLE_{table:04d}_METADATA"
        lines.extend(
            [
                f"pub static {name}: V2MetadataTable = V2MetadataTable {{",
                f"    table: {table}usize,",
                f"    description: {rust_string(values['description'])},",
                f"    r#type: {rust_string(values['type'])},",
                f"    Sseward: {rust_string(values['Sseward'])},",
                f"    where_used: {rust_string(values['where_used'])},",
                f"    hl7_version: {rust_string(values['hl7_version'])},",
                "};",
                "",
            ]
        )
    output.write_text("\n".join(lines), encoding="utf-8")
    subprocess.run(
        ["rustfmt", "--edition", "2021", str(output)],
        check=True,
    )
    return len(entries)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--input-dir", type=Path, default=Path(__file__).parent / "output"
    )
    parser.add_argument(
        "--output", type=Path, default=Path(__file__).parent / "rust" / "metadata.rs"
    )
    args = parser.parse_args()
    print(f"Generated {generate(args.input_dir, args.output)} metadata tables")


if __name__ == "__main__":
    main()