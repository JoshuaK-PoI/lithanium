use clap::Parser;


/// Argument parser using `clap`
#[derive(Parser, Debug)]
#[command(name = "lithc")]
pub(crate) struct Args {
    #[arg(short, long)]
    pub(crate) input: String,
    #[arg(short, long, required = false, default_value = "")]
    pub(crate) output: String,
    #[arg(short, long, required = false)]
    pub(crate) debug: bool,
}