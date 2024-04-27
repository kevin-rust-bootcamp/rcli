use std::{fmt::Display, fs, str::FromStr};

use crate::CmdExecutor;
use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, long, help = "Input file path")]
    pub input: String,
    #[arg(short, long, help = "Output file path")]
    pub output: Option<String>,
    #[arg(short, long, help = "Output file format", default_value = "json")]
    pub format: OutputFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid output format: {}", s)),
        }
    }
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> &'static str {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Yaml => write!(f, "yaml"),
        }
    }
}

impl CmdExecutor for CsvOpts {
    fn execute(self) -> anyhow::Result<()> {
        // Build the CSV reader and iterate over each record.
        let mut rdr = Reader::from_path(self.input)?;
        let mut rets = Vec::with_capacity(128);
        let header = rdr.headers()?.clone();
        for result in rdr.records() {
            let record = result?;
            let value = header
                .iter()
                .zip(record.iter())
                .collect::<serde_json::Value>();
            rets.push(value);
        }
        let output = if let Some(output) = self.output {
            output.clone()
        } else {
            format!("output.{}", self.format)
        };
        let content = match self.format {
            OutputFormat::Json => serde_json::to_string_pretty(&rets)?,
            OutputFormat::Yaml => serde_yaml::to_string(&rets)?,
        };
        fs::write(output, content)?;
        Ok(())
    }
}
#[derive(Debug, Deserialize, Serialize)]
struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}
