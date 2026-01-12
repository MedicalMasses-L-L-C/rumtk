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
use crate::utils::defaults::{DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_TYPE};
use crate::utils::types::{HTMLResult, RUMString, SharedAppConf, URLParams, URLPath};
use crate::{rumtk_web_get_string, rumtk_web_get_text_item, rumtk_web_render_html};
use askama::Template;
use phf_macros::phf_ordered_map;

#[derive(Template, Debug)]
#[template(
    source = "
        <style>
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
        </style>
        {% if custom_css_enabled %}
            <link href='/static/components/form/form.css' rel='stylesheet'>
        {% endif %}
        <script type='module' id='form-script' src='{{module_path}}'>
        </script>
            {% for element in elements %}
                {{ element|safe }}
            {% endfor %}
        </form>
    ",
    ext = "html"
)]
struct Form {
    typ: RUMString,
    title: RUMString,
    module_path: RUMString,
    css_class: RUMString,
    custom_css_enabled: bool,
}

pub fn form(path_components: URLPath, params: URLParams, state: SharedAppConf) -> HTMLResult {
    let typ = rumtk_web_get_text_item!(params, PARAMS_TYPE, DEFAULT_TEXT_ITEM);
    let css_class = rumtk_web_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let custom_css_enabled = state.lock().expect("Lock failure").custom_css;

    let text_store = rumtk_web_get_string!(SECTION_TITLES, DEFAULT_NO_TEXT);
    let en_text = rumtk_web_get_text_item!(&text_store, "0", &&phf_ordered_map!());
    let itm = rumtk_web_get_text_item!(&en_text, &typ, &&phf_ordered_map!());
    let title = RUMString::from(rumtk_web_get_text_item!(&itm, "title", ""));

    rumtk_web_render_html!(Form {
        typ: RUMString::from(typ),
        title,
        css_class: RUMString::from(css_class),
        custom_css_enabled
    })
}
