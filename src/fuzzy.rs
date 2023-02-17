use std::collections::HashSet;

use log::info;

pub trait Fuzzy {
    fn lookup_string(&self) -> String;

    fn filter_duplicates(&self, input: &str) -> String {
        let words: Vec<&str> = input.split_whitespace().collect();
        let unique_words: HashSet<&str> = words.iter().cloned().collect();
        let filtered: Vec<&str> = unique_words.iter().cloned().collect();
        let filtered_string = filtered.join(" ");
        info!("{filtered_string}");
        filtered_string
    }
}
