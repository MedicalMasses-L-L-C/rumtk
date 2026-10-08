#!/usr/bin/env python3
"""Extract the coded-content tables from the HL7 v2.9 vocabulary text export."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


TABLE_TITLE = re.compile(r"(?:^|\s)(\d{4})\s+-\s+(.+?)\s*$")
CODED_CONTENT = re.compile(r"Table\s*(\d{4})\s*Coded\s+Content", re.IGNORECASE)
SECTION_TITLE = "2C.6                                                             HL7 Version 2 Code Tables"
OUTPUT_PREFIX = "v29_table_"


def table_titles(lines: list[str]) -> list[tuple[int, str, str]]:
    """Return numbered table titles, starting with the actual code-table section."""
    section_start = next(
        index for index, line in enumerate(lines) if SECTION_TITLE in line
    )
    titles = []
    for index in range(section_start + 1, len(lines)):
        match = TABLE_TITLE.search(lines[index])
        if match:
            titles.append((index, match.group(1), match.group(2).strip()))
    return titles


def extract(source: Path, output_dir: Path) -> int:
    lines = source.read_text(encoding="utf-8", errors="replace").splitlines()
    titles = table_titles(lines)
    extracted: list[tuple[str, int, int]] = []
    output_dir.mkdir(parents=True, exist_ok=True)
    for old_output in output_dir.glob(f"{OUTPUT_PREFIX}*.txt"):
        old_output.unlink()

    for title_position, (title_index, number, _description) in enumerate(titles):
        next_title = (
            titles[title_position + 1][0]
            if title_position + 1 < len(titles)
            else len(lines)
        )
        block = lines[title_index:next_title]
        coded_index = next(
            (index for index, line in enumerate(block) if CODED_CONTENT.search(line)),
            None,
        )
        if coded_index is None:
            continue

        coded_number = CODED_CONTENT.search(block[coded_index]).group(1)
        if coded_number != number:
            raise ValueError(
                f"table title {number} at source line {title_index + 1} "
                f"does not match coded-content heading {coded_number}"
            )
        table_text = "\n".join(block[coded_index:]).rstrip() + "\n"
        normalized = re.sub(r"\s+", "", table_text).lower()
        if not all(column in normalized for column in ("value", "displayname", "definition")):
            raise ValueError(f"table {number} has no Value/Display Name/Definition header")
        output_dir.joinpath(f"{OUTPUT_PREFIX}{number}.txt").write_text(
            table_text, encoding="utf-8"
        )
        extracted.append((number, title_index + 1, next_title))

    if not extracted:
        raise ValueError("no coded-content tables were found")

    expected = {number for _, number, _ in titles if number in {item[0] for item in extracted}}
    actual = {path.stem.removeprefix(OUTPUT_PREFIX) for path in output_dir.glob(f"{OUTPUT_PREFIX}*.txt")}
    if actual != expected:
        raise ValueError(f"output mismatch: expected {len(expected)}, found {len(actual)}")
    return len(extracted)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "source", type=Path, nargs="?", default=Path(__file__).with_name("v29_C2_tables.txt")
    )
    parser.add_argument(
        "--output-dir", type=Path, default=Path(__file__).parent
    )
    args = parser.parse_args()
    count = extract(args.source, args.output_dir)
    print(f"Extracted {count} tables from {args.source}")


if __name__ == "__main__":
    main()