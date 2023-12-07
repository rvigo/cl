#[macro_export]
macro_rules! load_command_handler {
    () => {{
        use cl_core::resource::commands_file_handler::CommandsFileHandler;
        CommandsFileHandler::new("../benches/data/sample.toml".into())
    }};
    ($path:expr) => {
        use cl_core::resource::commands_file_handler::CommandsFileHandler;
        CommandsFileHandler::new($path.into())
    };
}

#[macro_export]
macro_rules! build_command {
    (
        $(namespace => $namespace:expr;)?
        $(command => $command:expr;)?
        $(alias =>  $alias:expr;)?
        $(tags=>   $tags:expr;)?
        $(description => $description:expr;)?
    ) => {{
        use cl_core::command::CommandBuilder;

        let mut namespace = "any";
        let mut command = "any";
        let mut alias = "any";
        let mut tags = None::<Vec<String>>;
        let mut description = None::<String>;

        $(namespace = $namespace;)?
        $(command = $command;)?
        $(alias = $alias;)?
        $(tags = $tags;)?
        $(description = $description;)?

        let mut builder = CommandBuilder::default();
        builder
            .namespace(namespace)
            .command(command)
            .alias(alias)
            .tags(tags)
            .description(description);

        builder.build()
    }};
}
