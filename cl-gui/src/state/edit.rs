use cl_core::{Command, CommandBuilder};

#[derive(Default)]
pub struct EditState {
    alias: Option<String>,
    command: Option<String>,
    description: Option<String>,
    tags: Option<Vec<String>>,
    namespace: Option<String>,
}

impl EditState {
    pub fn update_alias(&mut self, alias: Option<String>) {
        self.alias = alias;
    }

    pub fn update_command(&mut self, command: Option<String>) {
        self.command = command;
    }

    pub fn update_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn update_tags(&mut self, tags: Option<Vec<String>>) {
        self.tags = tags;
    }

    pub fn update_namespace(&mut self, namespace: Option<String>) {
        self.namespace = namespace;
    }

    /// Returns a snapshot of the current edit state without consuming fields.
    pub fn get(&self) -> Command<'static> {
        CommandBuilder::default()
            .alias(self.alias.clone().unwrap_or_default())
            .command(self.command.clone().unwrap_or_default())
            .namespace(self.namespace.clone().unwrap_or_default())
            .description(self.description.clone())
            .tags(self.tags.clone())
            .build()
    }

    /// Consumes the edit state fields after a successful operation.
    /// Call this only after confirming the edit/insert operation completed.
    pub fn clear(&mut self) {
        self.alias = None;
        self.command = None;
        self.namespace = None;
        self.description = None;
        self.tags = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_alias() {
        let mut state = EditState::default();

        state.update_alias(Some("myalias".to_string()));

        assert_eq!(state.alias, Some("myalias".to_string()));
    }

    #[test]
    fn test_update_command() {
        let mut state = EditState::default();

        state.update_command(Some("ls -la".to_string()));

        assert_eq!(state.command, Some("ls -la".to_string()));
    }

    #[test]
    fn test_update_description() {
        let mut state = EditState::default();

        state.update_description(Some("desc".to_string()));

        assert_eq!(state.description, Some("desc".to_string()));
    }

    #[test]
    fn test_update_tags() {
        let mut state = EditState::default();

        state.update_tags(Some(vec!["tag1".to_string(), "tag2".to_string()]));

        assert_eq!(
            state.tags,
            Some(vec!["tag1".to_string(), "tag2".to_string()])
        );
    }

    #[test]
    fn test_update_namespace() {
        let mut state = EditState::default();

        state.update_namespace(Some("ns".to_string()));

        assert_eq!(state.namespace, Some("ns".to_string()));
    }

    #[test]
    fn test_get_command() {
        let mut state = EditState::default();
        state.update_alias(Some("a".to_string()));
        state.update_command(Some("c".to_string()));
        state.update_namespace(Some("n".to_string()));
        state.update_description(Some("d".to_string()));
        state.update_tags(Some(vec!["t1".to_string()]));

        let cmd = state.get();

        assert_eq!(cmd.alias, "a");
        assert_eq!(cmd.command, "c");
        assert_eq!(cmd.namespace, "n");
        assert_eq!(cmd.description(), "d".to_string());
        assert_eq!(cmd.tags_as_string(), "t1".to_string());
    }

    #[test]
    fn test_get_does_not_consume_fields() {
        let mut state = EditState::default();
        state.update_alias(Some("a".to_string()));
        state.update_command(Some("c".to_string()));
        state.update_namespace(Some("n".to_string()));
        state.update_description(Some("d".to_string()));
        state.update_tags(Some(vec!["t1".to_string()]));

        let _ = state.get();

        assert_eq!(state.alias, Some("a".to_string()));
        assert_eq!(state.command, Some("c".to_string()));
        assert_eq!(state.namespace, Some("n".to_string()));
        assert_eq!(state.description, Some("d".to_string()));
        assert_eq!(state.tags, Some(vec!["t1".to_string()]));
    }

    #[test]
    fn test_clear_consumes_all_fields() {
        let mut state = EditState::default();
        state.update_alias(Some("a".to_string()));
        state.update_command(Some("c".to_string()));
        state.update_namespace(Some("n".to_string()));
        state.update_description(Some("d".to_string()));
        state.update_tags(Some(vec!["t1".to_string()]));

        state.clear();

        assert!(state.alias.is_none());
        assert!(state.command.is_none());
        assert!(state.namespace.is_none());
        assert!(state.description.is_none());
        assert!(state.tags.is_none());
    }
}
