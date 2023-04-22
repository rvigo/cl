use anyhow::Result;
use copypasta::{ClipboardContext, ClipboardProvider};
use std::error::Error;
use thiserror::Error;

/// Wraps clipboard related functions
pub struct Clipboard {
    clipboard: ClipboardContext,
}

impl Clipboard {
    pub fn new() -> Result<Clipboard, ClipboardError> {
        match ClipboardContext::new() {
            Ok(clipboard) => Ok(Self { clipboard }),
            Err(cause) => Err(ClipboardError::CannotInitiateClipboard { cause }),
        }
    }

    pub fn set_content<T>(&mut self, content: T) -> Result<(), ClipboardError>
    where
        T: Into<String>,
    {
        self.clipboard
            .set_contents(content.into())
            .map_err(|cause| ClipboardError::CannotCopyContentToClipboard { cause })
    }
}

#[derive(Error, Debug)]
pub enum ClipboardError {
    #[error("Cannot initiate clipboard")]
    CannotInitiateClipboard {
        #[source]
        cause: Box<dyn Error + Send + Sync>,
    },

    #[error("Cannot copy content to clipboard")]
    CannotCopyContentToClipboard {
        #[source]
        cause: Box<dyn Error + Send + Sync>,
    },
}

#[cfg(test)]
pub mod test {
    use super::*;
    use anyhow::anyhow;

    #[test]
    pub fn should_copy_content() -> Result<()> {
        let mut clipboard = Clipboard::new()?;
        let text = "hello world";

        clipboard.set_content(text)?;

        let result = clipboard
            .clipboard
            .get_contents()
            .map_err(|err| anyhow!(err))?;

        assert_eq!(result, text);
        Ok(())
    }
}
