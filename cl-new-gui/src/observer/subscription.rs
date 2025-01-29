use std::collections::BTreeMap;
use std::ptr;

pub struct SubscriptionSet<Publisher, V> {
    pub subscriptions: BTreeMap<Publisher, Vec<Subscriber<V>>>,
}

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
        let subscription = Subscriber {
            is_active: true,
            listener,
        };

        self.subscriptions
            .entry(key)
            .or_default()
            .push(subscription);
    }

    pub fn remove(&mut self, targets: &[V]) {
        // not sure if this `ptr::eq` is ok, but it works
        for subscribers in self.subscriptions.values_mut() {
            subscribers.retain(|subscriber| {
                !targets
                    .iter()
                    .any(|target| ptr::eq(&subscriber.listener, target))
            });
        }

        self.subscriptions
            .retain(|_, subscribers| !subscribers.is_empty());
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
