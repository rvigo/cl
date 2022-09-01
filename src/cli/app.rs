use clap::{AppSettings, Arg, ColorChoice, Command};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn build_app() -> Command<'static> {
    Command::new(PKG_NAME)
        .version(VERSION)
        .color(ColorChoice::Auto)
        .setting(AppSettings::DeriveDisplayOrder)
        .dont_collapse_args_in_usage(true)
        .args_conflicts_with_subcommands(true)
        .propagate_version(true)
        .about("Group your commands and aliases in an organized and human readable place.")
        .subcommand(
            Command::new("exec")
                .visible_aliases(&["X", "x"])
                .about("Run your commands via CLI")
                .arg(
                    Arg::new("alias")
                        .value_name("ALIAS")
                        .help("The alias to be executed")
                        .required(true),
                )
                .arg(
                    Arg::new("args")
                        .value_name("ARGS")
                        .help(
                            "The args of the alias command to be executed\n\
                                Flags should be escaped with '\\' and surrounded by quotes\n   \
                                e.g: cl exec <some-alias> '\\--flag'",
                        )
                        .takes_value(true)
                        .multiple_values(true)
                        .requires("alias"),
                )
                .arg(
                    Arg::new("namespace")
                        .short('n')
                        .long("namespace")
                        .help("The namespace in case of duplicated aliases")
                        .value_name("NAMESPACE")
                        .requires("alias"),
                )
                .arg(
                    Arg::new("named")
                        .value_name("NAMED PARAMETERS")
                        .help(
                            "The command named parameters. Should be used after all args\n   \
                                e.g: cl exec <some-alias> -- --named-parameter value",
                        )
                        .last(true)
                        .takes_value(true)
                        .multiple_values(true)
                        .requires("alias"),
                ),
        )
}

#[cfg(test)]
mod test {
    use super::*;
    fn get_matches(argv: &[&str]) -> clap::ArgMatches {
        let app = build_app();
        app.get_matches_from(argv)
    }

    #[test]
    fn should_run_with_no_subcommand() {
        let argv = ["cl"];
        let matches = get_matches(&argv);

        assert_eq!(matches.args_present(), false);
        assert_eq!(matches.subcommand().is_some(), false);
    }

    #[test]
    fn should_run_with_x_subcommand() {
        let argv = ["cl", "exec", "test_alias", "arg1"];
        let matches = get_matches(&argv);

        assert_eq!(matches.subcommand().is_some(), true);
    }

    #[test]
    fn should_run_with_x_subcommand_and_named_parameters() {
        let argv = ["cl", "exec", "test_alias", "--", "--named", "value"];
        let matches = get_matches(&argv);

        assert_eq!(matches.subcommand().is_some(), true);
    }
}
