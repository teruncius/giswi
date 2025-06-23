use std::{collections::HashMap, process::Command};
use log::*;
use clap::Parser;
use serde::{Deserialize};

#[derive(Parser)]
struct Cli {
    name: String,
}

#[derive(Deserialize, Debug)]
struct Config {
    name: String,
    email: String,
    signingkey: Option<String>,
    gpgsign: bool,
}

fn main() {
    stderrlog::new().module(module_path!()).init().unwrap();

    let args = Cli::parse();
    debug!("{}", args.name);

    let home = std::env::home_dir().unwrap();
    debug!("{:?}", home);
    let path = home.join(".giswi.json");
    debug!("{:?}", path);

    let content = std::fs::read_to_string(path).unwrap();
    debug!("{}", content);

    let config: HashMap<String, Config> = serde_json::from_str(&content).unwrap();
    debug!("{:?}", config);

    let entry = config.get(&args.name).unwrap();
    debug!("{:?}", entry);

    let mut command = Command::new("git");
    command.args(["config", "--global", "user.name", &entry.name]);
    debug!("{:?}", command);
    let output = command.output().unwrap();
    debug!("{:?}", output);

    let mut command = Command::new("git");
    command.args(["config", "--global", "user.email", &entry.email]);
    debug!("{:?}", command);
    let output = command.output().unwrap();
    debug!("{:?}", output);

    let mut command = Command::new("git");
    if entry.signingkey.is_some() {
        let key = entry.signingkey.as_ref().unwrap();
        command.args(["config", "--global", "user.signingkey", &key]);
    } else {
        command.args(["config", "--global", "--unset", "user.signingkey"]);
    }
    debug!("{:?}", command);
    let output = command.output().unwrap();
    debug!("{:?}", output);

    let sign = if entry.gpgsign { "true" } else { "false" };
    let mut command = Command::new("git");
    command.args(["config", "--global", "commit.gpgsign", sign]);
    debug!("{:?}", command);
    let output = command.output().unwrap();
    debug!("{:?}", output);
}
