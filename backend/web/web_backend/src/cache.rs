use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
use std::time::Duration;
use std::mem::replace;

/// Put max size on cache.
/// Put timeout on cache entries.

struct MaxSize {
    max_size: usize,
    filo: Vec<String>,
    index: usize
}

// Emluate queue where we can save the Keys from cache
// As long as the queue is not filled, we save all the data

// When the queue hits maximum, we will start replacing the
// oldest entries.
impl MaxSize {
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
    fn put(&mut self, data: String) -> Option<String> {
        // if the cache has not been filled yet
        if self.filo.len() < self.max_size {
            self.filo.push(data);
            self.index += 1;
            None
        } else {
            // If we are at the last index, set index to start position
            if self.has_reached_max_size() {
                self.index = 0;
            }

            let ret = replace(&mut self.filo[self.index], data);
            self.index += 1;
            Some(ret)
        }
    }

    fn has_reached_max_size(&self) -> bool {
        self.filo.len() == self.max_size
    }

}

pub struct CacheBuilder {
    max_size: Option<usize>,
    timeout: Option<Duration>,
}

impl CacheBuilder {
    
    pub fn new() -> Self {
        CacheBuilder {
            timeout: None,
            max_size: None,
        }
    }

    pub fn with_max_size(self, max_size: usize) -> Self {
        CacheBuilder {
            max_size: Some(max_size),
            timeout: self.timeout
        }
    }

    pub fn with_timeout(self, __timeout: Duration) -> Self {
        unimplemented!();
    }

    pub fn build<V>(self) -> Cache<V> {
        Cache {
            cache: RwLock::new(HashMap::new()),
            __timeout: self.timeout,
            max_size: self.max_size.map(|size| RwLock::new(MaxSize::new(size))),
        }
    }
}

impl Default for CacheBuilder {
    fn default() -> Self {
        CacheBuilder::new()
    }
}

pub struct Cache<V> {
    cache: RwLock<HashMap<String, Arc<V>>>,
    max_size: Option<RwLock<MaxSize>>,
    __timeout: Option<Duration>,
}

impl<V: Clone> Cache<V> {

    pub fn builder() -> CacheBuilder {
        CacheBuilder::default()
    }

    pub async fn lookup(&self, key: &str) -> Option<Arc<V>> {
        self.cache.read().await.get(key).cloned()
    }

    // Save key and value in cache
    pub async fn store(&self, key: &str, value: V) -> Arc<V> {
        let value = Arc::new(value);
        self.cache.write().await.insert(String::from(key), value.clone());

        // If the max_size is set
        if let Some(max_size) = &self.max_size {
            // if we have filled the buffer
            if let Some(to_remove) = max_size.write().await.put(String::from(key)) {
                // remove the least recent
                self.cache.write().await.remove(&to_remove);
            }
        }
        value.clone()
    }
}

// Write tests
// Benchmark