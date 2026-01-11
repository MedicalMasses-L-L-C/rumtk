#!/bin/bash
#
#     rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
#     This toolkit aims to be reliable, simple, performant, and standards compliant.
#     Copyright (C) 2026  Luis M. Santos, M.D.
#     Copyright (C) 2026  MedicalMasses L.L.C.
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
echo "Pushing Message through PIPEs!"

files=$(ls "$1"/*.hl7)

for f in $files; do
  echo "hyperfine --warmup 3 --runs 10 cat $f | ../target/release/rumtk-hl7-v2-parse"
  hyperfine --warmup 3 --runs 10 --export-json "$f.json" "cat $f | ../target/release/rumtk-hl7-v2-parse"
done

pushd "$1" || exit
for f in $files; do
  echo "perf record -F99 --call-graph dwarf cat $f | ../target/release/rumtk-hl7-v2-parse"
  perf record -F99 --call-graph dwarf -o "cat $f | ../target/release/rumtk-hl7-v2-parse"
  break
done
popd || exit

