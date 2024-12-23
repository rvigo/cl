use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CommandArgType {
    NamedParameter,
    NonNamedParameter,
}

#[derive(Debug, Default)]
pub struct CommandArgs {
    /// Hashmap with two groups (named parameters / non named parameters)
    command_args: HashMap<CommandArgType, Vec<CommandArg>>,
    /// Reference list with collected named parameters keys
    named_parameters: Vec<String>,
}

impl CommandArgs {
    pub fn init(named_parameters: Vec<String>) -> CommandArgs {
        Self {
            command_args: HashMap::default(),
            named_parameters,
        }
    }

    pub fn push(&mut self, command_arg: CommandArg) {
        if self.named_parameters.contains(&command_arg.arg) {
            self.command_args
                .entry(CommandArgType::NamedParameter)
                .or_default()
                .push(command_arg);
        } else {
            self.command_args
                .entry(CommandArgType::NonNamedParameter)
                .or_default()
                .push(command_arg);
        }
    }

    pub fn named_parameters(&self) -> Option<&Vec<CommandArg>> {
        self.command_args.get(&CommandArgType::NamedParameter)
    }

    pub fn options(&self) -> Option<&Vec<CommandArg>> {
        self.command_args.get(&CommandArgType::NonNamedParameter)
    }

    pub fn has_named_parameters(&self) -> bool {
        !self.named_parameters.is_empty()
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CommandArg {
    arg: String,
    prefix: Option<String>,
    value: Option<String>,
}

impl CommandArg {
    pub fn new(arg: String, prefix: Option<String>, value: Option<String>) -> CommandArg {
        Self { arg, prefix, value }
    }

    pub fn as_key_value_pair(&self) -> (String, String) {
        let key = self.arg.to_string();
        let value = self.value.to_owned().unwrap_or_default();
        (key, value)
    }

    pub fn is_empty(&self) -> bool {
        self.arg.is_empty() && (self.prefix.is_none() || self.prefix.as_ref().unwrap().is_empty())
    }
}

impl ToString for CommandArg {
    fn to_string(&self) -> String {
        let prefix = self.prefix.to_owned().unwrap_or_default();

        match &self.value {
            Some(value) => format!("{}{}={}", prefix, self.arg, value),
            None => format!("{}{}", prefix, self.arg),
        }
    }
}
