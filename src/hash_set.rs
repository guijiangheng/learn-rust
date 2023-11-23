use core::hash::Hash;
use std::{
    borrow::Borrow,
    cmp::Ordering,
    ops::{BitAnd, BitOr, BitXor, Sub},
};

use super::hash_map::HashMap;

#[derive(Clone)]
pub struct HashSet<T: Hash + Eq> {
    hash_map: HashMap<T, ()>,
}

impl<T: Hash + Eq> HashSet<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            hash_map: HashMap::with_capacity(capacity),
        }
    }

    pub fn len(&self) -> usize {
        self.hash_map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hash_map.is_empty()
    }

    pub fn contains<Q>(&self, key: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        self.hash_map.iter().any(|x| x.0.borrow() == key)
    }

    pub fn insert(&mut self, value: T) -> bool {
        self.hash_map.insert(value, ()).is_some()
    }

    pub fn remove<Q>(&mut self, key: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.hash_map.remove(key).is_some()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.hash_map.iter().map(|x| &x.0)
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        if self.len() > other.len() {
            return false;
        }

        self.iter().all(|x| other.contains(x))
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        other.is_subset(self)
    }

    pub fn is_disjoint(self, other: &Self) -> bool {
        self.iter().all(|x| !other.contains(x))
    }

    pub fn intersection<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &T> {
        self.iter()
            .chain(other.iter().filter(|x| !self.contains(x)))
    }

    pub fn difference<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &T> {
        self.iter().filter(|x| other.contains(x))
    }

    pub fn union<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &T> {
        self.iter().chain(other.difference(self))
    }

    pub fn symmetric_difference<'a>(&'a self, other: &'a Self) -> impl Iterator<Item = &T> {
        self.difference(other).chain(other.difference(self))
    }
}

impl<T: Hash + Eq> Default for HashSet<T> {
    fn default() -> Self {
        Self {
            hash_map: HashMap::new(),
        }
    }
}

impl<T: Hash + Eq> FromIterator<T> for HashSet<T> {
    fn from_iter<IntoItr: IntoIterator<Item = T>>(iter: IntoItr) -> Self {
        let mut set = HashSet::new();

        for x in iter {
            set.insert(x);
        }

        set
    }
}

impl<'a, 'b, T> BitOr<&'b HashSet<T>> for &'a HashSet<T>
where
    T: Hash + Eq + Clone,
{
    type Output = HashSet<T>;

    fn bitor(self, other: &'b HashSet<T>) -> Self::Output {
        self.union(other).cloned().collect()
    }
}

impl<'a, 'b, T> BitAnd<&'b HashSet<T>> for &'a HashSet<T>
where
    T: Hash + Eq + Clone,
{
    type Output = HashSet<T>;

    fn bitand(self, other: &'b HashSet<T>) -> Self::Output {
        self.intersection(other).cloned().collect()
    }
}

impl<'a, 'b, T> Sub<&'b HashSet<T>> for &'a HashSet<T>
where
    T: Hash + Eq + Clone,
{
    type Output = HashSet<T>;

    fn sub(self, other: &'b HashSet<T>) -> Self::Output {
        self.difference(other).cloned().collect()
    }
}

impl<'a, 'b, T: Hash + Eq + Clone> BitXor<&'b HashSet<T>> for &'a HashSet<T> {
    type Output = HashSet<T>;

    fn bitxor(self, other: &'b HashSet<T>) -> Self::Output {
        self.symmetric_difference(other).cloned().collect()
    }
}

impl<T: Hash + Eq> PartialEq for HashSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().all(|x| other.contains(x))
    }
}

impl<T: Hash + Eq> PartialOrd for HashSet<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.is_subset(other), self.len() == other.len()) {
            (true, true) => Some(Ordering::Equal),
            (true, false) => Some(Ordering::Less),
            (false, _) => Some(Ordering::Greater).filter(|_| self.is_superset(other)),
        }
    }
}

impl<T: Hash + Eq + std::fmt::Debug> std::fmt::Debug for HashSet<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.iter();
        while let Some(k) = iter.next() {
            writeln!(fmt, "{:?}", k)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_set() -> HashSet<i32> {
        let mut set = HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set
    }

    #[test]
    fn test_insert() {
        let set = create_set();
        assert_eq!(set.len(), 3);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
    }

    #[test]
    fn test_remove() {
        let mut set = create_set();
        set.remove(&1);
        assert_eq!(set.len(), 2);
        assert!(!set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
    }

    #[test]
    fn test_subset_superset() {
        let (mut a, b) = (create_set(), create_set());
        a.insert(4);
        assert!(b.is_subset(&a));
        assert!(a.is_superset(&b));
    }

    #[test]
    fn test_eq() {
        let (mut a, b) = (create_set(), create_set());
        assert!(a == b);
        a.insert(4);
        assert!(a != b);
    }

    #[test]
    fn test_order() {
        let (mut a, mut b) = (create_set(), create_set());
        a.insert(4);
        assert!(a > b);
        b.insert(5);
        assert!(!(a > b));
    }

    #[test]
    fn test_bit_op() {
        let (mut a, b) = (create_set(), create_set());
        a.insert(4);
        let c = &a & &b;
        assert!(c == a);
    }
}
