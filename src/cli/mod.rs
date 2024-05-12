mod csv;
mod genpass;
use clap::Parser;

pub use csv::CsvOpts;
pub use csv::OutputFormat;
pub use genpass::GenPassOpts;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name="rcli",version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "generate random password")]
    GenPass(GenPassOpts),
}
