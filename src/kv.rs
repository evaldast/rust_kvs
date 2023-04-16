use std::collections::HashMap;

/// The `KvStore` stores pairs of string key/value pairs.
///
/// The key/value pairs are stored in a in-memory `HashMap`.
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Initializes new KvStore.
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// Sets the value for a given key.
    ///
    /// If key already exists, previous value will be overwritten.
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Gets the value for a given key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    /// Removes a given ke given key.
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
