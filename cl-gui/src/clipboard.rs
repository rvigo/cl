use anyhow::{Context, Result};
use copypasta::{ClipboardContext, ClipboardProvider};

/// Wraps clipboard related functions
pub struct Clipboard {
    clipboard: ClipboardContext,
}

impl Clipboard {
    pub fn new() -> Result<Clipboard> {
        ClipboardContext::new()
            .map(|clipboard| Self { clipboard })
            .map_err(|cause| anyhow::anyhow!(cause))
            .context("cannot initiate clipboard")
    }

    pub fn set_content<T>(&mut self, content: T) -> Result<()>
    where
        T: Into<String>,
    {
        self.clipboard
            .set_contents(content.into())
            .map_err(|e| anyhow::anyhow!(e))
            .context("cannot copy content to clipboard")
    }
}
