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

use branches::prefetch_read_data;

pub const CPU_L1_PREFETCH: i32 = 0;
pub const CPU_L2_PREFETCH: i32 = 1;
pub const CPU_L3_PREFETCH: i32 = 2;
pub const CPU_NONTEMPORAL_PREFETCH: i32 = 3;
pub const DEFAULT_CPU_L1_CACHE_LINE_SIZE: usize = 64; // Number of bytes in a typical x86_64 CPU L1 cache line.
pub const DEFAULT_CPU_L1_CACHE_SIZE: usize = 32 * 1024; // Number of bytes in a typical x86_64 CPU L1 cache per core.
pub const DEFAULT_CPU_PAGE_SIZE: usize = 4 * 1024; // Typical CPU page size
pub const DEFAULT_AVX_SIMD_SIZE: usize = 32;

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