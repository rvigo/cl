mod command_item;
mod commands;
mod configs;
mod file_service;
mod gui;
mod utils;

use commands::Commands;
use gui::contexts::app::AppContext;
use std::error::Error;

//TODO remover trait Debug dos structs
//TODO ajustar log file
fn main() -> Result<(), Box<dyn Error>> {
    configs::log_config::init()?;

    let commands = Commands::init();

    let mut app_context = AppContext::create(commands)?;

    app_context.render()?;

    app_context.clear()?;
    app_context.callback_command()?;

    Ok(())
}
