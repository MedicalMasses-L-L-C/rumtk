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

pub const FORM_CSS: &str = r"
            :root {
                --white: #ffffff;
                --tropical-green: #00755E;
                --spanish-red: #E60026;
            }

            .form-default-container {
                display: flex;
                flex-direction: column;

                background-color: var(--color-indigo);
                border-radius: 10px;

                width: 70%;
                min-width: 200px;
                max-width: 1000px;

                justify-items: center;
                justify-content: center;
                align-items: center;
                place-items: center;

                padding: 20px;
            }

            .form-default-container > input{
                width: 80%;
            }

            .form-default-container > input:invalid{
                background-color: var(--spanish-red);
                color: var(--white);
            }

            .form-default-container > input:invalid::placeholder{
                color: var(--white);
            }

            .form-default-container > input:valid{
                background-color: var(--tropical-green);
                color: var(--white);
            }

            .form-default-container > input:valid::placeholder{
                color: var(--white);
            }

            .form-default-container > textarea{
                min-width: 90%;
                min-height: 300px;
                object-fit: scale-down;
            }

            .form-default-container > #submit {
                width: 200px;
                height: 100px;

                border-radius: 10px;
            }

            form:valid > #submit {
                opacity: 1.0;
                pointer-events: auto;
                background-color: var(--tropical-green);
                color: var(--white);
            }

            form:invalid > #submit {
                opacity: 0.5;
                pointer-events: none;
                background-color: var(--spanish-red);
                color: var(--white);
            }
";
