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
use indexmap::IndexMap;
use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, RandomState};
use std::mem::MaybeUninit;

pub type ArenaVec<'a, V> = Vec<V, &'a Arena>;
pub type ArenaVecDeque<'a, V> = VecDeque<V, &'a Arena>;
pub type ArenaHashMap<'a, K, V> = HashMap<K, V, RandomState, &'a Arena>;

pub struct ArenaOrderedHashMap<'a, 'b, K, V> {
    order: ArenaVec<'a, &'b K>,
    data: ArenaHashMap<'a, K, V>
}

impl<'a, 'b, K, V> ArenaOrderedHashMap<'a, 'b, K, V> {
    pub fn new_in(arena: &Arena) -> Self {
        Self {
            order: ArenaVec::new_in(arena),
            data: ArenaHashMap::new_in(arena)
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            order: ArenaVec::with_capacity_in
        }
    }
}

///
/// Build Vector instance of type [ArenaVec] which is arena aware.
///
#[inline(always)]
pub fn new_vec<V>(arena: &Arena, len: Option<usize>) -> ArenaVec<V> {
    match len {
        Some(len) => ArenaVec::<V>::with_capacity_in(len, arena),
        None => ArenaVec::<V>::new_in(arena),
    }
}

///
/// Build a Vector instance of type [ArenaVec] which is arena aware using the items passed.
///
#[inline(always)]
pub fn new_vec_from<V, const N: usize>(data: [V; N], arena: &Arena) -> ArenaVec<V>
where
    V: Sized + Clone
{
    let mut v: ArenaVec<V> = new_vec(arena, Some(data.len()));
    v.extend(data);
    v
}

///
/// Build Queue instance of type [ArenaVecDeque] which is arena aware.
///
#[inline(always)]
pub fn new_vecdeque<V>(arena: &Arena, len: Option<usize>) -> ArenaVecDeque<V> {
    match len {
        Some(len) => ArenaVecDeque::<V>::with_capacity_in(len, arena),
        None => ArenaVecDeque::<V>::new_in(arena),
    }
}

///
/// Build a Queue instance of type [ArenaVecDeque] which is arena aware using the items passed.
///
#[inline(always)]
pub fn new_vecdeque_from<V, const N: usize>(data: [V; N], arena: &Arena) -> ArenaVecDeque<V>
where
    V: Sized + Clone
{
    let mut vd: ArenaVecDeque<V> = new_vecdeque(arena, Some(data.len()));
    vd.extend(data);
    vd
}

///
/// Build a Hash Table instance of type [ArenaHashMap] which is arena aware.
///
#[inline(always)]
pub fn new_hashmap<K, V>(arena: &Arena, len: Option<usize>) -> ArenaHashMap<K, V> {
    match len {
        Some(len) => ArenaHashMap::<K, V>::with_capacity_and_hasher_in(
            len,
            RandomState::new(),
            arena,
        ),
        None => ArenaHashMap::<K, V>::new_in(arena),
    }
}

///
/// Build a Hash Table instance of type [ArenaHashMap] which is arena aware using the items passed.
///
#[inline(always)]
pub fn new_hashmap_from<K, V, const N: usize>(data: [(K, V); N], arena: &Arena) -> ArenaHashMap<K, V>
where
    K: Sized + Clone + Eq + Hash,
    V: Sized + Clone
{
    let mut htable: ArenaHashMap<K, V> = new_hashmap(arena, Some(data.len()));
    for (k, v) in data {
        htable.insert(k, v);
    }
    htable
}

///
/// Build a Hash Table instance of type [ArenaHashMap] which is arena aware.
///
#[inline(always)]
pub fn new_orderedmap<K, V>(arena: &Arena, len: Option<usize>) -> ArenaHashMap<K, V> {
    match len {
        Some(len) => HashMap::<K, V, RandomState, &Arena>::with_capacity_and_hasher_in(
            len,
            RandomState::new(),
            arena,
        ),
        None => HashMap::<K, V, RandomState, &Arena>::new_in(arena),
    }
}

#[inline(always)]
pub fn new_box<V>(v: V, arena: &Arena) -> Box<V, &Arena> {
    vec![1,2,4];
    Box::<V, &Arena>::new_in(v, arena)
}

#[inline(always)]
pub fn new_box_uninit<V>(arena: &Arena) -> Box<MaybeUninit<V>, &Arena> {
    Box::new_uninit_in(arena)
}

#[inline(always)]
pub fn new_box_zeroed<V>(arena: &Arena) -> Box<MaybeUninit<V>, &Arena> {
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
