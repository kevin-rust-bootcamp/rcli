use crate::process::process_genpass;
use crate::CmdExecutor;
use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct GenPassOpts {
    #[arg(
        short,
        long,
        default_value_t = 1,
        help = "the count of passwords to generate"
    )]
    pub count: u32,
    #[arg(
        short,
        long,
        default_value_t = 16,
        help = "the length of password to generate"
    )]
    pub length: u8,
    #[arg(
        short,
        long,
        default_value_t = true,
        help = "include uppercase letters"
    )]
    pub uppercase: bool,
    #[arg(long, default_value_t = false, help = "include lowercase letters")]
    pub lowercase: bool,
    #[arg(short, long, default_value_t = true, help = "include numbers")]
    pub number: bool,
    #[arg(short, long, default_value_t = false, help = "include symbols")]
    pub symbol: bool,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "output the strength of the password"
    )]
    pub output_strength: bool,
}

impl CmdExecutor for GenPassOpts {
    fn execute(self) -> Result<()> {
        process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.number,
            self.symbol,
            self.count,
            self.output_strength,
        )
        .unwrap();
        Ok(())
    }
}
