use crate::{command::Command, hashmap, CommandMap, CommandVec};
use log::debug;

/// Caches a `Command` list using the namespace as a key for faster search
#[derive(Default)]
pub struct Cache {
    cache: CommandMap,
}

impl Cache {
    pub fn new(command_list: CommandVec) -> Cache {
        let mut namespace_map: CommandMap = hashmap!();

        command_list.into_iter().for_each(|c| {
            namespace_map
                .entry(c.namespace.to_owned())
                .or_default()
                .push(c);
        });

        let mut cache_info = Cache {
            cache: namespace_map,
        };

        cache_info.sort_cached_values();
        cache_info
    }

    #[inline]
    pub fn get_entry(&mut self, namespace: &str) -> CommandVec {
        self.cache
            .get(namespace)
            .unwrap_or(&Vec::default())
            .to_owned()
    }

    #[inline]
    pub fn update_entry(&mut self, new_command_item: &Command, old_command_item: &Command) {
        let new_namespace = &new_command_item.namespace;

        debug!("updating {new_namespace} cache entries with the new command");
        let commands = self.cache.entry(new_namespace.to_owned()).or_default();
        commands.push(new_command_item.to_owned());

        self.remove_entry(old_command_item);

        self.sort_cached_values()
    }

    #[inline]
    pub fn remove_entry(&mut self, command_item: &Command) {
        let namespace = &command_item.namespace;

        if let Some(commands) = self.cache.get_mut(namespace) {
            if let Some(index) = commands.iter().position(|c| c.eq(command_item)) {
                debug!("removing old cache entry from {namespace}");
                commands.remove(index);
            }
        }
    }

    #[inline]
    pub fn insert_entry(&mut self, command_item: Command) {
        let namespace = &command_item.namespace;
        if let Some(commands) = self.cache.get_mut(namespace) {
            commands.push(command_item)
        } else {
            self.cache.insert(namespace.to_string(), vec![command_item]);
        }

        self.sort_cached_values()
    }

    #[inline]
    fn sort_cached_values(&mut self) {
        for commands in self.cache.values_mut() {
            commands.sort_by_key(|c| c.alias.to_lowercase());
        }
    }
}
