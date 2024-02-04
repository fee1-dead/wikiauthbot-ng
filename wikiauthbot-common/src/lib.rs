use std::mem::replace;
use std::sync::Arc;
use std::time::Duration;
use std::hash::Hash;

use dashmap::mapref::entry::Entry;
use dashmap::DashMap;

struct Counter<T> {
    id: u8,
    inner: T,
}

/// A hashmap that has an expiry for each entries.
pub struct ExpiringHashMap<K, V> {
    map: Arc<DashMap<K, Counter<V>>>,
    expiry: Duration,
}

impl<K: Eq + Hash + Clone + Send + Sync + 'static, V: Send + Sync + 'static> ExpiringHashMap<K, V> {
    pub fn new(expiry: Duration) -> Self {
        Self {
            map: Arc::new(DashMap::new()),
            expiry,
        }
    }

    pub fn insert(&self, k: K, v: V) -> Option<V> {
        let mut ret = None;
        let id = match self.map.entry(k.clone()) {
            Entry::Occupied(mut occ) => {
                let val = occ.get_mut();
                val.id = val.id.wrapping_add(1);
                ret = Some(replace(&mut val.inner, v));
                val.id
            }
            Entry::Vacant(entry) => {
                entry.insert(Counter { id: 0, inner: v });
                0
            }
        };
        let expiry = self.expiry;
        let map = self.map.clone();
        tokio::spawn(async move {
            tokio::time::sleep(expiry).await;
            if let Entry::Occupied(occ) = map.entry(k) {
                if occ.get().id == id {
                    occ.remove_entry();
                }
            }
        });
        ret
    }
}