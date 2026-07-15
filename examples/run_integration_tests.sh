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
echo "Resetting work directory..."
rm -rf ./demo/tmp/interface/*.out
rm -rf ./demo/tmp/interface/*.log

echo "Running Tests..."
echo "Running STD Interface test..."
./examples/stdin_interface.sh
sleep 5
echo "Running Echo Interface test..."
#./examples/echo_interface.sh
