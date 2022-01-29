use std::{collections::HashMap, fs::File, io::Read, path::Path};

use serde::{Deserialize, Serialize};

/// Manages configuration variables
/// All configuration details are specified in `velocity.toml`
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// name of the status page
    /// you can find this at the home page of instatus.com
    /// example: Hydralite, Discord, Apple
    pub name: String,
    /// your key to the instatus api
    pub api_key: String,
    /// endpoints to monitor
    pub monitors: HashMap<String, Monitor>,
    /// frequency to monitor endpoints, in seconds
    /// your endpoints will be pinged every `frequency` seconds
    pub frequency: u64,
    /// maximum connection timeout for all endpoints
    /// default: 30s
    pub max_connection_timeout: Option<u64>,
    /// incident monitoring time, in number of requests
    /// when the number of requests specified above have been satisfied, we will mark the incident as resolved
    /// default: 60
    pub incident_monitoring_time: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Monitor {
    pub url: String,
    #[serde(rename = "type")]
    pub type_: MonitorType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MonitorType {
    Uptime,
    Latency,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let mut file = File::open(path).unwrap_or_else(|error| {
            eprintln!("💥 failed to read config file: {}", error);
            std::process::exit(1);
        });

        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let mut config = serde_json::from_str::<Config>(&contents).unwrap();

        if config.max_connection_timeout.is_none() {
            config.max_connection_timeout = Some(30);
        }

        if config.incident_monitoring_time.is_none() {
            config.incident_monitoring_time = Some(60);
        }

        config
    }
}
