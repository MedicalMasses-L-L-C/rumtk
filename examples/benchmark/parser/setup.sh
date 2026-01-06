#!/bin/bash
#
#     rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
#     This toolkit aims to be reliable, simple, performant, and standards compliant.
#     Copyright (C) 2026  Luis M. Santos, M.D.
#
#     This program is free software: you can redistribute it and/or modify
#     it under the terms of the GNU General Public License as published by
#     the Free Software Foundation, either version 3 of the License, or
#     (at your option) any later version.
#
#     This program is distributed in the hope that it will be useful,
#     but WITHOUT ANY WARRANTY; without even the implied warranty of
#     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#     GNU General Public License for more details.
#
#     You should have received a copy of the GNU General Public License
#     along with this program.  If not, see <https://www.gnu.org/licenses/>.
#

pushd "$1" || exit

echo "Generating HL7 sample files..."
for i in {0..99}; do
  # Generate a 2MB BASE 64 encoded string to embed in ORU to simulate embedded report
  random_string=$(head -c 2097152 /dev/urandom | base64 -w 0)
  fpath="$1/$i.hl7"
  cp "$3/path_report_enterprisehealth.hl7" "$fpath"
  pdf_data="\nOBX|51|ED|4050097^Surg Path Final Report^^4050097^Surg Path Final Report||^PDF^^base64^$random_string||||||F|||20120309132541"
  echo "$pdf_data" >> "$fpath"
  echo "Generated file $fpath..."
done

echo "Benchmark samples found in $1"

popd || exit
