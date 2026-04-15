use cl_core::Command;

/// A trait to give a type an optimized string representation to fuzzy searches
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
        use std::fmt::Write;
        let mut buf = String::with_capacity(
            self.alias.len() + self.command.len() + self.namespace.len() + 16, // separators + extras
        );
        let _ = write!(buf, "{} {} {}", self.alias, self.command, self.namespace);
        let tags = self.tags_as_string();
        if !tags.is_empty() {
            let _ = write!(buf, " {tags}");
        }
        let desc = self.description();
        if !desc.is_empty() {
            let _ = write!(buf, " {desc}");
        }
        buf
    }
}
