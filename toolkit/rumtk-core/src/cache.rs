/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2024  Luis M. Santos, M.D.
 * Copyright (C) 2025  MedicalMasses L.L.C.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
 */

pub use ahash::AHashMap;
use core::hash::Hash;
pub use once_cell::unsync::Lazy;
use std::sync::{Arc, MappedRwLockReadGuard, RwLock, RwLockReadGuard};
/**************************** Constants**************************************/
/// I don't think most scenarios will need more than 10 items worth of memory pre-allocated at a time.
pub const DEFAULT_CACHE_PAGE_SIZE: usize = 10;
/**************************** Caches ****************************************/

/**************************** Types *****************************************/
///
/// Generic Cache store object. One use case will be to use a search string as the key and store
/// the search parsing object here.
///
pub type RUMCache<K, V> = AHashMap<K, V>;
pub type LazyRUMCache<K, V> = Lazy<Arc<RwLock<RUMCache<K, V>>>>;
pub type LazyRUMCacheValue<V> = MappedRwLockReadGuard<'static, V>;

/**************************** Traits ****************************************/

/**************************** Helpers ***************************************/
macro_rules! cache_unwrap {
    ( $k:expr ) => {{
        |d| {
            d.get($k).expect("Item not found in cache despite having inserted it previously within this scope! This is completely unexpected and a fatal bug!")
        }
    }};
    ( $k:expr, $default:expr ) => {{
        |d| {
            match d.get($k) {
                Some(val) => val,
                None => $default
            }
        }
    }};
}
pub const fn new_cache<K, V>() -> LazyRUMCache<K, V> {
    LazyRUMCache::new(|| {
        Arc::new(RwLock::new(RUMCache::with_capacity(
            DEFAULT_CACHE_PAGE_SIZE,
        )))
    })
}

pub unsafe fn get_or_set_from_cache<K, V, F>(
    cache: *mut LazyRUMCache<K, V>,
    expr: &K,
    new_fn: F,
) -> LazyRUMCacheValue<V>
where
    K: Hash + Eq + Clone + 'static,
    V: Clone,
    F: Fn(&K) -> V,
{
    let cache_entity = &mut *cache;
    if !cache_entity.read().unwrap().contains_key(&expr) {
        let cache_ref = Arc::get_mut(cache_entity).unwrap();
        cache_ref
            .write()
            .unwrap()
            .insert(expr.clone(), new_fn(&expr).clone());
    }
    RwLockReadGuard::map(cache_entity.read().unwrap(), cache_unwrap!(expr))
}

pub unsafe fn cache_push<K, V>(
    cache: *mut LazyRUMCache<K, V>,
    expr: &K,
    val: &V,
) -> LazyRUMCacheValue<V>
where
    K: Hash + Eq + Clone + 'static,
    V: Clone,
{
    let cache_entity = &mut *cache;
    cache_entity
        .write()
        .unwrap()
        .insert(expr.clone(), val.clone());
    RwLockReadGuard::map(cache_entity.read().unwrap(), cache_unwrap!(expr))
}

pub unsafe fn cache_get<K, V>(
    cache: *mut LazyRUMCache<K, V>,
    expr: &K,
    default: &'static V,
) -> LazyRUMCacheValue<V>
where
    K: Hash + Eq + Clone + 'static,
    V: Clone,
{
    let cache_entity = &mut *cache;
    let cache_ref = cache_entity.read().unwrap();
    RwLockReadGuard::map(cache_ref, cache_unwrap!(expr, default))
}

pub mod cache_macros {
    ///
    /// Searches for item in global cache. If global cache lacks item, create item using factory
    /// function passed to this macro.
    ///
    /// ```
    /// use rumtk_core::rumtk_cache_fetch;
    /// use rumtk_core::cache::{new_cache, LazyRUMCache};
    /// use std::sync::Arc;
    ///
    /// type StringCache = LazyRUMCache<String, String>;
    ///
    /// fn init_cache(k: &String) -> String {
    ///    String::from(k)
    /// }
    ///
    /// let mut cache: StringCache = new_cache();
    ///
    /// let test_key: String = String::from("Hello World");
    /// let v = rumtk_cache_fetch!(
    ///     &raw mut cache,
    ///     &test_key,
    ///     init_cache
    /// );
    ///
    /// assert_eq!(test_key.as_str(), v.as_str(), "The inserted key is not the same to what was passed as input!");
    ///
    ///
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_cache_fetch {
        ( $cache:expr, $key:expr, $func:expr ) => {{
            use $crate::cache::get_or_set_from_cache;
            // Do not remove the clippy disable decorator here since we do intend to expand within
            // the unsafe block. Expanding elsewhere prevents us from getting access to the cache's
            // internal references due to compiler error
            #[allow(clippy::macro_metavars_in_unsafe)]
            unsafe {
                get_or_set_from_cache($cache, $key, $func)
            }
        }};
    }

