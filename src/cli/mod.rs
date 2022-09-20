pub(super) mod app;
pub(super) mod subcommands;

pub(super) mod utils {
    use anyhow::{bail, Context, Result};
    use std::collections::HashMap;
    use strfmt::strfmt;

    const DEFAULT_NAMED_PARAMS_ERROR_MESSAGE: &str = "This command has named parameters! \
    You should provide them exactly as in the command";
    const INVALID_NAMED_PARAMS_ERROR_MESSAGE: &str = "Invalid named arguments! \
        You should provide them exactly as in the command";
    const INVALID_NAMED_PARAMS_VALUE_ERROR_MESSAGE: &str = "Invalid named arguments values!";

    pub fn prepare_command(
        mut command: String,
        named_args: Vec<String>,
        args: Vec<String>,
    ) -> Result<String> {
        if command.contains('#') {
            let mut mapped_args = HashMap::<String, String>::new();
            for arg in named_args.clone() {
                if arg.starts_with("--") {
                    if arg.contains('=') {
                        //handle --option=value string
                        let mut values = arg.split('=');
                        let key = values.next().unwrap().to_string().replacen("--", "", 1);
                        let value = values.next().unwrap().to_string();
                        mapped_args.insert(key, value);
                        continue;
                    }

                    let key = arg.replacen("--", "", 1);
                    mapped_args.insert(key, String::default());
                } else {
                    let key = mapped_args
                        .iter()
                        .find(|(key, value)| !key.is_empty() && value.is_empty())
                        .unwrap_or((&String::default(), &String::default()))
                        .0
                        .to_owned();
                    mapped_args.insert(key, arg);
                }
            }
            validate_args(&mapped_args, &command)?;
            command = command.replace('#', "");
            command = strfmt(&command, &mapped_args).context(format!(
                "Cannot build the command with these arguments: {}\n\n{}",
                named_args.join(", "),
                DEFAULT_NAMED_PARAMS_ERROR_MESSAGE,
            ))?
        }
        if !args.is_empty() {
            command = format!("{} {}", command, &args.join(" "));
        }

        Ok(command)
    }

    fn validate_args(mapped_args: &HashMap<String, String>, command: &str) -> Result<()> {
        let mut error_message: &str = "";
        if mapped_args.is_empty() {
            error_message = DEFAULT_NAMED_PARAMS_ERROR_MESSAGE;
        } else if mapped_args.iter().any(|(k, _)| k.is_empty()) {
            error_message = INVALID_NAMED_PARAMS_ERROR_MESSAGE;
        } else if mapped_args.iter().any(|(_, v)| v.is_empty()) {
            error_message = INVALID_NAMED_PARAMS_VALUE_ERROR_MESSAGE;
        }
        if !error_message.is_empty() {
            let message: String = if mapped_args.is_empty() {
                format!("Cannot run the command {}\n\n{}", command, error_message)
            } else {
                format!(
                    "Cannot run the command {} with the provided arguments\n\n{}",
                    command, error_message
                )
            };
            bail!(message)
        }

        Ok(())
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn should_prepare_simple_command() {
            let result = prepare_command(String::from("echo hello"), vec![], vec![]);
            assert_eq!(result.is_ok(), true);
            assert_eq!(result.unwrap(), "echo hello");
        }

        #[test]
        fn should_prepare_a_command_with_named_parameters() {
            let result = prepare_command(
                String::from("echo #{name}"),
                vec![String::from("--name"), String::from("unit_test")],
                vec![],
            );
            assert_eq!(result.is_ok(), true);
            assert_eq!(result.unwrap(), "echo unit_test");
        }

        #[test]
        fn should_return_error_when_an_invalid_named_parameter_is_given() {
            let named_args: Vec<String> =
                vec![String::from("--invalid"), String::from("unit_test")];
            let result: Result<String> =
                prepare_command(String::from("echo #{name}"), named_args.clone(), vec![]);
            assert_eq!(result.is_err(), true);
            assert_eq!(
                result.unwrap_err().to_string(),
                format!(
                    "Cannot build the command with these arguments: {}\n\n{}",
                    named_args.join(", "),
                    DEFAULT_NAMED_PARAMS_ERROR_MESSAGE
                ),
            );
        }

        #[test]
        fn should_return_error_when_an_invalid_named_parameter_value_is_given() {
            let named_args: Vec<String> = vec![String::from("--name")];
            let result: Result<String> =
                prepare_command(String::from("echo #{name}"), named_args.clone(), vec![]);
            assert_eq!(result.is_err(), true);
            assert_eq!(
                result.unwrap_err().to_string(),
                format!(
                    "Cannot run the command echo #{{name}} with the provided arguments\n\n{}",
                    INVALID_NAMED_PARAMS_VALUE_ERROR_MESSAGE
                )
            );
        }
    }
}
