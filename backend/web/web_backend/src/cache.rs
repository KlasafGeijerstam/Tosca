use std::collections::HashMap;
use std::mem::replace;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

struct MaxSize {
    max_size: usize,
    filo: Vec<String>,
    index: usize,
}

struct Timeout {
    timeout: Duration,
    id_to_time: HashMap<String, Instant>,
}

impl Timeout {
    fn new(timeout: Duration) -> Self {
        Timeout {
            timeout,
            id_to_time: HashMap::new(),
        }
    }

    fn put(&mut self, key: String) {
        self.id_to_time.insert(key.clone(), Instant::now());
    }

    fn remove(&mut self, key: String) {
        self.id_to_time.remove(&key);
    }

    // If this is invalid, n
    fn is_valid(&mut self, key: String) -> bool {
        if let Some(k) = self.id_to_time.get(&key) {
            if k.elapsed() >= self.timeout {
                self.remove(key);
                return false;
            }
        }
        false
    }
}

// Emluate queue where we can save the Keys from cache
// As long as the queue is not filled, we save all the data

// When the queue hits maximum, we will start replacing the
// oldest entries.
impl MaxSize {
    fn new(max_size: usize) -> Self {
        MaxSize {
            max_size,
            filo: Vec::new(), // first-in-last-out
            index: 0,
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
            timeout: self.timeout,
        }
    }

    pub fn with_timeout(self, timeout: Duration) -> Self {
        CacheBuilder {
            max_size: self.max_size,
            timeout: Some(timeout),
        }
    }

    pub fn build<V>(self) -> Cache<V> {
        Cache {
            cache: RwLock::new(HashMap::new()),
            timeout: self
                .timeout
                .map(|duration| RwLock::new(Timeout::new(duration))),
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
    timeout: Option<RwLock<Timeout>>,
}

impl<V: Clone> Cache<V> {
    pub fn builder() -> CacheBuilder {
        CacheBuilder::default()
    }

    pub async fn lookup(&self, key: &str) -> Option<Arc<V>> {
        // If the timeout is set
        if let Some(timeout) = &self.timeout {
            // If the entry is too old
            if !timeout.write().await.is_valid(String::from(key)) {
                // TODO: if this is invalid, MaxSize will contain one unnecessary
                // entry
                return None;
            }
        }
        self.cache.read().await.get(key).cloned()
    }

    // Save key and value in cache
    pub async fn store(&self, key: &str, value: V) -> Arc<V> {
        let value = Arc::new(value);
        self.cache
            .write()
            .await
            .insert(String::from(key), value.clone());

        // If the timeout is set
        if let Some(timeout) = &self.timeout {
            timeout.write().await.put(String::from(key));
        }

        // If the max_size is set
        if let Some(max_size) = &self.max_size {
            // if we have filled the buffer
            if let Some(to_remove) = max_size.write().await.put(String::from(key)) {
                // remove the least recent
                self.cache.write().await.remove(&to_remove);
                // If the timeout is set
                if let Some(timeout) = &self.timeout {
                    timeout.write().await.remove(String::from(key));
                }
            }
        }
        value.clone()
    }
}

// TODO: Write tests
#[tokio::test]
async fn max_size_test() {
}

// TODO: Setup benchmark