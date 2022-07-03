mod command_file_service;
mod command_item;
mod commands;
mod configs;
mod gui;
mod utils;

use gui::contexts::app::AppContext;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app_context = AppContext::create()?;

    app_context.render()?;

    app_context.clear()?;
    app_context.callback_command()?;

    Ok(())
}
