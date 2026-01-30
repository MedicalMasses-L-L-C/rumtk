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

pub const LIST_CSS: &str = r"
    .item-default-container {
        max-width: 1000px;
        margin-inline: auto;
        padding: 10px;
    }

    .item-default-title {

    }

    .item-default-title:hover {
        background-color: var(--color-darkpurple);
    }

    .item-default-details {
        text-align: left;
        text-wrap: wrap;
    }

    .portrait-card-default-container {
        align-content: center;
        align-items: center;
        justify-items: center;
        max-width: 100%;
        width: fit-content;
        padding: 10px;
    }

    .portrait-card-default-container > table > tbody {
        display: flex;
        justify-items: center;
        flex-wrap: wrap;
        align-content: center;
        justify-content: center;
        flex-direction: column;
        vertical-align: top;
    }

    .portrait-card-default-row {
        display: flex;
        flex-wrap: wrap;
        /* min-width: 200px; */
        background-color: transparent;
        justify-content: space-around;
        justify-items: center;
        align-items: flex-start;
        flex-direction: column;
    }

    @media screen
    and (width > 700px) {
        .portrait-card-default-row {
            flex-direction: row;
        }
    }

    .portrait-card-default-item {
        width: 300px;
        padding: 50px;
    }
    
    .contact-card-default-portrait {
        height: 250px;
        border-radius: 5%;
        border-width: 5px;
        border-style: groove;
        border-color: var(--color-darkpurple);
    }

    .contact-card-default-container {
    }

    .contact-card-default-container > p {
        margin: 0;
    }

    .contact-card-centered-container {
        max-width: fit-content;
        margin-inline: auto;
    }

    .contact-card-centered-container > p {
        margin: 0;
    }
";
