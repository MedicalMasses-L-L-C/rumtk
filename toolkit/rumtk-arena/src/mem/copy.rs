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

use crate::cpu::CPU_SIMD_64_SIZE;

#[inline]
pub fn copy_simd_slice<'a, const LANE_SIZE: usize>(src: &[u8], mut dst: &'a mut [u8]) -> &'a mut [u8] {
    let (prefix, middle, postfix) = src.as_simd::<LANE_SIZE>();
    let prefix_len = prefix.len();
    let postfix_len = postfix.len();

    dst[..prefix_len].copy_from_slice(prefix);
    dst = &mut dst[prefix_len..];

    for chunk in middle.into_iter() {
        chunk.copy_to_slice(&mut dst[..LANE_SIZE]);
        dst = &mut dst[LANE_SIZE..];
    }

    dst[..postfix_len].copy_from_slice(postfix);
    dst
}

#[inline]
pub fn copy_from_slice<'a>(src: &[u8], dst: &'a mut [u8]) -> &'a mut [u8] {
    debug_assert!(src.len() <= dst.len(), "Destination memory slice is smaller than source! This is a bug near the call site of copy_from_slice!");
    copy_simd_slice::<CPU_SIMD_64_SIZE>(
        src,
        dst,
    )
}

#[macro_export]
macro_rules! rumtk_mem_quick_array_init {
    ( $typ:ty, $size:expr ) => {{
        use ::std::mem;
        const DATA_SLICE_LEN: usize = $size * size_of::<$typ>();
        let arr: [$typ; $size] = unsafe { mem::transmute([0u8; DATA_SLICE_LEN]) };
        arr
    }};
    ( $typ:ty, $size:expr, $default:expr ) => {{
        use ::std::mem;
        let arr: [$typ; $size] = [const {$default}; $size];
        arr
    }};
}
