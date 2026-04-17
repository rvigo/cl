use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

pub struct SubscriptionSet<Publisher, V> {
    pub subscriptions: HashMap<Publisher, Vec<Subscriber<V>>>,
}

#[derive(Debug)]
pub struct Subscriber<V> {
    pub listener: V,
}

impl<K, V: 'static> Default for SubscriptionSet<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self {
            subscriptions: HashMap::new(),
        }
    }
}

impl<K, V: 'static> SubscriptionSet<K, V>
where
    K: Eq + Hash,
{
    /// Removes individual subscribers for `key` that match the predicate.
    ///
    /// The predicate receives each subscriber's value; return `true` to
    /// **remove** it.  If the subscriber list for `key` becomes empty the
    /// key is also removed, mirroring the behaviour of [`remove`].
    ///
    /// This is used by [`LayerStack`] when popping a layer: instead of
    /// removing *all* subscribers for a `TypeId` (which would also destroy
    /// subscriptions registered by lower layers), only the subscribers
    /// belonging to the departing layer are removed.
    pub fn remove_matching<F>(&mut self, key: &K, mut predicate: F)
    where
        F: FnMut(&V) -> bool,
    {
        if let Some(subs) = self.subscriptions.get_mut(key) {
            subs.retain(|sub| !predicate(&sub.listener));
            if subs.is_empty() {
                self.subscriptions.remove(key);
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&[Subscriber<V>]> {
        self.subscriptions.get(key).map(|v| v.as_slice())
    }

    pub fn extend(&mut self, other: Self) {
        for (key, subscriptions) in other.subscriptions {
            self.subscriptions
                .entry(key)
                .or_default()
                .extend(subscriptions);
        }
    }
}

impl<K: Eq + Hash, T> From<BTreeMap<K, Vec<T>>> for SubscriptionSet<K, T> {
    fn from(value: BTreeMap<K, Vec<T>>) -> Self {
        Self {
            subscriptions: value
                .into_iter()
                .map(|(key, value)| {
                    let subscriptions = value
                        .into_iter()
                        .map(|listener| Subscriber { listener })
                        .collect();
                    (key, subscriptions)
                })
                .collect(),
        }
    }
}

impl<K: Eq + Hash + Clone, T: Clone> From<&BTreeMap<K, Vec<T>>> for SubscriptionSet<K, T> {
    fn from(value: &BTreeMap<K, Vec<T>>) -> Self {
        Self {
            subscriptions: value
                .iter()
                .map(|(key, value)| {
                    let subscriptions = value
                        .iter()
                        .cloned()
                        .map(|listener| Subscriber { listener })
                        .collect();
                    (key.clone(), subscriptions)
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_set<K: Eq + Hash, V>(
        pairs: impl IntoIterator<Item = (K, Vec<V>)>,
    ) -> SubscriptionSet<K, V> {
        let map: HashMap<K, Vec<V>> = pairs.into_iter().collect();
        SubscriptionSet {
            subscriptions: map
                .into_iter()
                .map(|(key, values)| {
                    let subs = values
                        .into_iter()
                        .map(|v| Subscriber { listener: v })
                        .collect();
                    (key, subs)
                })
                .collect(),
        }
    }

    #[test]
    fn test_get() {
        let set = make_set([("topic1", vec![42_i32])]);
        let subs = set.get(&"topic1").unwrap();
        assert_eq!(subs.len(), 1);
        assert_eq!(subs[0].listener, 42);
    }

    #[test]
    fn test_extend() {
        let mut set1 = make_set([("topic1", vec![1_i32])]);
        let set2 = make_set([("topic1", vec![2_i32]), ("topic2", vec![3_i32])]);

        set1.extend(set2);

        assert_eq!(set1.get(&"topic1").unwrap().len(), 2);
        assert_eq!(set1.get(&"topic2").unwrap().len(), 1);
    }

    #[test]
    fn test_from_btreemap() {
        let mut map = BTreeMap::new();
        map.insert("topic1", vec![1_i32, 2]);
        let set: SubscriptionSet<_, _> = map.into();
        assert_eq!(set.get(&"topic1").unwrap().len(), 2);
    }

    #[test]
    fn remove_matching_all_removes_entire_key() {
        let mut set = make_set([("topic1", vec![1_i32, 2]), ("topic2", vec![3_i32])]);
        assert_eq!(set.get(&"topic1").map(|v| v.len()), Some(2));
        // removing all entries is equivalent to the old remove(key)
        set.remove_matching(&"topic1", |_| true);
        assert!(set.get(&"topic1").is_none());
        assert_eq!(set.get(&"topic2").map(|v| v.len()), Some(1));
    }

    #[test]
    fn remove_matching_only_removes_matching_subscribers() {
        let mut set = make_set([("topic1", vec![1_i32, 2, 3])]);
        // Remove only the subscriber with value == 2
        set.remove_matching(&"topic1", |v| *v == 2);
        let subs = set.get(&"topic1").unwrap();
        assert_eq!(subs.len(), 2);
        assert!(subs.iter().all(|s| s.listener != 2));
    }

    #[test]
    fn remove_matching_drops_key_when_list_becomes_empty() {
        let mut set = make_set([("topic1", vec![42_i32])]);
        set.remove_matching(&"topic1", |_| true);
        assert!(set.get(&"topic1").is_none());
    }

    #[test]
    fn remove_matching_leaves_other_keys_untouched() {
        let mut set = make_set([("a", vec![1_i32]), ("b", vec![2_i32])]);
        set.remove_matching(&"a", |_| true);
        assert!(set.get(&"a").is_none());
        assert_eq!(set.get(&"b").unwrap().len(), 1);
    }
}
