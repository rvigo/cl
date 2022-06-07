use commands::Commands;
use gui::structs::app_context::AppContext;

mod command_item;
mod commands;
mod config;
mod file_service;
mod gui;
mod utils;
use std::error::Error;

/*
    TODO inserir tela de insert REFATORADA
    TODO repensar em como guardar o estado dos campos selecionados
*/

fn main() -> Result<(), Box<dyn Error>> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let mut commands = Commands::init();
    let mut namespaces = commands.namespaces();

    namespaces.insert(0, "All".to_string());

    let mut app_context = AppContext::new(commands, namespaces)?;

    app_context.render()?;

    app_context.restore_terminal()?;

    Ok(())
}
