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
use core::fmt::{Debug, Formatter, Write};
use std::collections::hash_map::Values;
use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::hash::{Hash, RandomState};
use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut};

pub type ArenaVec<'a, V> = Vec<V, &'a Arena>;
pub type ArenaVecDeque<'a, V> = VecDeque<V, &'a Arena>;
pub type ArenaHashMap<'a, K, V> = HashMap<K, V, RandomState, &'a Arena>;

#[derive(Clone)]
pub struct ArenaOrderedHashMap<'a, K, V> {
    order: ArenaVec<'a, K>,
    data: ArenaHashMap<'a, K, V>
}

impl<'a, 'b, K, V> ArenaOrderedHashMap<'a, K, V>
where
    K: Eq + Hash + Clone,
    V: Clone + PartialEq
{
    pub fn new_in(arena: &'a Arena) -> Self {
        Self {
            order: new_vec(arena, None),
            data: new_hashmap(arena, None)
        }
    }

    pub fn with_capacity(capacity: usize, arena: &'a Arena) -> Self {
        Self {
            order: new_vec(arena, Some(capacity)),
            data: new_hashmap(arena, Some(capacity))
        }
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if !self.data.contains_key(&key) {
            self.order.push(key.clone())
        }
        self.data.insert(key, value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let indx = self.order.iter().position(|v: &K| v == key)?;
        self.order.remove(indx);
        self.data.remove(key)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.data.get_mut(key)
    }

    pub fn len(&self) -> usize {
        self.order.len()
    }

    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }

    pub fn clear(&mut self) {
        self.order.clear();
        self.data.clear();
    }

    pub fn keys(&self) -> &ArenaVec<K> {
        &self.order
    }

    pub fn values(&self) -> Values<K, V> {
        self.data.values()
    }

    fn eq_values(&self, other: &Self) -> bool {
        let mut equal = true;
        for k in self.keys() {
            equal = equal && self[k] == other[k];
        }
        equal
    }
}

impl<'a, 'b, K, V> PartialEq for ArenaOrderedHashMap<'a, K, V>
where
    K: Eq + Hash + Clone,
    V: Clone + PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.order == other.order && self.eq_values(other)
    }
}

impl<'a, 'b, K, V> Index<&K> for ArenaOrderedHashMap<'a, K, V>
where
    K: Eq + Hash + Clone,
    V: Clone + PartialEq
{
    type Output = V;
    fn index(&self, index: &K) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<'a, 'b, K, V> IndexMut<&K> for ArenaOrderedHashMap<'a, K, V>
where
    K: Eq + Hash + Clone,
    V: Clone + PartialEq
{
    fn index_mut(&mut self, index: &K) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<'a, 'b, K, V> Debug for ArenaOrderedHashMap<'a, K, V>
where
    K: Debug + std::cmp::Eq + Hash,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        f.write_str("{ ")?;
        for k in self.order.iter() {
            let v = &self.data[k];
            f.write_str(&format!("{:?}: {:?}, ", k, v));
        }
        f.write_str("}")?;
        Ok(())
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
/// Build an Ordered Hash Table instance of type [ArenaOrderedHashMap] which is arena aware.
///
#[inline(always)]
pub fn new_orderedhashmap<K, V>(arena: &Arena, len: Option<usize>) -> ArenaOrderedHashMap<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone + PartialEq
{
    match len {
        Some(len) => ArenaOrderedHashMap::<K, V>::with_capacity(
            len,
            arena,
        ),
        None => ArenaOrderedHashMap::<K, V>::new_in(arena),
    }
}

///
/// Build an Ordered Hash Table instance of type [ArenaOrderedHashMap] which is arena aware using the items passed.
///
#[inline(always)]
pub fn new_orderedhashmap_from<K, V, const N: usize>(data: [(K, V); N], arena: &Arena) -> ArenaOrderedHashMap<K, V>
where
    K: Sized + Clone + Eq + Hash,
    V: Sized + Clone + PartialEq
{
    let mut htable: ArenaOrderedHashMap<K, V> = new_orderedhashmap(arena, Some(data.len()));
    for (k, v) in data {
        htable.insert(k, v);
    }
    htable
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

#[macro_export]
macro_rules! rumtk_arena_orderedhashmap {
    ( $arena:expr ) => {{
        use $crate::collections::new_orderedhashmap;
        new_orderedhashmap($arena, None)
    }};
    ( $items:expr, $arena:expr ) => {{
        use $crate::collections::new_orderedhashmap_from;
        new_orderedhashmap_from($items, $arena)
    }};
}
