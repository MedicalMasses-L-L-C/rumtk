use crate::mm_render_html;
use crate::utils::types::HTMLResult;
use askama::Template;

#[derive(Debug)]
pub struct FontAwesomeCSSElement {
    file: &'static str,
    version: &'static str,
    sha: &'static str,
}

#[derive(Template, Debug)]
#[template(
    source = "
        {% for e in elements %}
            <link rel='stylesheet' href='https://cdnjs.cloudflare.com/ajax/libs/font-awesome/{{e.version}}/css/{{e.file}}' integrity='{{e.sha}}' crossorigin='anonymous' referrerpolicy='no-referrer' onerror='this.onerror=null;this.href=\'/static/fontawesome-free/css/{{e.file}}\';' />
        {% endfor %}
    ",
    ext = "html"
)]
pub struct FontAwesome {
    elements: Vec<FontAwesomeCSSElement>,
}

pub fn fontawesome() -> HTMLResult {
    let elements = vec![
        FontAwesomeCSSElement {
            file: "fontawesome.min.css",
            version: "7.0.1",
            sha: "sha512-M5Kq4YVQrjg5c2wsZSn27Dkfm/2ALfxmun0vUE3mPiJyK53hQBHYCVAtvMYEC7ZXmYLg8DVG4tF8gD27WmDbsg==",
        },
        FontAwesomeCSSElement {
            file: "regular.min.css",
            version: "7.0.1",
            sha: "sha512-x3gns+l9p4mIK7vYLOCUoFS2P1gavFvnO9Its8sr0AkUk46bgf9R51D8xeRUwCSk+W93YbXWi19BYzXDNBH5SA==",
        },
        FontAwesomeCSSElement {
            file: "brands.min.css",
            version: "7.0.1",
            sha: "sha512-WxpJXPm/Is1a/dzEdhdaoajpgizHQimaLGL/QqUIAjIihlQqlPQb1V9vkGs9+VzXD7rgI6O+UsSKl4u5K36Ydw==",
        },
    ];

    mm_render_html!(FontAwesome { elements })
}
