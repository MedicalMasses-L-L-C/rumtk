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
FOOTER = re.compile(r"\n+Pag.*\n?")


def clean_tables(input_dir: Path, output_dir: Path) -> tuple[int, int, int]:
    """Clean every table in input_dir and return file and match counts."""
    tables = sorted(input_dir.glob("*.txt"))
    output_dir.mkdir(parents=True, exist_ok=True)
    for old_output in output_dir.glob("*.txt"):
        old_output.unlink()

    matched = 0
    footers = 0
    for table in tables:
        text = table.read_text(encoding="utf-8", errors="replace")
        cleaned, replacements = HEADER.subn("\n", text)
        matched += replacements
        cleaned, replacements = FOOTER.subn("", cleaned)
        footers += replacements
        output_dir.joinpath(table.name).write_text(cleaned, encoding="utf-8")

    return len(tables), matched, footers


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
    count, matched, footers = clean_tables(args.input_dir, args.output_dir)
    print(f"Processed {count} tables; removed {matched} headers and {footers} footers")


if __name__ == "__main__":
    main()