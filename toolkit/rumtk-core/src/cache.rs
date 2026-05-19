/*
 * rumtk attempts to implement HL7 and medical protocols for interoperability in medicine.
 * This toolkit aims to be reliable, simple, performant, and standards compliant.
 * Copyright (C) 2024  Luis M. Santos, M.D. <lsantos@medicalmasses.com>
 * Copyright (C) 2025  MedicalMasses L.L.C. <contact@medicalmasses.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::base::RUMResult;
use crate::net::tcp::{AsyncOwnedRwLockReadGuard, AsyncOwnedRwLockWriteGuard};
use crate::threading::thread_primitives::SafeLock;
use crate::threading::threading_functions::{lock_read, new_lock, process_write_critical_section};
use crate::types::RUMHashMap;
use crate::{rumtk_critical_section_write, rumtk_lock_read, rumtk_new_lock};
use clap::builder::TypedValueParser;
use ::core::hash::Hash;
pub use once_cell::unsync::Lazy;
use std::fmt::Debug;
use std::sync::LazyLock;
/**************************** Constants**************************************/
/// I don't think most scenarios will need more than 10 items worth of memory pre-allocated at a time.
pub const DEFAULT_CACHE_PAGE_SIZE: usize = 10;
/**************************** Caches ****************************************/

/**************************** Types *****************************************/
///
/// Generic Cache store object. One use case will be to use a search string as the key and store
/// the search parsing object here.
///
pub type RUMCache<K, V> = RUMHashMap<K, V>;
pub type LockedCache<K, V> = SafeLock<RUMCache<K, V>>;
pub type LazyRUMCache<K, V> = LazyLock<LockedCache<K, V>>;
pub type RUMCacheValue<V> = AsyncOwnedRwLockReadGuard<V>;
pub type RUMCacheMutValue<V> = AsyncOwnedRwLockWriteGuard<V>;
pub type RUMCacheWriteGuard<K,V> = AsyncOwnedRwLockWriteGuard<RUMCache<K, V>>;

/**************************** Traits ****************************************/

/**************************** Helpers ***************************************/

pub const fn new_cache<K, V>() -> LazyRUMCache<K, V>
where
    K: Debug,
    V: Debug,
{
    LazyLock::new(|| {
        let cache = RUMCache::<K, V>::with_capacity(DEFAULT_CACHE_PAGE_SIZE);
        rumtk_new_lock!(cache)
    })
}

pub unsafe fn cache_push<K, V>(
    cache: *mut LazyRUMCache<K, V>,
    key: &K,
    val: V,
)
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    rumtk_critical_section_write!((**cache).clone(), |mut guard: RUMCacheWriteGuard<K,V>| {
        guard.insert(key.clone(), val.clone());
    })
}

pub unsafe fn cache_get<K, V>(
    cache: *mut LazyRUMCache<K, V>,
    key: &K,
) -> Option<V>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    let guard = rumtk_lock_read!(*cache);
    match (*guard).get(key) {
        Some(val) => Some(val.clone()),
        None => None
    }
}

