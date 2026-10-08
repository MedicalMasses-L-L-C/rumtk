#!/usr/bin/env python3
"""Extract the Table Metadata sections from the HL7 v2.9 vocabulary text export."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


OUTPUT_PREFIX = "v29_table_"
OUTPUT_SUFFIX = "_metadata.txt"


def spaced_word(value: str) -> str:
    """Return a regex fragment matching a word with PDF-added whitespace."""
    return r"\s*".join(map(re.escape, value))


METADATA_HEADING = re.compile(r"^\s*Table\s+Metadata\s*$", re.IGNORECASE)
TABLE_TITLE = re.compile(r"^\s*\d{4}\s+-\s+\S")
CODED_CONTENT = re.compile(
    r"Table\s*\d{4}\s*Coded\s+Content", re.IGNORECASE
)
TABLE_FIELD = re.compile(
    rf"^\s*{spaced_word('Table')}\s+((?:\d\s*){{4}})(?=\s|$)",
    re.IGNORECASE,
)
FIELD_PATTERNS = {
    "Table": TABLE_FIELD,
    "Description": re.compile(
        rf"^\s*{spaced_word('Description')}\b", re.IGNORECASE
    ),
    "Type": re.compile(rf"^\s*{spaced_word('Type')}\b", re.IGNORECASE),
    "Steward": re.compile(rf"^\s*{spaced_word('Steward')}\b", re.IGNORECASE),
    "where used": re.compile(
        rf"^\s*{spaced_word('where used')}\b", re.IGNORECASE
    ),
    "HL7 Version Introduced": re.compile(
        rf"^\s*{spaced_word('HL7 Version Introduced')}\b", re.IGNORECASE
    ),
}
FOOTER_PATTERN = re.compile(
    r"^[^\n]*Pag\s+e\b.*?December\s+2019\.?[^\n]*(?:\n|$)",
    re.IGNORECASE | re.MULTILINE | re.DOTALL,
)


def remove_footer(lines: list[str]) -> list[str]:
    """Remove the PDF page footer spanning the page and date lines."""
    text = "\n".join(lines)
    return FOOTER_PATTERN.sub("", text).splitlines()


def metadata_value_column(lines: list[str]) -> int:
    """Return the value column used by the other fields in a metadata section."""
    for line in lines:
        field_match = re.match(r"^\s*(?:Description|Steward|where\s+used)\s+", line, re.IGNORECASE)
        if field_match:
            value_match = re.search(r"\S", line[field_match.end() :])
            if value_match:
                return field_match.end() + value_match.start()
    for line in lines:
        field_match = FIELD_PATTERNS["HL7 Version Introduced"].match(line)
        if field_match:
            value_match = re.search(r"\S", line[field_match.end() :])
            if value_match:
                return field_match.end() + value_match.start()
    raise ValueError("metadata section has no field value column")


def normalize_field_labels(lines: list[str]) -> list[str]:
    """Replace PDF-added whitespace and align metadata field values."""
    value_column = metadata_value_column(lines)
    normalized: list[str] = []
    for line in lines:
        table_match = TABLE_FIELD.match(line)
        if table_match:
            indentation = line[: len(line) - len(line.lstrip())]
            table_number = re.sub(r"\s+", "", table_match.group(1))
            padding = " " * max(1, value_column - len(indentation) - len("Table"))
            line = f"{indentation}Table{padding}{table_number}"
        else:
            for field in ("Type", "Steward", "where used"):
                field_match = FIELD_PATTERNS[field].match(line)
                if field_match:
                    indentation = line[: len(line) - len(line.lstrip())]
                    value_match = re.search(r"\S", line[field_match.end() :])
                    if value_match is None:
                        line = f"{indentation}{field}"
                        break
                    value_start = field_match.end() + value_match.start()
                    padding = " " * max(
                        1, value_column - len(indentation) - len(field)
                    )
                    line = f"{indentation}{field}{padding}{line[value_start:]}"
                    break
        normalized.append(line)
    return normalized


def metadata_sections(lines: list[str]) -> tuple[list[tuple[str, list[str]]], int]:
    """Return metadata sections and the number missing a source HL7 version row."""
    sections: list[tuple[str, list[str]]] = []
    missing_hl7_version = 0
    index = 0
    while index < len(lines):
        if not METADATA_HEADING.match(lines[index]):
            index += 1
            continue

        field_lines: dict[str, int] = {}
        table_number: str | None = None
        boundary = len(lines)
        for candidate in range(index + 1, len(lines)):
            if candidate > index and (
                METADATA_HEADING.match(lines[candidate])
                or TABLE_TITLE.match(lines[candidate])
            ):
                boundary = candidate
                break
            if CODED_CONTENT.search(lines[candidate]):
                boundary = candidate
                break
            table_match = TABLE_FIELD.match(lines[candidate])
            if table_match is not None:
                table_number = re.sub(r"\s+", "", table_match.group(1))
                field_lines["Table"] = candidate
            for field, pattern in FIELD_PATTERNS.items():
                if field != "Table" and pattern.match(lines[candidate]):
                    field_lines[field] = candidate

        missing = [field for field in FIELD_PATTERNS if field not in field_lines]
        required_missing = [
            field
            for field in missing
            if field not in {"Steward", "where used", "HL7 Version Introduced"}
        ]
        if table_number is None or required_missing:
            location = index + 1
            raise ValueError(
                f"metadata section at source line {location} is incomplete; "
                f"missing {', '.join(required_missing) or 'table number'}"
            )
        if "HL7 Version Introduced" in missing:
            missing_hl7_version += 1
        end = field_lines.get("HL7 Version Introduced", boundary - 1) + 1
        section = remove_footer(lines[index:end])
        sections.append((table_number, normalize_field_labels(section)))
        index = boundary

    if not sections:
        raise ValueError("no Table Metadata sections were found")
    return sections, missing_hl7_version


def extract(source: Path, output_dir: Path) -> int:
    """Extract metadata files and return the number written."""
    lines = source.read_text(encoding="utf-8", errors="replace").splitlines()
    sections, missing_hl7_version = metadata_sections(lines)
    numbers = [number for number, _ in sections]
    if len(numbers) != len(set(numbers)):
        raise ValueError("duplicate table numbers found in metadata sections")

    output_dir.mkdir(parents=True, exist_ok=True)
    for old_output in output_dir.glob(f"{OUTPUT_PREFIX}*{OUTPUT_SUFFIX}"):
        old_output.unlink()

    for number, section in sections:
        output_dir.joinpath(
            f"{OUTPUT_PREFIX}{number}{OUTPUT_SUFFIX}"
        ).write_text("\n".join(section).rstrip() + "\n", encoding="utf-8")

    actual = {
        path.name.removeprefix(OUTPUT_PREFIX).removesuffix(OUTPUT_SUFFIX)
        for path in output_dir.glob(f"{OUTPUT_PREFIX}*{OUTPUT_SUFFIX}")
    }
    expected = set(numbers)
    if actual != expected:
        raise ValueError(
            f"output mismatch: expected {len(expected)}, found {len(actual)}"
        )
    return len(sections), missing_hl7_version


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "source", type=Path, nargs="?", default=Path(__file__).with_name("v29_C2_tables.txt")
    )
    parser.add_argument(
        "--output-dir", type=Path, default=Path(__file__).parent / "output"
    )
    args = parser.parse_args()
    count, missing_hl7_version = extract(args.source, args.output_dir)
    print(f"Extracted {count} metadata tables from {args.source}")
    if missing_hl7_version:
        print(
            f"Warning: {missing_hl7_version} source metadata tables have no "
            "HL7 Version Introduced row"
        )


if __name__ == "__main__":
    main()