use std::collections::BTreeMap;

pub struct SubscriptionSet<Publisher, V> {
    pub subscriptions: BTreeMap<Publisher, Vec<Subscriber<V>>>,
}

#[derive(Debug)]
pub struct Subscriber<V> {
    pub listener: V,
}

impl<K, V: 'static> Default for SubscriptionSet<K, V>
where
    K: Ord,
{
    fn default() -> Self {
        Self {
            subscriptions: BTreeMap::new(),
        }
    }
}

impl<K, V: 'static> SubscriptionSet<K, V>
where
    K: Ord,
{
    pub fn remove(&mut self, key: &K) {
        self.subscriptions.remove(key);
    }

    pub fn get(&self, key: &K) -> Option<&Vec<Subscriber<V>>> {
        self.subscriptions.get(key)
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

impl<K: Ord, T> From<BTreeMap<K, Vec<T>>> for SubscriptionSet<K, T> {
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

impl<K: Ord + Clone, T: Clone> From<&BTreeMap<K, Vec<T>>> for SubscriptionSet<K, T> {
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

    fn make_set<K: Ord, V>(pairs: impl IntoIterator<Item = (K, Vec<V>)>) -> SubscriptionSet<K, V> {
        pairs.into_iter().collect::<BTreeMap<_, _>>().into()
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
    fn test_remove_removes_all_components_for_key() {
        let mut set = make_set([("topic1", vec![1_i32, 2]), ("topic2", vec![3_i32])]);
        assert_eq!(set.get(&"topic1").map(|v| v.len()), Some(2));
        set.remove(&"topic1");
        assert!(set.get(&"topic1").is_none());
        assert_eq!(set.get(&"topic2").map(|v| v.len()), Some(1));
    }
}
