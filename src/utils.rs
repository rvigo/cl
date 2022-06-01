use toml;

pub fn to_toml<T>(value: &T) -> String
where
    T: for<'de> serde::Deserialize<'de> + serde::Serialize,
{
    toml::to_string(&value).expect("Unable to convert to toml.")
}

pub fn from_toml<T>(text: &str) -> T
where
    T: for<'de> serde::Deserialize<'de> + serde::Serialize,
{
    toml::from_str(text).expect("Unable to parse file.")
}