

#[cfg(test)]
mod test;

pub struct Cache<K, V> {
    // Max Size of the cache
    pub max_size: usize,
    /// Basic vector of values for testing
    pub cache: Vec<(K, V)>,
}

trait LRUCache<K, V> {
    /// Initialize the cache with `max` items stored
    fn initialize(max: usize) -> Self;
    /// Retrieve the value stored by key, if it exists
    fn get(&mut self, key: K) -> Option<V>;
    /// Store a `value` under `key`
    fn set(&mut self, key: K, value: V);
}

impl<K, V> LRUCache<K, V> for Cache<K, V>
where
    K: std::cmp::PartialEq + Copy,
    V: Copy
{
    /// Initialize the cache with `max` items stored
    fn initialize(max: usize) -> Self {
        // TODO max = 0
        Self {
            max_size: max,
            cache: Vec::with_capacity(max),
        }
    }
    /// Retrieve the value stored by key, if it exists
    fn get(&mut self, key: K) -> Option<V> {
        // let mut cache = self.cache.clone();
        let mut existing_key: Option<(K, V)> = None;
        self.cache(|(k, v)| {
            if k == &key {
                existing_key = Some((*k, *v));
                false
            } else {
                true
            }
        });
        if let Some((k, v)) = existing_key {
            let value = Some(v);
            // Push value to end of vec
            self.cache.push((k, v));
            //self.cache = cache;
            value
        } else {
            None
        }
    }
    /// Store a `value` under `key`
    fn set(&mut self, key: K, value: V) {
        // Check if key already exists in cache
        let mut key_exists = false;
        self.cache.retain(|kv| {
            if kv.0 == key {
                key_exists = true;
                false
            } else {
                true
            }
        });
        // The key doesn't exist, check size
        if !key_exists {
            let size = self.cache.len();
            if size >= self.max_size {
                self.cache.remove(0);
            }
        }
        self.cache.push((key, value));
    }
}