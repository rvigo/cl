use std::collections::BTreeMap;

pub struct SubscriptionSet<Publisher, V> {
    pub subscriptions: BTreeMap<Publisher, Vec<Subscriber<V>>>,
}

#[derive(Debug)]
pub struct Subscriber<V> {
    pub is_active: bool,
    // TODO should it be a Rc<V>?
    pub listener: V,
}

impl<V: 'static> Subscriber<V> {
    pub fn new(listener: V) -> Subscriber<V> {
        Self {
            is_active: true,
            listener,
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
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
    pub fn add(&mut self, key: K, listener: V) {
        let subscription = Subscriber {
            is_active: true,
            listener,
        };

        self.subscriptions
            .entry(key)
            .or_default()
            .push(subscription);
    }

    pub fn remove(&mut self, key: &K) {
        // not sure if this `ptr::eq` is ok, but it works
        self.subscriptions.remove(key);
    }

    pub fn get(&self, key: &K) -> Option<&Vec<Subscriber<V>>> {
        self.subscriptions.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut Vec<Subscriber<V>>> {
        self.subscriptions.get_mut(key)
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
                        .map(|listener| Subscriber {
                            is_active: true,
                            listener,
                        })
                        .collect();
                    (key, subscriptions)
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screen::layer::{Layer, MainScreenLayer};

    #[test]
    fn test_add_and_get() {
        let mut set = SubscriptionSet::<&str, i32>::default();
        set.add("topic1", 42);
        let subs = set.get(&"topic1").unwrap();
        assert_eq!(subs.len(), 1);
        assert_eq!(subs[0].listener, 42);
        assert!(subs[0].is_active);
    }

    #[test]
    fn test_deactivate() {
        let mut sub = Subscriber::new(10);
        assert!(sub.is_active);
        sub.deactivate();
        assert!(!sub.is_active);
    }

    #[test]
    fn test_extend() {
        let mut set1 = SubscriptionSet::<&str, i32>::default();
        set1.add("topic1", 1);

        let mut set2 = SubscriptionSet::<&str, i32>::default();
        set2.add("topic1", 2);
        set2.add("topic2", 3);

        set1.extend(set2);

        let subs1 = set1.get(&"topic1").unwrap();
        assert_eq!(subs1.len(), 2);
        let subs2 = set1.get(&"topic2").unwrap();
        assert_eq!(subs2.len(), 1);
    }

    #[test]
    fn test_from_btreemap() {
        let mut map = BTreeMap::new();
        map.insert("topic1", vec![1, 2]);
        let set: SubscriptionSet<_, _> = map.into();
        let subs = set.get(&"topic1").unwrap();
        assert_eq!(subs.len(), 2);
        assert!(subs.iter().all(|s| s.is_active));
    }

    #[test]
    fn test_remove_removes_all_components_for_key() {
        let mut set = SubscriptionSet::<&str, i32>::default();
        set.add("topic1", 1);
        set.add("topic1", 2);
        set.add("topic2", 3);
        // Ensure topic1 exists and has two subscribers
        assert_eq!(set.get(&"topic1").map(|v| v.len()), Some(2));
        // Remove all components for topic1
        set.remove(&"topic1");
        // topic1 should no longer exist
        assert!(set.get(&"topic1").is_none());
        // topic2 should still exist
        assert_eq!(set.get(&"topic2").map(|v| v.len()), Some(1));
    }
}
