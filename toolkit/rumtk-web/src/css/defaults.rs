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

pub const DEFAULT_OUT_CSS_DIR: &str = "./static/css";
pub const DEFAULT_OUT_CSS: &str = "bundle.min.css";
pub const DEFAULT_CSS: &str = concat!(
    include_str!("animations.css"),
    include_str!("basic.css"),
    include_str!("components.css"),
    include_str!("default.css"),
    include_str!("fonts.css"),
    include_str!("forms.css"),
    include_str!("gap.css"),
    include_str!("imgs.css"),
    include_str!("index.css"),
    include_str!("layout.css"),
    include_str!("media.css"),
    include_str!("theme.css"),
);