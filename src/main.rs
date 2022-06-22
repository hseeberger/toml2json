use anyhow::{Context, Result};
use serde_json::Value as JsonValue;
use std::{
    fs,
    io::{stdin, BufRead},
};
use structopt::StructOpt;
use toml::de;

#[derive(Debug, StructOpt)]
#[structopt(about = "convert TOML to JSON")]
struct Opt {
    #[structopt(short, long)]
    file: Option<String>,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let toml = match &opt.file {
        Some(file) => read_from_file(file)?,
        None => read_from_stdin()?,
    };
    let json = de::from_str::<JsonValue>(&toml).context("Cannot convert TOML to JSON")?;
    println!("{json}");

    Ok(())
}

fn read_from_file(file: &str) -> Result<String> {
    fs::read_to_string(file).context("Cannot read TOML input")
}

fn read_from_stdin() -> Result<String> {
    stdin()
        .lock()
        .lines()
        .try_fold(String::default(), |lines, line| {
            line.map(|line| format!("{lines}{line}\n"))
        })
        .context("Cannot read TOML from stdin")
}
