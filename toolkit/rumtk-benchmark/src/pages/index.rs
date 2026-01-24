/*
 *     rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 *     This toolkit aims to be reliable, simple, performant, and standards compliant.
 *     Copyright (C) 2026  Luis M. Santos, M.D.
 *
 *     This program is free software: you can redistribute it and/or modify
 *     it under the terms of the GNU General Public License as published by
 *     the Free Software Foundation, either version 3 of the License, or
 *     (at your option) any later version.
 *
 *     This program is distributed in the hope that it will be useful,
 *     but WITHOUT ANY WARRANTY; without even the implied warranty of
 *     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *     GNU General Public License for more details.
 *
 *     You should have received a copy of the GNU General Public License
 *     along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use rumtk_web::rumtk_web_render_component;
use rumtk_web::utils::*;

const APP_SCRIPT: &str = r"
    var globalCache = new Map();

    document.getElementById('file').addEventListener('change', function(event) {
            const selectedFile = event.target.files[0];
            if (selectedFile) {
              // You can use the FileReader API to read the contents if needed
              const reader = new FileReader();

              // Define what happens when the file is loaded
              reader.onload = function(e) {
                const contents = e.target.result;
                globalCache.set('pdf', contents);
                console.log(globalCache);
              };

              // Read the file as text (or use readAsDataURL for images)
              FileReader.readAsDataURL();
            }
        }
    );
";

pub fn index(app_state: SharedAppState) -> RenderedPageComponents {
    let upload_form = rumtk_web_render_component!(
        "form",
        [
            ("type", "upload"),
            ("title", "welcome"),
            ("target", "progress_hidden"),
            ("endpoint", "pdf")
        ],
        app_state.clone()
    );
    let cache_script =
        rumtk_web_render_component!("script", [("contents", APP_SCRIPT)], app_state.clone());

    vec![upload_form, cache_script]
}
