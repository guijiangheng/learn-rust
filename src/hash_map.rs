use std::borrow::Borrow;
use std::mem;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

const LOAD_FACTOR: f64 = 0.75;

fn make_index<X>(x: &X, capacity: usize) -> usize
where
    X: Hash + Eq + ?Sized,
{
    let mut hasher = DefaultHasher::new();
    x.hash(&mut hasher);
    (hasher.finish() as usize) % capacity
}

pub struct HashMap<K, V>
where
    K: Hash + Eq,
{
    buckets: Vec<Vec<(K, V)>>,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        Self::with_capacity(4)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut me = Self {
            buckets: Vec::with_capacity(capacity),
        };

        for _ in 0..capacity {
            me.buckets.push(Vec::new());
        }

        me
    }

    pub fn len(&self) -> usize {
        self.buckets.iter().map(|x| x.len()).sum()
    }

    pub fn get<Q>(&self, q: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.buckets[make_index(q, self.buckets.len())]
            .iter()
            .find(|(k, _)| k.borrow() == q)
            .map(|(_, v)| v)
    }

    pub fn remove<Q>(&mut self, q: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let index = make_index(q, self.buckets.len());
        self.buckets[index]
            .iter()
            .position(|(k, _)| k.borrow() == q)
            .map(|k| self.buckets[index].swap_remove(k).1)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.try_resize();
        let index = make_index(&key, self.buckets.len());
        let item = self.buckets[index].iter_mut().find(|(k, _)| *k == key);
        match item {
            Some((_, v)) => Some(mem::replace(v, value)),
            None => {
                self.buckets[index].push((key, value));
                None
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.buckets.iter().flat_map(|x| x).map(|x| x)
    }

    fn try_resize(&mut self) {
        if self.len() as f64 / self.buckets.len() as f64 > LOAD_FACTOR {
            let mut new_map = Self::with_capacity(self.buckets.len() << 1);
            self.buckets
                .iter_mut()
                .flat_map(|v| std::mem::replace(v, vec![]))
                .for_each(|(k, v)| {
                    new_map.insert(k, v);
                });
            *self = new_map;
        }
    }
}

impl<K, V> Default for HashMap<K, V>
where
    K: Hash + Eq,
{
    fn default() -> Self {
        Self::with_capacity(4)
    }
}
pub struct IntoIter<K: Hash + Eq, V>(HashMap<K, V>);

impl<K, V> Iterator for IntoIter<K, V>
where
    K: Hash + Eq,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(v) = self.0.buckets.get_mut(0) {
            if v.is_empty() {
                self.0.buckets.swap_remove(0);
            } else {
                return Some(v.swap_remove(0));
            }
        }

        None
    }
}

impl<K, V> IntoIterator for HashMap<K, V>
where
    K: Hash + Eq,
{
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

pub struct Iter<'a, K, V>
where
    K: Hash + Eq,
{
    hash_map: &'a HashMap<K, V>,
    pos: (usize, usize),
}

impl<'a, K, V> Iterator for Iter<'a, K, V>
where
    K: Hash + Eq,
{
    type Item = &'a (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(v) = self.hash_map.buckets.get(self.pos.0) {
            if self.pos.1 == v.len() {
                self.pos = (self.pos.0 + 1, 0);
            } else {
                let x = &v[self.pos.1];
                self.pos.1 += 1;
                return Some(x);
            }
        }

        None
    }
}

impl<'a, K, V> IntoIterator for &'a HashMap<K, V>
where
    K: Hash + Eq,
{
    type Item = &'a (K, V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            hash_map: self,
            pos: (0, 0),
        }
    }
}

impl<K, V> std::fmt::Debug for HashMap<K, V>
where
    K: Hash + Eq + std::fmt::Debug,
    V: std::fmt::Debug,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.iter();
        while let Some((k, v)) = iter.next() {
            writeln!(fmt, "{:?} -> {:?}", k, v)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert() {
        let mut map: HashMap<String, i32> = HashMap::new();
        map.insert("1".to_owned(), 1);
        map.insert("2".to_owned(), 2);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("1"), Some(&1));
        assert_eq!(map.get("2"), Some(&2));
        assert_eq!(map.get("3"), None);

        map.insert("2".to_owned(), 3);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("2"), Some(&3));
    }

    #[test]
    fn remove() {
        let mut map: HashMap<String, i32> = HashMap::new();
        map.insert("1".to_owned(), 1);
        map.insert("2".to_owned(), 2);
        let x = map.remove("1");
        assert_eq!(map.len(), 1);
        assert_eq!(x, Some(1));
    }

    #[test]
    fn into_iter() {
        let mut map: HashMap<String, i32> = HashMap::new();
        map.insert("1".to_owned(), 1);
        map.insert("2".to_owned(), 2);

        let v: Vec<_> = map.into_iter().collect();
        assert!(v.contains(&("1".to_owned(), 1)));
        assert!(v.contains(&("2".to_owned(), 2)));
    }

    #[test]
    fn into_iter_ref() {
        let mut map: HashMap<String, i32> = HashMap::new();
        map.insert("1".to_owned(), 1);
        map.insert("2".to_owned(), 2);

        let v: Vec<_> = (&map).into_iter().collect();
        assert!(v.iter().any(|x| x.0 == "1" && x.1 == 1));
        assert!(v.iter().any(|x| x.0 == "2" && x.1 == 2));
    }
}
