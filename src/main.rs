use std::{collections::HashMap, process::Command};
use log::*;
use clap::{Parser, Subcommand};
use serde::{Deserialize};

#[derive(Parser)]
#[derive(Debug)]
#[command(name = "giswi")]
#[command(version = "0.1.0")]
#[command(about = "Switch between multiple git profiles", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
#[derive(Debug)]
enum Commands {
    /// Switch to a specific git profile
    To {
        #[clap(help = "Name of the profile to switch to")]
        name: String
    },
    /// List all available git profiles
    List { },
}

#[derive(Deserialize, Debug)]
struct Config {
    name: String,
    email: String,
    signingkey: Option<String>,
    gpgsign: bool,
}

type ConfigMap = HashMap<String, Config>;

fn main() {
    stderrlog::new().module(module_path!()).init().unwrap();

    let cli = Cli::parse();
    debug!("{:?}", cli.command);

    match &cli.command {
        Commands::To { name } => {
            debug!("switching to profile");
            switch_to_profile(&name);
        }
        Commands::List { } => {
            debug!("list profiles");
            list_profiles();
        }
    }
}

fn load_config() -> ConfigMap {
    let home: std::path::PathBuf = std::env::home_dir().unwrap();
    debug!("{:?}", home);
    let path = home.join(".giswi.json");
    debug!("{:?}", path);

    let content = std::fs::read_to_string(path).unwrap();
    debug!("{}", content);

    let config: HashMap<String, Config> = serde_json::from_str(&content).unwrap();
    debug!("{:?}", config);

    return config;
}

fn list_profiles() {
    let config = load_config();
    for name in config.keys() {
        println!("{}", name);
    }
}

fn switch_to_profile(name: &String) {
    let config = load_config();
    let entry = config.get(name).unwrap();
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
