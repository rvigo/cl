use std::{
    io::{self, Read},
    str::FromStr,
};

/// HEAVILY inspired by clap-stdin crate
#[derive(Clone, Debug)]
pub struct MaybeStdin<T> {
    pub value: T,
}

impl<T> FromStr for MaybeStdin<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let source = Source::from_str(s)?;

        T::from_str(source.get_value()?.trim())
            .map_err(|e| Error::Parse(format!("{e}")))
            .map(|value| Self { value })
    }
}

enum Source {
    Stdin,
    Arg(String),
}

impl Source {
    fn get_value(self) -> Result<String, Error> {
        match self {
            Source::Stdin => {
                let mut buffer = String::new();
                io::stdin().lock().read_to_string(&mut buffer)?;
                Ok(buffer)
            }
            Source::Arg(arg) => Ok(arg),
        }
    }
}

impl FromStr for Source {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Source::Stdin)
        } else {
            Ok(Source::Arg(s.to_string()))
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to parse value: {0}")]
    Parse(String),
    #[error(transparent)]
    StdIn(#[from] io::Error),
}
