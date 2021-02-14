use tokio::sync::RwLock;
use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;


/// TODO: Evaluate if crate `cached` should be used instead.
/// Put max size on cache.
/// Put timeout on cache entries.

pub struct Cache<K, V> {
    cache: RwLock<HashMap<K, V>>,
}

impl<K: Eq + Hash + Clone, V: Clone> Cache<K, V> {
    pub fn new() -> Self {
        Cache {
            cache: RwLock::new(HashMap::new()),
        }
    }

    pub async fn lookup(&self, key: &K) -> Option<V> {
        self.cache.read().await.get(key).cloned()
    }

    pub async fn store(&self, key: &K, value: V) {
        self.cache.write().await.insert(key.clone(), value);
    }
}
