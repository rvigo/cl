use std::collections::HashSet;

/// A trait to give a type an optmized string representation to fuzzy searches
///
/// *Does not* override any `to_string` or similar implementation
pub trait Fuzzy {
    /// Returns an optimized string representation
    ///
    /// # Example
    ///
    /// ```
    /// struct MyStruc{
    ///     foo: String,
    ///     bar: Option<u32>
    /// }
    ///
    /// impl Fuzzy for MyStruct{
    ///     fn lookup_string(&self) -> String {
    ///         return format!("{} {:?}", self.foo, self.bar)
    ///     }
    /// }
    /// ```
    fn lookup_string(&self) -> String;

    fn filter_duplicates(&self, input: &str) -> String {
        let words: Vec<&str> = input.split_whitespace().collect();
        let unique_words: HashSet<&str> = words.iter().cloned().collect();
        let filtered: Vec<&str> = unique_words.iter().cloned().collect();

        filtered.join(" ")
    }
}
