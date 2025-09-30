use dashmap::DashMap;
use std::hash::Hash;
use std::sync::Arc;
use tracing::debug;

/// Generic cache using DashMap for frequently-read state
pub struct Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    store: Arc<DashMap<K, V>>,
    name: String,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Create a new cache with a given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            store: Arc::new(DashMap::new()),
            name: name.into(),
        }
    }

    /// Insert or update a value in the cache
    pub fn set(&self, key: K, value: V) {
        self.store.insert(key, value);
    }

    /// Get a value from the cache
    pub fn get(&self, key: &K) -> Option<V> {
        self.store.get(key).map(|entry| entry.value().clone())
    }

    /// Remove a value from the cache
    pub fn remove(&self, key: &K) -> Option<V> {
        self.store.remove(key).map(|(_, v)| v)
    }

    /// Check if a key exists in the cache
    pub fn contains(&self, key: &K) -> bool {
        self.store.contains_key(key)
    }

    /// Clear all entries from the cache
    pub fn clear(&self) {
        self.store.clear();
        debug!("Cleared cache: {}", self.name);
    }

    /// Get the number of entries in the cache
    pub fn len(&self) -> usize {
        self.store.len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    /// Get all keys in the cache
    pub fn keys(&self) -> Vec<K> {
        self.store.iter().map(|entry| entry.key().clone()).collect()
    }

    /// Get all values in the cache
    pub fn values(&self) -> Vec<V> {
        self.store.iter().map(|entry| entry.value().clone()).collect()
    }

    /// Iterate over all entries and apply a function
    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(&K, &V),
    {
        self.store.iter().for_each(|entry| {
            f(entry.key(), entry.value());
        });
    }

    /// Get cache name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<K, V> Clone for Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        Self {
            store: Arc::clone(&self.store),
            name: self.name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic_operations() {
        let cache: Cache<String, i32> = Cache::new("test_cache");

        // Test insert and get
        cache.set("key1".to_string(), 42);
        assert_eq!(cache.get(&"key1".to_string()), Some(42));

        // Test contains
        assert!(cache.contains(&"key1".to_string()));
        assert!(!cache.contains(&"key2".to_string()));

        // Test len
        assert_eq!(cache.len(), 1);
        assert!(!cache.is_empty());

        // Test remove
        let removed = cache.remove(&"key1".to_string());
        assert_eq!(removed, Some(42));
        assert_eq!(cache.get(&"key1".to_string()), None);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_cache_multiple_entries() {
        let cache: Cache<u64, String> = Cache::new("multi_cache");

        // Insert multiple entries
        for i in 0..10 {
            cache.set(i, format!("value_{}", i));
        }

        assert_eq!(cache.len(), 10);

        // Get all keys
        let keys = cache.keys();
        assert_eq!(keys.len(), 10);

        // Get all values
        let values = cache.values();
        assert_eq!(values.len(), 10);

        // Clear cache
        cache.clear();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_cache_update() {
        let cache: Cache<String, i32> = Cache::new("update_cache");

        cache.set("key".to_string(), 1);
        assert_eq!(cache.get(&"key".to_string()), Some(1));

        // Update value
        cache.set("key".to_string(), 2);
        assert_eq!(cache.get(&"key".to_string()), Some(2));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_cache_for_each() {
        let cache: Cache<u64, u64> = Cache::new("iterate_cache");

        for i in 0..5 {
            cache.set(i, i * 2);
        }

        let mut sum = 0;
        cache.for_each(|_, v| {
            sum += v;
        });

        assert_eq!(sum, 0 + 2 + 4 + 6 + 8);
    }

    #[test]
    fn test_cache_clone() {
        let cache1: Cache<String, i32> = Cache::new("original");
        cache1.set("key".to_string(), 42);

        let cache2 = cache1.clone();
        assert_eq!(cache2.get(&"key".to_string()), Some(42));

        // Modifications in one should be visible in the other (same Arc)
        cache2.set("key2".to_string(), 100);
        assert_eq!(cache1.get(&"key2".to_string()), Some(100));
    }
}
