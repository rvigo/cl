mod command_item;
mod commands;
mod config;
mod file_service;
mod gui;
mod utils;

use commands::Commands;
use gui::contexts::app::AppContext;
use std::error::Error;

//TODO remover trait Debug dos structs
fn main() -> Result<(), Box<dyn Error>> {
    log4rs::init_file("./src/config/log4rs.yaml", Default::default()).unwrap();

    let commands = Commands::init();

    let mut app_context = AppContext::create(commands)?;

    app_context.render()?;

    app_context.clear()?;
    app_context.callback_command()?;

    Ok(())
}
