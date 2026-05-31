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
use crate::Arena;
use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, RandomState};
use std::mem::MaybeUninit;

pub type ArenaVec<'a, T> = Vec<T, &'a Arena>;
pub type ArenaVecDeque<'a, T> = VecDeque<T, &'a Arena>;
pub type ArenaHashMap<'a, K, T> = HashMap<K, T, RandomState, &'a Arena>;

///
/// Build Vector instance of type [ArenaVec] which is arena aware.
///
#[inline(always)]
pub fn new_vec<T>(arena: &Arena, len: Option<usize>) -> ArenaVec<T> {
    match len {
        Some(len) => Vec::<T, &Arena>::with_capacity_in(len, arena),
        None => Vec::<T, &Arena>::new_in(arena),
    }
}

///
/// Build a Vector instance of type [ArenaVec] which is arena aware using the items passed.
///
#[inline(always)]
pub fn new_vec_from<'a, 'b, T>(data: &'a [T], arena: &'b Arena) -> ArenaVec<'b, T>
where
    T: Sized + Clone
{
    let mut v: ArenaVec<T> = new_vec(arena, Some(data.len()));
    for d in data {
        v.push(d.clone());
    }
    v
}

///
/// Build Queue instance of type [ArenaVecDeque] which is arena aware.
///
#[inline(always)]
pub fn new_vecdeque<T>(arena: &Arena, len: Option<usize>) -> ArenaVecDeque<T> {
    match len {
        Some(len) => VecDeque::<T, &Arena>::with_capacity_in(len, arena),
        None => VecDeque::<T, &Arena>::new_in(arena),
    }
}

///
/// Build a Queue instance of type [ArenaVecDeque] which is arena aware using the items passed.
///
#[inline(always)]
pub fn new_vecdeque_from<'a, 'b, T>(data: &'a [T], arena: &'b Arena) -> ArenaVecDeque<'b, T>
where
    T: Sized + Clone
{
    let mut vd: ArenaVecDeque<T> = new_vecdeque(arena, Some(data.len()));
    for d in data {
        vd.push_back(d.clone());
    }
    vd
}

///
/// Build a Hash Table instance of type [ArenaHashMap] which is arena aware.
///
#[inline(always)]
pub fn new_hashmap<K, T>(arena: &Arena, len: Option<usize>) -> ArenaHashMap<K, T> {
    match len {
        Some(len) => HashMap::<K, T, RandomState, &Arena>::with_capacity_and_hasher_in(
            len,
            RandomState::new(),
            arena,
        ),
        None => HashMap::<K, T, RandomState, &Arena>::new_in(arena),
    }
}

///
/// Build a Hash Table instance of type [ArenaHashMap] which is arena aware using the items passed.
///
#[inline(always)]
pub fn new_hashmap_from<'a, 'b, K, T>(data: &'a [(K, T)], arena: &'b Arena) -> ArenaHashMap<'b, K, T>
where
    K: Sized + Clone + Eq + Hash,
    T: Sized + Clone
{
    let mut htable: ArenaHashMap<K, T> = new_hashmap(arena, Some(data.len()));
    for (k, v) in data {
        htable.insert(k.clone(), v.clone());
    }
    htable
}

#[inline(always)]
pub fn new_box<T>(v: T, arena: &Arena) -> Box<T, &Arena> {
    vec![1,2,4];
    Box::<T, &Arena>::new_in(v, arena)
}

#[inline(always)]
pub fn new_box_uninit<T>(arena: &Arena) -> Box<MaybeUninit<T>, &Arena> {
    Box::new_uninit_in(arena)
}

#[inline(always)]
pub fn new_box_zeroed<T>(arena: &Arena) -> Box<MaybeUninit<T>, &Arena> {
    Box::new_zeroed_in(arena)
}

#[macro_export]
macro_rules! rumtk_arena_vec {
    ( $arena:expr ) => {{
        use $crate::collections::new_vec;
        new_vec($arena, None)
    }};
    ( $items:expr, $arena:expr ) => {{
        use $crate::collections::new_vec_from;

        new_vec_from($items, $arena)
    }};
}

#[macro_export]
macro_rules! rumtk_arena_vecdeque {
    ( $arena:expr ) => {{
        use $crate::collections::new_vecdeque;
        new_vecdeque($arena, None)
    }};
    ( $items:expr, $arena:expr ) => {{
        use $crate::collections::new_vecdeque_from;
        new_vecdeque_from($items, $arena)
    }};
}

#[macro_export]
macro_rules! rumtk_arena_hashmap {
    ( $arena:expr ) => {{
        use $crate::collections::new_hashmap;
        new_hashmap($arena, None)
    }};
    ( $items:expr, $arena:expr ) => {{
        use $crate::collections::new_hashmap_from;
        new_hashmap_from($items, $arena)
    }};
}

#[macro_export]
macro_rules! rumtk_arena_vec_type {
    ( $T:ty ) => {{
        use $crate::collections::ArenaVec;
        ArenaVec<$T>
    }};
}

#[macro_export]
macro_rules! rumtk_arena_vecdeque_type {
    ( $T:ty ) => {{
        use $crate::collections::ArenaVecDeque;
        ArenaVecDeque<$T>
    }};
}

#[macro_export]
macro_rules! rumtk_arena_hashmap_type {
    ( $K:ty, $T:ty ) => {{
        use $crate::collections::ArenaHashMap;
        ArenaHashMap<$K, $T>
    }};
}
