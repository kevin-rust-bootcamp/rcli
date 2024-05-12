use clap::Parser;
use rcli::{CmdExecutor, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(csv_opts) => csv_opts.execute().unwrap(),
        SubCommand::GenPass(opts) => opts.execute().unwrap(),
    }
    Ok(())
}
