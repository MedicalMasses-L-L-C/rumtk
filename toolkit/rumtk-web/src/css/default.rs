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

pub const DEFAULT_CSS: &str = r"
        pre {
            max-width: 100%;
            white-space: pre-wrap;
            overflow-wrap: normal;
            text-wrap: wrap;
            word-wrap: break-word;
        }
        
        textarea {
            max-width: 100%;
        }
        
        img {
            max-width: 100%;
        }
        :root {
            --color-indigo: #431089;
            --color-turqoise: #49e2f4;
            --color-bg-pink: #b296da;
            --color-bg-light-pink: #c791c6;
            --color-bg-neutral: #e2a1b9;
            --color-bg-light-blue: #8bbbd7;
            --color-bg-white: #c3e4e8;
            --color-darkpurple: #93268F;
            --color-cerulean: #077ABD;
            --color-cerulean-light: #0982c9;
            --color-ticklemepink: #ff7bac;
            --color-jaguar: #0c0b0e;
        
            --color-nepal: #93a7b5;
            --color-snuff: #fed3fd;
            --color-heliotrope: #c46ded;
            --color-glacier: #72abc6;
            --color-magnolia: #f6edfb;
        
            --color-navlink: var(--color-bg-white);
        }
    ";
