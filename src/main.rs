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
use gui::entities::app::AppContext;
use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

fn main() -> Result<()> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {f}:{L} — {m}{n}",
        )))
        .build("log/output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Debug),
        )?;

    log4rs::init_config(config)?;
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
