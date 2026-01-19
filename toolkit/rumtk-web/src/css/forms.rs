/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2025  Luis M. Santos, M.D.
 * Copyright (C) 2025  Nick Stephenson
 * Copyright (C) 2025  Ethan Dixon
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
                max-width: 650px;

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
