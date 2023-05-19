use std::collections::HashMap;

struct LRUCache {
    cap: i32,
    data: HashMap<i32, i32>,
    head: Box<Entry>,
    tail: Option<Box<Entry>>,
}

struct Entry {
    key: i32,
    next: Option<Box<Entry>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            cap: capacity,
            data: HashMap::new(),
            head: Box::new(Entry { key: 0, next: None }),
            tail: None,
        }
    }

    fn get(&self, key: i32) -> i32 {
        let v = match self.data.get(&key) {
            None => return -1,
            Some(x) => x.clone(),
        };

        v
    }

    fn put(&self, key: i32, value: i32) {}
}
