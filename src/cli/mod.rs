pub(super) mod app;

pub(super) mod utils {
    use anyhow::{bail, Context, Result};
    use std::collections::HashMap;
    use strfmt::strfmt;

    pub fn prepare_command(
        mut command: String,
        named_args: Vec<String>,
        args: Vec<String>,
    ) -> Result<String> {
        if command.contains('#') {
            let mut mapped_args = HashMap::<String, String>::new();
            for arg in named_args {
                if arg.starts_with("--") {
                    if arg.contains('=') {
                        //handle --option=value string
                        let mut values = arg.split('=');
                        let key = values.next().unwrap().to_string().replace("--", "");
                        let value = values.next().unwrap().to_string();
                        mapped_args.insert(key, value);
                        continue;
                    }

                    let key = arg.replace("--", "");
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
            validate_args(&mapped_args)?;

            command = command.replace('#', "");
            strfmt(&command, &mapped_args).context("Cannnot map named args!")
        } else {
            if !args.is_empty() {
                command = format!("{} {}", command, &args.join(" "));
            }

            Ok(command)
        }
    }

    fn validate_args(mapped_args: &HashMap<String, String>) -> Result<()> {
        if mapped_args.is_empty() {
            bail!(
                "This command has named parameters! You should provide them exactly as in the command"
            )
        } else if mapped_args.iter().any(|(k, _)| k.is_empty()) {
            bail!("Invalid named arguments! You should provide them exactly as in the command")
        } else if mapped_args.iter().any(|(_, v)| v.is_empty()) {
            bail!("Invalid named arguments values!")
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
            let result = prepare_command(
                String::from("echo #{name}"),
                vec![String::from("--invalid"), String::from("unit_test")],
                vec![],
            );
            assert_eq!(result.is_err(), true);
            assert_eq!(result.unwrap_err().to_string(), "Cannnot map named args!");
        }

        #[test]
        fn should_return_error_when_an_invalid_named_parameter_value_is_given() {
            let result = prepare_command(
                String::from("echo #{name}"),
                vec![String::from("--name")],
                vec![],
            );
            assert_eq!(result.is_err(), true);
            assert_eq!(
                result.unwrap_err().to_string(),
                "Invalid named arguments values!"
            );
        }
    }
}