pub unsafe fn cache_get_or_set<K, V, F>(
    cache: *mut LazyRUMCache<K, V>,
    key: &K,
    default_func: F
) -> RUMResult<V>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
    F: Fn() -> RUMResult<V>
{
    let val = match cache_get(cache, key) {
        Some(val) => val,
        None => {
            cache_push(cache, key, default_func()?);
            cache_get(cache, key).unwrap()
        }
    };
    Ok(val)
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
    /// use rumtk_core::base::RUMResult;
    ///
    /// type StringCache = LazyRUMCache<String, String>;
    ///
    /// fn init_cache(k: &String) -> RUMResult<String> {
    ///    Ok(String::from(k))
    /// }
    ///
    /// let mut cache: StringCache = new_cache();
    ///
    /// let test_key: String = String::from("Hello World");
    /// let v = rumtk_cache_fetch!(
    ///     &raw mut cache,
    ///     &test_key,
    ///     || {init_cache(&test_key)}
    /// ).unwrap();
    ///
    /// assert_eq!(test_key.as_str(), v.as_str(), "The inserted key is not the same to what was passed as input!");
    ///
    ///
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_cache_fetch {
        ( $cache:expr, $key:expr, $default_func:expr ) => {{
            use $crate::cache::{cache_get_or_set};
            // Do not remove the clippy disable decorator here since we do intend to expand within
            // the unsafe block. Expanding elsewhere prevents us from getting access to the cache's
            // internal references due to compiler error
            #[allow(clippy::macro_metavars_in_unsafe)]
            unsafe {
                cache_get_or_set($cache, $key, $default_func)
            }
        }};
    }

    ///
    /// Overrides contents in cache at key ```K```. No checks are performed for the existence of the
    /// key in the cache. Be careful not to override necessary data.
    ///
    /// ```
    /// use rumtk_core::{rumtk_cache_fetch, rumtk_cache_get, rumtk_cache_push};
    /// use rumtk_core::cache::{new_cache, LazyRUMCache, cache_push};
    /// use std::sync::Arc;
    /// use rumtk_core::base::RUMResult;
    ///
    /// type StringCache = LazyRUMCache<String, Vec<String>>;
    ///
    /// fn init_cache(k: &String) -> RUMResult<Vec<String>> {
    ///    Ok(vec![])
    /// }
    ///
    /// let mut cache: StringCache = new_cache();
    ///
    /// let test_key: String = String::from("Hello World");
    /// let test_value: Vec<String> = vec![String::from("?????")];
    ///
    /// rumtk_cache_fetch!(
    ///     &raw mut cache,
    ///     &test_key,
    ///     || {init_cache(&test_key)}
    /// );
    ///
    /// rumtk_cache_push!(
    ///     &raw mut cache,
    ///     &test_key,
    ///     test_value.clone()
    /// );
    ///
    /// let v = rumtk_cache_get!(
    ///     &raw mut cache,
    ///     &test_key
    /// ).unwrap();
    ///
    /// assert_eq!(test_value.get(0).unwrap(), v.get(0).unwrap(), "The inserted key is not the same to what was passed as input!");
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
    /// Retrieves the cached item in immutable mode.
    ///
    /// ```
    /// use rumtk_core::{rumtk_cache_fetch, rumtk_cache_push, rumtk_cache_get};
    /// use rumtk_core::cache::{new_cache, LazyRUMCache, cache_push, cache_get};
    /// use std::sync::Arc;
    /// use rumtk_core::base::RUMResult;
    ///
    /// type StringCache = LazyRUMCache<String, Vec<String>>;
    ///
    /// fn init_cache(k: &String) -> RUMResult<Vec<String>> {
    ///    Ok(vec![])
    /// }
    ///
    /// let mut cache: StringCache = new_cache();
    /// static DEFAULT_VEC: Vec<String> = vec![];
    ///
    /// let test_key: String = String::from("Hello World");
    /// let test_value: Vec<String> = vec![String::from("?????")];
    ///
    /// rumtk_cache_fetch!(
    ///     &raw mut cache,
    ///     &test_key,
    ///     || {init_cache(&test_key)}
    /// );
    ///
    /// rumtk_cache_push!(
    ///     &raw mut cache,
    ///     &test_key,
    ///     test_value.clone()
    /// );
    ///
    /// let v = rumtk_cache_get!(
    ///     &raw mut cache,
    ///     &test_key
    /// ).unwrap();
    ///
    /// assert_eq!(test_value.get(0).unwrap(), v.get(0).unwrap(), "The inserted key is not the same to what was passed as input!");
    ///
    ///
    /// ```
    ///
    #[macro_export]
    macro_rules! rumtk_cache_get {
        ( $cache:expr, $key:expr ) => {{
            use $crate::cache::cache_get;
            // Do not remove the clippy disable decorator here since we do intend to expand within
            // the unsafe block. Expanding elsewhere prevents us from getting access to the cache's
            // internal references due to compiler error
            #[allow(clippy::macro_metavars_in_unsafe)]
            unsafe {
                cache_get($cache, $key)
            }
        }};
    }
}
