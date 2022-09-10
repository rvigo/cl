use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    Export,
    Import,
}

#[derive(Parser)]
pub struct Share {
    #[clap(value_parser, arg_enum, help = "Export/Import mode")]
    pub mode: Mode,
    #[clap(
        short,
        long = "file_location",
        required = false,
        default_value = "shared.toml",
        hide_default_value = true,
        help = "If <MODE> is `export`, the location of the output file\n\
        If `import`, the location of the source file\n\
        (Default value is the current directory)",
        value_parser
    )]
    pub file_location: PathBuf,
    #[clap(
        short,
        long,
        multiple_values = true,
        help = "The namespace(s) to be imported from/exported to file\nIf none, all aliases will be processed"
    )]
    pub namespace: Option<Vec<String>>,
}
