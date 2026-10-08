#!/usr/bin/env python3
"""Remove HL7 page headers and footers from extracted v2.9 table text files."""

from __future__ import annotations

import argparse
import re
from pathlib import Path


HEADER = re.compile(
    r"\n+Health Level Seven, Version 2.9.*?Usage Note.*?\n.*?",
    re.DOTALL,
)
TABLE_HEADER = re.compile(
    r"(?m)^[ \t]*Value[^\n]*Display Name[^\n]*Definition[^\n]*"
    r"Comment/[^\n]*Status[^\n]*(?:\n[ \t]*(?:Usage[ \t]*)?Note[ \t]*)?(?:\n|$)"
)
FOOTER = re.compile(r"\n+Health Level Seven, Version 2.9.*?Pag.*No rmative Pu blication.*|\n+Pag.*?\n.*?Value.*?\n.*?\n|\n+Pag.*?\n.*?\n+")
FOOTER_AT_END = re.compile(
    r"\n+Health Level Seven, Version 2.9[^\n]*Pag[^\n]*"
    r"\n[^\n]*No rmative Pu blication\.[^\n]*"
    r"(?:"
    r"(?:\n[ \t]*)*"
    r"(?=[ \t]*Value[^\n]*(?:Usage[^\n]*|\n[^\n]*Usage[^\n]*))"
    r"|(?:\n[^\n]*)*\Z"
    r")"
)


def clean_tables(input_dir: Path, output_dir: Path) -> tuple[int, int, int, int]:
    """Clean every table in input_dir and return file and match counts."""
    tables = sorted(input_dir.glob("*.txt"))
    output_dir.mkdir(parents=True, exist_ok=True)
    for old_output in output_dir.glob("*.txt"):
        old_output.unlink()

    matched = 0
    footers = 0
    repeated_headers = 0
    for table in tables:
        text = table.read_text(encoding="utf-8", errors="replace")
        cleaned, replacements = FOOTER_AT_END.subn("\n", text)
        footers += replacements
        cleaned, replacements = FOOTER.subn("", cleaned)
        footers += replacements
        cleaned, replacements = HEADER.subn("\n", cleaned)
        matched += replacements
        header_seen = False

        def keep_first_header(match: re.Match[str]) -> str:
            nonlocal header_seen, repeated_headers
            if not header_seen:
                header_seen = True
                return match.group(0)
            repeated_headers += 1
            return ""

        cleaned = TABLE_HEADER.sub(keep_first_header, cleaned)
        output_dir.joinpath(table.name).write_text(cleaned, encoding="utf-8")

    return len(tables), matched, footers, repeated_headers


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--input-dir",
        type=Path,
        default=Path(__file__).parent / "tables",
    )
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=Path(__file__).parent / "output",
    )
    args = parser.parse_args()
    count, matched, footers, repeated_headers = clean_tables(args.input_dir, args.output_dir)
    print(
        f"Processed {count} tables; removed {matched} headers, {repeated_headers} "
        f"repeated headers, and {footers} footers"
    )


if __name__ == "__main__":
    main()