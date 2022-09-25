use clap::Parser;

#[derive(Parser)]
pub struct Config {
    #[clap(
        long,
        short = 'q',
        required = false,
        takes_value = true,
        help = "Set the default quiet mode"
    )]
    pub default_quiet_mode: Option<bool>,
}
