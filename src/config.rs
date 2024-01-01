use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use clap::{Parser};


pub fn data_dir_absolute_path(data_dir: String) -> PathBuf {
    if let Some(a) = data_dir.strip_prefix('~') {
        if let Some(b) = data_dir.strip_prefix("~/") {
            home::home_dir().unwrap().join(b)
        } else {
            home::home_dir().unwrap().join(a)
        }
    } else {
        PathBuf::from(&data_dir)
    }
}

pub fn from_file<T: Default + serde::de::DeserializeOwned>(path: &PathBuf) -> T {
    match std::fs::read(path) {
        Ok(file_content) => toml::from_slice::<T>(&file_content).map_or_else(
            |e| {
                eprintln!("Couldn't parse config file: {e}");
                T::default()
            },
            |config| config,
        ),
        Err(_) => T::default(),
    }
}

/// Error raised if something is wrong with the configuration.
#[derive(PartialEq, Eq, Debug)]
pub struct ConfigError(String);

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Configuration error: {}", self.0)
    }
}

impl std::error::Error for ConfigError {}

#[derive(Parser, Debug)]
#[clap(
    name = "ToyRaft",
    author,
    about,
    long_about = "A Rust Implementation Of Raft Specification",
    version
)]
pub struct Opt {

}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Config {
    // API
    node_name: String,
    nodes: Vec<NodeConfig>
    
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct NodeConfig {
    name: String,
    address: String,
}

impl Config {

}

