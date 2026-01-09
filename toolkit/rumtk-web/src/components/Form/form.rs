use crate::utils::defaults::{DEFAULT_NO_TEXT, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_TYPE};
use crate::utils::types::{HTMLResult, MMString, SharedAppState, URLParams, URLPath};
use crate::{mm_get_conf, mm_get_text_item, mm_render_html};
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
    typ: MMString,
    title: MMString,
    module_path: MMString,
    css_class: MMString,
}

pub fn form(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let typ = mm_get_text_item!(params, PARAMS_TYPE, DEFAULT_TEXT_ITEM);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let text_store = mm_get_conf!(SECTION_TITLES, DEFAULT_NO_TEXT);
    let en_text = mm_get_text_item!(&text_store, "0", &&phf_ordered_map!());
    let itm = mm_get_text_item!(&en_text, &typ, &&phf_ordered_map!());
    let title = MMString::from(mm_get_text_item!(&itm, "title", ""));

    mm_render_html!(Form {
        typ: MMString::from(typ),
        title,
        css_class: MMString::from(css_class),
    })
}