    ///
    /// Overrides contents in cache at key ```K```. No checks are performed for the existence of the
    /// key in the cache. Be careful not to override necessary data.
    ///
    /// ```
    /// use rumtk_core::{rumtk_cache_fetch, rumtk_cache_push};
    /// use rumtk_core::cache::{new_cache, LazyRUMCache, cache_push};
    /// use std::sync::Arc;
    ///
    /// type StringCache = LazyRUMCache<String, Vec<String>>;
    ///
    /// fn init_cache(k: &String) -> Vec<String> {
    ///    vec![]
    /// }
    ///
    /// let mut cache: StringCache = new_cache();
    ///
    /// let test_key: String = String::from("Hello World");
    /// let test_value: String = String::from("?????");
    ///
    /// rumtk_cache_fetch!(
    ///     &raw mut cache,
    ///     &test_key,
    ///     init_cache
    /// );
    ///
    /// let v = rumtk_cache_push!(
    ///     &raw mut cache,
    ///     &test_key,
    ///     &vec![test_value.clone()]
    /// );
    ///
    /// assert_eq!(test_value.as_str(), v.get(0).unwrap().as_str(), "The inserted key is not the same to what was passed as input!");
    ///
    ///
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_cache_push {
        ( $cache:expr, $key:expr, $val:expr ) => {{
            use $crate::cache::cache_push;
            // Do not remove the clippy disable decorator here since we do intend to expand within
            // the unsafe block. Expanding elsewhere prevents us from getting access to the cache's
            // internal references due to compiler error
            #[allow(clippy::macro_metavars_in_unsafe)]
            unsafe {
                cache_push($cache, $key, $val)
            }
        }};
    }

    ///
    /// Overrides contents in cache at key ```K```. No checks are performed for the existence of the
    /// key in the cache. Be careful not to override necessary data.
    ///
    /// ```
    /// use rumtk_core::{rumtk_cache_fetch, rumtk_cache_push, rumtk_cache_get};
    /// use rumtk_core::cache::{new_cache, LazyRUMCache, cache_push, cache_get};
    /// use std::sync::Arc;
    ///
    /// type StringCache = LazyRUMCache<String, Vec<String>>;
    ///
    /// fn init_cache(k: &String) -> Vec<String> {
    ///    vec![]
    /// }
    ///
    /// let mut cache: StringCache = new_cache();
    /// static DEFAULT_VEC: Vec<String> = vec![];
    ///
    /// let test_key: String = String::from("Hello World");
    /// let test_value: String = String::from("?????");
    ///
    /// rumtk_cache_fetch!(
    ///     &raw mut cache,
    ///     &test_key,
    ///     init_cache
    /// );
    ///
    /// rumtk_cache_push!(
    ///     &raw mut cache,
    ///     &test_key,
    ///     &vec![test_value.clone()]
    /// );
    ///
    /// let v = rumtk_cache_get!(
    ///     &raw mut cache,
    ///     &test_key,
    ///     &DEFAULT_VEC
    /// );
    ///
    /// assert_eq!(test_value.as_str(), v.get(0).unwrap().as_str(), "The inserted key is not the same to what was passed as input!");
    ///
    ///
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_cache_get {
        ( $cache:expr, $key:expr, $default_function:expr ) => {{
            use $crate::cache::cache_get;
            // Do not remove the clippy disable decorator here since we do intend to expand within
            // the unsafe block. Expanding elsewhere prevents us from getting access to the cache's
            // internal references due to compiler error
            #[allow(clippy::macro_metavars_in_unsafe)]
            unsafe {
                cache_get($cache, $key, $default_function)
            }
        }};
    }
}
