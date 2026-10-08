#!/usr/bin/env python
# -*- coding: utf-8 -*-

# pypdf: Preserves horizontal positioning and layout
from pypdf import PdfReader
from sys import argv

reader = PdfReader(argv[1])

results = ""

for page in reader.pages:
   results += page.extract_text(extraction_mode="layout")

# Extract text preserving layout structure
print(results)   
