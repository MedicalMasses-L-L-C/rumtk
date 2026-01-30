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

    .socials-default-container {
        display: flex;
        color: var(--color-bg-white);
        padding: 1em;
        filter: contrast(10%);
    }

    .text-card-default {
        max-width: 1700px;
        padding: 20px;
        background-color: var(--color-indigo);

        border-radius: 15px;
    }

    .title-default-container {
        display: block;
        height: 40px;
        align-content: center;
        margin-block: 20px 20px;
    }

    .title-default {
        display: block;
        margin-block: 0;
    }

    .title-default-overlay {
        position: relative;
        margin-block: 0;
        z-index: var(--mid-layer);
        bottom: 1.25em;

        background-image: var(--img-glitch-0);
        background-repeat: repeat;
        background-clip: text;
        color: transparent;
        background-position: center;
        filter: blur(5px);

        animation: slide 30s infinite linear;
    }

    .navlink:link, .navlink:visited {
        color: var(--color-navlink);
    }

    .navlink:hover {
        background-color: var(--color-darkpurple);
        border-radius: 10px;
    }

    .label-default {
        text-wrap: wrap;
        margin: auto;
    }

    .brand-name {
        background-image: linear-gradient(to right, var(--color-darkpurple), var(--color-ticklemepink), var(--color-cerulean), var(--color-turqoise));
        background-clip: text;
        color: transparent;
    }

    .formatted-label-default {
        text-wrap: wrap;
        margin: auto;
    }

    .footer-default-container, #footer {
        display: grid;
        background-color: var(--color-indigo);
        color: white;
        padding: 1em;
        justify-items: center;
    }

    .contact-centered-button-container {
        max-width: fit-content;
        margin-inline: auto;

        height: 90px;
    }

    .contact-centered-button {
        background: radial-gradient(circle,var(--color-darkpurple) 0%, var(--color-indigo) 70%);

        color: var(--color-bg-white);

        border-radius: 15px;
    }

    .logo {
        display: flex;
        justify-content: center;
        margin-bottom: 20px;
    }

    .logo-default {
        min-height: 200px;
    }

    .logo-small {
        width: 64px;
        height: auto;
    }

    .info-card-default-container {
        display: flex;
        min-height: 250px;
        min-width: 200px;
        width: 100%;
        padding: 10px;
    }

    .info-card-default-titlebox {
        display: flex;
        flex: 1;
        background-color: var(--color-indigo);
        align-items: end;
        padding: 10px;
        justify-content: center;

        max-width: fit-content;
    }

    .info-card-default-descbox {
        display: flex;
        flex: 2;
        background-color: var(--color-darkpurple);
        align-items: center;
        padding: 10px;
        justify-content: center;
    }

    @media (width < 700px) {
        .info-card-default-container {
            flex-direction: column;
            min-height: 400px;
        }

        .info-card-default-container > :first-child {
            transform: translate(0, 30px);
        }

        .info-card-default-titlebox {
            z-index: var(--mid-layer);
        }

        .info-card-default-descbox {
            margin: 0;
        }
    }
";
