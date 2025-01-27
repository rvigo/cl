use anyhow::{anyhow, Result};
use copypasta::{ClipboardContext, ClipboardProvider};

/// Wraps clipboard related functions
pub struct Clipboard {
    clipboard: ClipboardContext,
}

impl Clipboard {
    pub fn new() -> Result<Clipboard> {
        match ClipboardContext::new() {
            Ok(clipboard) => Ok(Self { clipboard }),
            Err(cause) => Err(anyhow!(cause)),
        }
    }

    pub fn set_content<T>(&mut self, content: T) -> Result<()>
    where
        T: Into<String>,
    {
        self.clipboard
            .set_contents(content.into())
            .map_err(|e| anyhow!(e))
    }
}
