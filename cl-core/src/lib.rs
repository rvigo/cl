pub mod command;
pub mod commands;
pub mod config;
pub mod logger;
pub mod preferences;
pub mod resource;

#[macro_export]
macro_rules! hashmap {
        () => {{
            std::collections::HashMap::new()
        }};

        ($($key:expr => $value:expr),* ) => {{
            let mut map = map!();
            $(
            map.insert($key, $value);
            ) +

            map
        }};
    }
