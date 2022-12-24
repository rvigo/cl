mod cli;
mod command;
mod commands;
mod gui;
mod resources;

use ::log::info;
use anyhow::Result;
use clap::Parser;
use cli::{
    app::{App, SubCommand},
    subcommands::{
        config::config_subcommand, exec::exec_subcommand, misc::misc_subcommand,
        share::share_subcommand,
    },
};
use gui::entities::app::AppContext;
use resources::log;

fn main() -> Result<()> {
    log::init()?;

    info!("ola");

    let app = App::parse();
    match app.subcommand {
        Some(SubCommand::Exec(exec)) => exec_subcommand(exec),
        Some(SubCommand::Share(share)) => share_subcommand(share),
        Some(SubCommand::Misc(misc)) => misc_subcommand(misc),
        Some(SubCommand::Config(config)) => config_subcommand(config),
        _ => run_main_app(),
    }
}

fn run_main_app() -> Result<()> {
    let mut app_context = AppContext::create()?;
    app_context.render()?;
    app_context.clear()?;

    app_context.callback_command()
}
