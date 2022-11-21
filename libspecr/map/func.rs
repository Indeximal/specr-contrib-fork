use crate::libspecr::*;

impl<K: GcCompat + Clone + Hash + Eq, V: GcCompat + Clone> Map<K, V> {
    pub fn get(&self, k: K) -> Option<V> {
        self.0.call_ref_unchecked(|m| m.get(&k).cloned())
    }

    pub fn index_at(&self, k: K) -> V {
        self.get(k).unwrap()
    }

    pub fn remove(&mut self, k: K) -> Option<V> {
        self.0.mutate(|m| {
            m.remove(&k)
        })
    }

    pub fn contains_key(&self, k: K) -> bool {
        self.0.call_ref_unchecked(|m| {
            m.contains_key(&k)
        })
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.0.mutate(|m| {
            m.insert(k, v)
        })
    }

    pub fn try_insert(&mut self, k: K, v: V) -> Result<(), ()> {
        if self.contains_key(k.clone()) {
            return Err(());
        }

        self.insert(k, v);

        Ok(())
    }

    // TODO not yet lazy!
    pub fn keys(self) -> impl Iterator<Item=K> {
        let map = self.0.get();
        let keys: Vec<K> = map.iter().map(|(x, _)| x.clone()).collect();
        keys.into_iter()
    }

    // TODO not yet lazy!
    pub fn values(self) -> impl Iterator<Item=V> {
        let map = self.0.get();
        let values: Vec<V> = map.iter().map(|(_, x)| x.clone()).collect();
        values.into_iter()
    }
}
