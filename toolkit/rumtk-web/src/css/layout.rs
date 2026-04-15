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

pub const LAYOUT_CSS: &str = r"
            .header-default-container {
                position: fixed;
                top: 0;

                padding: 1em;

                display: flex;
                flex-direction: row;
                flex-wrap: wrap;

                align-items: center;
                justify-content: space-between;
                justify-items: center;

                background-color: var(--color-indigo);
                border-bottom: var(--color-turqoise) 0.1em solid;

                width: 100%;
                backdrop-filter: blur(5px);

                opacity: 0.9;
                height: fit-content;

                z-index: var(--top-layer);
            }

            .header-default-navlogo {
                position: relative;
                left: 0;
                min-width: 64px;

                display: flex;
                flex-direction: row;
                justify-content: space-around;
            }

            .header-default-navactions {
                position: relative;

                align-self: center;
                width: fit-content;
                min-width: 200px;

                display: flex;
                flex-direction: row;
                justify-content: space-around;
                justify-items: center;

                padding: 10px;
            }

            .header-default-misc {
                position: relative;
                right: 0;

                display: flex;
                flex-direction: row;
                justify-content: space-around;
                width: 17.5%;
            }
";
