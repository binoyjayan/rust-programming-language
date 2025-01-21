use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use std::{mem, ops};

const INITIAL_NBUCKETS: usize = 1;

#[derive(Debug)]
pub struct HashMap<K, V> {
    // each bucket is a Vec of (key, value) pairs
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}

impl<K, V> Default for HashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
            items: 0,
        }
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + PartialEq,
{
    pub fn bucket(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.buckets.len() as u64) as usize
    }

    pub fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_NBUCKETS,
            n => 2 * n,
        };
        let mut new_buckets = Vec::with_capacity(target_size);
        new_buckets.extend((0..target_size).map(|_| Vec::new()));
        // Drain all elements from buckets and insert them into new_buckets
        for (key, value) in self.buckets.drain(..).flatten() {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let bucket = (hasher.finish() % new_buckets.len() as u64) as usize;
            new_buckets[bucket].push((key, value));
        }
        mem::swap(&mut self.buckets, &mut new_buckets);
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // If buckets is empty or if map is 3/4th full
        if self.buckets.is_empty() || self.items > self.buckets.len() * 3 / 4 {
            self.resize();
        }
        let bucket = self.bucket(&key);
        let bucket = &mut self.buckets[bucket];

        self.items += 1;
        for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
            if ekey == &key {
                // return existing value
                return Some(mem::replace(evalue, value));
            }
        }
        bucket.push((key, value));
        None
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let bucket = self.bucket(key);
        self.buckets[bucket]
            .iter()
            .find(|(ref ekey, _)| ekey == key)
            .map(|(_, ref v)| v)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let bucket = self.bucket(key);
        let bucket = &mut self.buckets[bucket];
        let i = bucket.iter().position(|(ref ekey, _)| ekey == key)?;
        // use swap_remove to avoid shifting elements
        let val = bucket.swap_remove(i).1;
        self.items -= 1;
        Some(val)
    }

    pub fn len(&self) -> usize {
        self.items
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<K, V> ops::Index<&K> for HashMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    type Output = V;

    fn index(&self, key: &K) -> &Self::Output {
        self.get(key).expect("Key not found")
    }
}

pub struct HashMapIterator<'a, K, V> {
    map: &'a HashMap<K, V>,
    bucket: usize,
    at: usize,
}

impl<'a, K, V> HashMapIterator<'a, K, V> {
    fn new(map: &'a HashMap<K, V>) -> Self {
        Self {
            map,
            bucket: 0,
            at: 0,
        }
    }
}

impl<'a, K, V> Iterator for HashMapIterator<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        // Loop is required since the bucket list can be sparse
        loop {
            match self.map.buckets.get(self.bucket) {
                Some(bucket) => match bucket.get(self.at) {
                    Some((ref k, ref v)) => {
                        self.at += 1;
                        return Some((k, v));
                    }
                    None => {
                        self.bucket += 1;
                        self.at = 0;
                        continue;
                    }
                },
                None => {
                    return None;
                }
            }
        }
    }
}

impl<'a, K, V> IntoIterator for &'a HashMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = HashMapIterator<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        HashMapIterator::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut map = HashMap::new();
        assert_eq!(map.insert("foo", 42), None);
        assert_eq!(map.insert("foo", 23), Some(42));
    }

    #[test]
    fn test_get() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
        assert_eq!(map.get(&"foo"), Some(&42));
        assert_eq!(map.get(&"bar"), None);
    }
    #[test]
    fn test_contains_key() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
        assert!(map.contains_key(&"foo"));
        assert!(!map.contains_key(&"bar"));
    }

    #[test]
    fn test_remove() {
        let mut map = HashMap::new();
        map.insert("foo", 42);
        assert_eq!(map.get(&"foo"), Some(&42));
        assert_eq!(map.remove(&"bar"), None);
        assert_eq!(map.remove(&"foo"), Some(42));
        assert_eq!(map.remove(&"foo"), None);
        assert_eq!(map.get(&"foo"), None);
    }
    #[test]
    fn test_len() {
        let mut map = HashMap::new();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
        map.insert("foo", 42);
        assert_eq!(map.len(), 1);
        map.insert("bar", 23);
        assert_eq!(map.len(), 2);
        map.remove(&"foo");
        assert_eq!(map.len(), 1);
        assert!(!map.is_empty());
        map.remove(&"bar");
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }
}

#[test]
fn test_iterator() {
    let mut map = HashMap::new();
    map.insert("foo", 42);
    map.insert("bar", 23);
    map.insert("baz", 142);
    map.insert("quox", 7);
    for (&k, &v) in &map {
        match k {
            "foo" => assert_eq!(v, 42),
            "bar" => assert_eq!(v, 23),
            "baz" => assert_eq!(v, 142),
            "quox" => assert_eq!(v, 7),
            _ => unreachable!(),
        }
    }
    assert_eq!(map.into_iter().count(), 4);
}
