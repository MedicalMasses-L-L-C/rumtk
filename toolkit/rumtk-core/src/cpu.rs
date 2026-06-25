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

pub use branches::{likely as cpu_likely_branch, prefetch_read_data, unlikely as cpu_unlikely_branch};
pub use std::simd::prelude::*;

pub const CPU_L1_PREFETCH: i32 = 0;
pub const CPU_L2_PREFETCH: i32 = 1;
pub const CPU_L3_PREFETCH: i32 = 2;
pub const CPU_NONTEMPORAL_PREFETCH: i32 = 3;
pub const CPU_L1_CACHE_LINE_SIZE: usize = 64; // Number of bytes in a typical x86_64 CPU L1 cache line.
pub const CPU_L1_CACHE_SIZE: usize = 32 * 1024; // Number of bytes in a typical x86_64 CPU L1 cache per core.
pub const CPU_PAGE_SIZE: usize = 4 * 1024; // Typical CPU page size
pub const CPU_SIMD_64_SIZE: usize = 64;
pub const CPU_SIMD_32_SIZE: usize = 32;
pub const CPU_SIMD_16_SIZE: usize = 16;
pub const CPU_SIMD_8_SIZE: usize = 8;
pub const CPU_SEARCH_WINDOW_256_SIZE: usize = 256;
pub const CPU_SEARCH_WINDOW_128_SIZE: usize = 128;
pub const CPU_SEARCH_WINDOW_64_SIZE: usize = 64;
pub const CPU_SEARCH_WINDOW_32_SIZE: usize = 32;
pub const CPU_SEARCH_WINDOW_16_SIZE: usize = 16;


pub type u8xN<const SEARCH_WINDOW_SIZE: usize> = Simd<u8, SEARCH_WINDOW_SIZE>;

#[inline]
pub fn cpu_l3_prefetch(data: *const u8) {
    prefetch_read_data::<u8, CPU_L3_PREFETCH>(data);
}

#[inline]
pub fn cpu_l2_prefetch(data: *const u8) {
    prefetch_read_data::<u8, CPU_L2_PREFETCH>(data);
}

#[inline]
pub fn cpu_l1_prefetch(data: *const u8) {
    prefetch_read_data::<u8, CPU_L1_PREFETCH>(data);
}

#[inline(always)]
pub fn cpu_slice_to_array<const SLICE_SIZE: usize>(chunk: &[u8]) -> &[u8; SLICE_SIZE] {
    chunk.try_into().expect("length mismatch")
}

#[inline(always)]
pub fn cpu_find_fallback(chunk: &[u8], byte: u8) -> Option<usize> {
    chunk.iter().position(|c| *c==byte)
}

#[inline]
fn cpu_find_simd_avx2_n<const SEARCH_WINDOW_SIZE: usize>(data_vec: &u8xN<SEARCH_WINDOW_SIZE>, target: u8xN<SEARCH_WINDOW_SIZE>) -> Option<usize> {
    let mask = data_vec.simd_eq(target);

    if mask.any() {
        let bitmask = mask.to_bitmask();
        let lane_i = bitmask.trailing_zeros() as usize;
        return Some(lane_i);
    }

    None
}

#[inline]
pub fn cpu_find_simd_n<const LANE_SIZE: usize>
(
    chunk: &[u8],
    byte: u8,
) -> Option<usize>
{
    let mask = u8xN::<LANE_SIZE>::splat(byte);
    let (prefix, middle, postfix) = chunk.as_simd::<LANE_SIZE>();

    match cpu_find_fallback(prefix, byte) {
        Some(lane_i) => return Some(lane_i),
        None => {},
    }

    for (i, window) in middle.into_iter().enumerate() {
        match cpu_find_simd_avx2_n::<LANE_SIZE>(window, mask) {
            Some(lane_i) => {
                return Some(prefix.len() + (i * LANE_SIZE) + lane_i)
            },
            None => continue,
        }
    }

    match cpu_find_fallback(postfix, byte) {
        Some(lane_i) => Some(prefix.len() + (middle.len() * LANE_SIZE) + lane_i),
        None => None,
    }
}

#[inline]
pub fn cpu_find_simd(window: &[u8], byte: u8) -> Option<usize> {
    cpu_l3_prefetch(window.as_ptr());
    cpu_find_simd_n::<CPU_SIMD_64_SIZE>(
        window,
        byte,
    )
}