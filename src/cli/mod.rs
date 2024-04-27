mod csv;
use clap::Parser;

pub use csv::CsvOpts;

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
}
