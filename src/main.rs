mod cli;
mod command;
mod commands;
mod gui;
mod resources;

use anyhow::Result;
use clap::Parser;
use cli::{
    app::{App, SubCommand},
    subcommands::{
        config::config_subcommand, exec::exec_subcommand, misc::misc_subcommand,
        share::share_subcommand,
    },
};
use gui::entities::tui_application::TuiApplication;
use resources::{config::Config, log};

fn main() -> Result<()> {
    let config = Config::load()?;
    log::init(config.get_log_level(), config.get_app_home_dir())?;

    let app = App::parse();

    match app.subcommand {
        Some(SubCommand::Exec(exec)) => exec_subcommand(exec, config),
        Some(SubCommand::Share(share)) => share_subcommand(share, config),
        Some(SubCommand::Misc(misc)) => misc_subcommand(misc, config),
        Some(SubCommand::Config(sub_command_config)) => {
            config_subcommand(sub_command_config, config)
        }
        _ => run_main_app(config),
    }
}

fn run_main_app(config: Config) -> Result<()> {
    let mut tui = TuiApplication::create(config)?;
    tui.render()
}
