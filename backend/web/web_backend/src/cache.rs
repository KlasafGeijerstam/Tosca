use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use tokio::sync::RwLock;
use std::sync::Arc;

/// Put max size on cache.
/// Put timeout on cache entries.

struct MaxSize<T> {
    max_size: usize,
    filo: Vec<T>,
    index: usize
}

// Emluate queue where we can save the Keys from cache
// As long as the queue is not filled, we save all the data

// When the queue hits maximum, we will start replacing the
// oldest entries.
impl<T: Clone> MaxSize<T> {
    fn new(max_size: usize) -> Self {
        MaxSize {
            max_size,
            // First in last out
            // Maybe should be stack alloacted?
            filo: Vec::new(),
            index: 0
        }
    }

    // If the cache is filled. Return a key to data that should be
    // removed
    fn put(&mut self, data: T) -> Option<T> {
        // if the cache has not been filled yet
        if self.filo.len() < self.max_size {
            self.filo.push(data);
            self.index += 1;
            None
        } else {
            // If we are at the last index, set index to start position
            if self.is_max() {
                self.index = 0;
            }
            
            let ret = self.filo.get(self.index).unwrap().clone();
            self.filo[self.index] = data;
            Some(ret)
        }
    }

    fn is_max(&self) -> bool {
        self.filo.len() == self.max_size
    }

}



pub struct Cache<K, V> {
    cache: RwLock<HashMap<K, Arc<V>>>,
    max_size: Option<RwLock<MaxSize<K>>>,
    timeout: Option<u32>,
}

impl<K: Eq + Hash + Clone, V: Clone> Cache<K, V> {
    pub fn new() -> Self {
        Cache {
            cache: RwLock::new(HashMap::new()),
            timeout: None,
            max_size: None,
        }
    }

    pub fn with_max_size(self, max_size: usize) -> Self {
        Cache {
            cache: self.cache,
            max_size: Some(RwLock::new(MaxSize::new(max_size))),
            timeout: self.timeout
        }
    }

    pub fn with_timeout(self, timeout: u32) -> Self {
        unimplemented!();
    }

    pub async fn lookup(&self, key: &K) -> Option<Arc<V>> {
        self.cache.read().await.get(key).cloned()
    }

    // Save key and value in cache
    pub async fn store(&self, key: &K, value: V) {
        self.cache.write().await.insert(key.clone(), Arc::new(value.clone()));

        // If the max_size is set
        if let Some(max_size) = &self.max_size {
            // if we have filled the buffer
            if let Some(to_remove) = max_size.write().await.put(key.clone()) {
                // remove the least recent
                self.cache.write().await.remove(&to_remove);
            }
        }
    }
}
