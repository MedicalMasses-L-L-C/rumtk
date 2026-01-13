/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D.
 * Copyright (C) 2025  MedicalMasses L.L.C.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
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
    ";
