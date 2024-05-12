use std::fs;

use csv::Reader;
use serde::Deserialize;
use serde::Serialize;

use crate::cli::CsvOpts;
use crate::CmdExecutor;
use crate::OutputFormat;

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
