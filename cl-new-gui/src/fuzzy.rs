use cl_core::Command;

/// A trait to give a type an optmized string representation to fuzzy searches
///
/// *Does not* override any `to_string` or similar implementation
pub trait Fuzzy {
    /// Returns an optimized string representation
    ///
    /// # Example
    ///
    /// ```ignore
    /// struct MyStruct{
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
}

impl Fuzzy for Command<'_> {
    fn lookup_string(&self) -> String {
        format!(
            "{} {} {} {} {}",
            self.alias,
            self.command,
            self.namespace,
            self.tags_as_string(),
            self.description()
        )
        .trim()
        .to_owned()
    }
}
