/*
 *     rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 *     This toolkit aims to be reliable, simple, performant, and standards compliant.
 *     Copyright (C) 2026  Luis M. Santos, M.D. <lsantos@medicalmasses.com>
 *     Copyright (C) 2026  MedicalMasses L.L.C. <contact@medicalmasses.com>
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

const JS_SCRIPT_NEW_FILE_CACHE: &str = r"
    const fileCache = new Map();
";

const JS_SCRIPT_CACHE_UPLOAD_FILE: &str = r"
    document.getElementById('file').addEventListener('change', function(event) {
            const selectedFile = event.target.files[0];
            const file = {
                filename: '',
                contents: ''
            };
            if (selectedFile) {
              // You can use the FileReader API to read the contents if needed
              const reader = new FileReader();
              file.filename = selectedFile.name;

              // Define what happens when the file is loaded
              reader.onload = function(e) {
                file.contents = e.target.result;
                fileCache.set(file.filename, file);
              };

              // Read the file as text (or use readAsDataURL for images)
              reader.readAsDataURL(selectedFile);
            }
        }
    );
";

