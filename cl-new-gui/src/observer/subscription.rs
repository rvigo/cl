use std::any::Any;
use std::collections::BTreeMap;

pub struct SubscriptionSet<Publisher, V> {
    pub subscriptions: BTreeMap<Publisher, Vec<Subscription<V>>>,
}

pub struct Subscription<V> {
    pub is_stateful: bool,
    pub listener: V,
}

impl<V: 'static> Subscription<V> {
    pub fn new(listener: V, is_stateful: bool) -> Subscription<V> {
        Self {
            is_stateful,
            listener,
        }
    }

    pub fn as_any(&self) -> Subscription<Box<dyn Any + '_>> {
        let listener: Box<dyn Any> = Box::new(&self.listener);

        Subscription {
            is_stateful: self.is_stateful,
            listener,
        }
    }
}

impl<K, V: 'static> SubscriptionSet<K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        Self {
            subscriptions: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, key: K, listener: V) {
        let subscription = Subscription {
            is_stateful: true,
            listener,
        };

        self.subscriptions
            .entry(key)
            .or_default()
            .push(subscription);
    }

    pub fn remove(&mut self, key: K) {
        self.subscriptions.remove(&key);
    }

    pub fn get(&self, key: &K) -> Option<&Vec<Subscription<V>>> {
        self.subscriptions.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut Vec<Subscription<V>>> {
        self.subscriptions.get_mut(key)
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
                        .map(|listener| Subscription {
                            is_stateful: true,
                            listener,
                        })
                        .collect();
                    (key, subscriptions)
                })
                .collect(),
        }
    }
}
