#!/bin/bash
#
#     rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
#     This toolkit aims to be reliable, simple, performant, and standards compliant.
#     Copyright (C) 2025  Luis M. Santos, M.D.
#     Copyright (C) 2025  MedicalMasses L.L.C.
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

if [ -f ./demo/tmp/interface/out.log ]; then
  rm -r ./demo/tmp/interface/out.log
fi

echo "Setting up Interface Chain"
./target/release/rumtk-v2-interface --port 55555 --local > ./demo/tmp/interface/out.log &
sleep 1
./target/release/rumtk-v2-interface --port 55556 --local | ./target/release/rumtk-v2-interface --outbound --port 55555 --local &
sleep 1

echo "Pushing Message through PIPEs!"
cat ./examples/hl7/sample_hl7.hl7 | ./target/release/rumtk-v2-interface --outbound --local --port 55556

echo "Clean up"
sleep 1
pkill -i -e -f rumtk-v2-interface
sleep 10
sync ./demo/tmp/interface/out.log
cat ./demo/tmp/interface/out.log
sleep 1

echo "Output"
#DIFF=$( diff <(jq -S . examples/sample_hl7.json) <(jq -S . demo/tmp/interface/out.log) )
DIFF=$( diff <(cat ./examples/hl7/sample_hl7.hl7) <(cat ./demo/tmp/interface/out.log) )

if [ "$DIFF" != "" ]; then
    echo "Values mismatch!"
    echo ">>>>>>>>>>>>>>>>Input"
    cat -A ./examples/hl7/sample_hl7.hl7
    echo ""
    echo ">>>>>>>>>>>>>>>>Output"
    cat -A ./demo/tmp/interface/out.log
    echo ""
    echo ">>>>>>>>>>>>>>>>Diff"
    echo "$DIFF"
    exit 69
fi
