use std::{
    borrow::Borrow,
    cell::UnsafeCell,
    collections::{btree_map::Entry, BTreeMap},
};

#[derive(Debug)]
pub struct AppendMap<K, V> {
    inner: UnsafeCell<BTreeMap<K, Box<V>>>,
}

impl<K, V> Default for AppendMap<K, V> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<K, V> Clone for AppendMap<K, V>
where
    K: Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        let map = unsafe { &*self.inner.get() };
        Self {
            inner: UnsafeCell::new(map.clone()),
        }
    }
}

impl<K, V> AppendMap<K, V> {
    pub fn insert(&self, key: K, value: V) -> &V
    where
        K: Ord,
    {
        let map = unsafe { &mut *self.inner.get() };

        match map.entry(key) {
            Entry::Vacant(vacant) => vacant.insert(Box::new(value)),
            Entry::Occupied(_) => panic!("key is already present"),
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        let map = unsafe { &*self.inner.get() };
        map.get(key).map(Box::as_ref)
    }
}

#[cfg(test)]
mod tests {
    use super::AppendMap;

    #[test]
    #[should_panic]
    fn double_insert() {
        let map = AppendMap::default();
        map.insert("a", "b");
        map.insert("a", "c");
    }

    #[test]
    fn multi_borrow() {
        let map = AppendMap::default();
        map.insert("a", "A");
        let a = map.get("a").unwrap();
        map.insert("b", "B");

        let b = map.get("b").unwrap();

        assert_ne!(a, b);
    }
}
