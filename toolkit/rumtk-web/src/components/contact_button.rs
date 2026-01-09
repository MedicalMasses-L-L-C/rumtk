use crate::components::COMPONENTS;
use crate::utils::defaults::{DEFAULT_CONTACT_ITEM, DEFAULT_TEXT_ITEM, PARAMS_CSS_CLASS, PARAMS_FUNCTION, PARAMS_TYPE};
use crate::utils::types::{HTMLResult, MMString, SharedAppState, URLParams, URLPath};
use crate::{mm_get_text_item, mm_render_component, mm_render_html};
use askama::Template;

#[derive(Template, Debug)]
#[template(
    source = "
        <style>
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
        </style>
        {% if custom_css_enabled %}
            <link href="/static/components/contact_button.css" rel="stylesheet">
        {% endif %}
        <script type="module" id="contact_button">
            export function goto_contact() {
                window.location.href = './contact';
            }

            // @ts-ignore
            window.goto_contact = goto_contact;
        </script>
        <div class="contact-{{ css_class }}-button-container">
            <button class="contact-{{ css_class }}-button" onclick="{{ send_function }}()">
                {{title|safe}}
            </button>
        </div>
    ",
    ext = "html"
)]
pub struct ContactButton {
    title: MMString,
    typ: MMString,
    send_function: MMString,
    css_class: MMString,
}

pub fn contact_button(path_components: URLPath, params: URLParams, state: SharedAppState) -> HTMLResult {
    let typ = mm_get_text_item!(params, PARAMS_TYPE, DEFAULT_CONTACT_ITEM);
    let send_function = mm_get_text_item!(params, PARAMS_FUNCTION, DEFAULT_CONTACT_ITEM);
    let css_class = mm_get_text_item!(params, PARAMS_CSS_CLASS, DEFAULT_TEXT_ITEM);

    let title = mm_render_component!("title", [("type", typ)], state, COMPONENTS);

    mm_render_html!(
        ContactButton {
            title,
            typ: MMString::from(typ),
            send_function: MMString::from(send_function),
            css_class: MMString::from(css_class)
        }
    )
}
