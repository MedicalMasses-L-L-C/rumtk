/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D. <lsantos@medicalmasses.com>
 * Copyright (C) 2025  Ethan Dixon
 * Copyright (C) 2025  MedicalMasses L.L.C. <contact@medicalmasses.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

pub const BASIC_CSS: &str = r"

        :root {
            --lowest-layer: 0;
            --mid-layer: 50;
            --top-layer: 99;
        }

        .no-decoration, .undecorated {
            text-decoration: none;
        }

        .no-select {
            user-select: none;
        }

        .centered {
            max-width: fit-content;
            margin-inline: auto;
        }

        .twothird-width {
            width: 66vw;
        }

        .threequarter-width {
            width: 75vw;
        }

        .half-width {
            width: 50vw;
        }

        .padded-1x {
            padding: 1em;
        }

        .margin-1x {
            margin: 1em;
        }

        .md a {
            filter: invert();
        }

        .md td, .md th {
            padding: 5px;
        }
        
    ";
