/*
 _        _______  _______  _       __________________ _______ 
| \    /\(  ___  )(  ____ \( (    /|\__   __/\__   __/(  ___  )
|  \  / /| (   ) || (    \/|  \  ( |   ) (      ) (   | (   ) |
|  (_/ / | |   | || |      |   \ | |   | |      | |   | (___) |
|   _ (  | |   | || | ____ | (\ \) |   | |      | |   |  ___  |
|  ( \ \ | |   | || | \_  )| | \   |   | |      | |   | (   ) |
|  /  \ \| (___) || (___) || )  \  |___) (___   | |   | )   ( |
|_/    \/(_______)(_______)|/    )_)\_______/   )_(   |/     \|
                                                               
@authors: free thinkers of the world
    1. Qua Is X (Ukraine) qua.is.kyiv.ua@gmail.com
    /add your name here.../

 */

 // #![deny(warnings)]
 #![allow(dead_code)]

use toml;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use dirs;

static KOGNITA_CONFIG: &'static str = "KOGNITA_CONFIG";

#[derive(Debug, Deserialize)]
pub struct NodeConfig {
    pub description: Option<String>,
    pub version: Option<String>,
    pub net: Option<NetConfig>,
    pub peer: Option<PeerConfig>,
}

/// Sub-structs are decoded from tables, so this will decode from the `[server]`
/// table.
///
/// Again, each field is optional, meaning they don't have to be present.
#[derive(Debug, Deserialize)]
pub struct NetConfig {
    pub tcp_nodelay: Option<bool>,
    pub tcp_portreuse: Option<bool>,
    pub tcp_timeout: Option<u64>,
    pub quic_handshake_timeout: Option<u64>,
 }

#[derive(Debug, Deserialize)]
pub struct PeerConfig {
    pub pubsub_heartbeat_interval: Option<u64>,
    pub pubsub_duplicate_cache_time: Option<u64>,
    pub pubsub_topic: Option<String>,
    pub mdns_query_interval: Option<u64>,
    pub mdns_ttl: Option<u64>,
    pub ping_interval: Option<u64>,
    pub kad_query_timeout: Option<u64>,
}

fn load_config(cfg_path: &String) -> Result<Box<NodeConfig>, Box<dyn std::error::Error>> {
    let mut payload = String::new();
    File::open(cfg_path)
    .and_then(|mut f| f.read_to_string(&mut payload))
    .unwrap();
    let node_config: NodeConfig = toml::from_str(payload.as_str())?;
    Ok(Box::new(node_config))
}

fn load_from_env() -> Result<Box<NodeConfig>, Box<dyn std::error::Error>> {
    if let Ok(cfg_path) = env::var(KOGNITA_CONFIG) {
        if std::path::Path::new(&cfg_path).exists() {
            load_config(&cfg_path)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", cfg_path),
            )))
        }
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Environment variable not set",
        )))
    }
}

fn load_from_file(path: &String) -> Result<Box<NodeConfig>, Box<dyn std::error::Error>> {
    if std::path::Path::new(path).exists() {
        load_config(path)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", path),
        )))
    }
}

fn load_from_default() -> Result<Box<NodeConfig>, Box<dyn std::error::Error>> {
    let mut hd = dirs::home_dir().unwrap();
    hd.push(".kognita");
    hd.push("node.toml");
    let c: String = String::from(hd.as_path().to_str().unwrap());
    if std::path::Path::new(hd.as_os_str()).exists() {
        load_config(&c)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", c),
        )))
    }
}

pub fn load_node_config(config_file: &Option<String>) -> Result<Box<NodeConfig>, Box<dyn std::error::Error>> {
    // Conditions of loading the config file
    // 1 if the config file is given on the command line, use it if exists
    // 2 if the config file is not given, check the env var KOGNITA_NODE_CONFIG
    // 3 if the config file is not given, check the default location ~/.kognita/node.toml
    // 4 if the config file is not found, return an error

    // Condition 1 -----------------------
    if let Some(cfg_file) = config_file {
        if let Ok(cfg) = load_from_file(cfg_file) {
            return Ok(cfg);
        }
    }

    // Condition 2 -----------------------
    if let Ok(cfg) = load_from_env() {
        return Ok(cfg);
    }

    // Condition 3 -----------------------
    if let Ok(cfg) = load_from_default() {
        return Ok(cfg);
    }

    // Default -----------------------
    Err(
        Box::new(
            std::io::Error::new(
                std::io::ErrorKind::NotFound, 
                "No valid Kognita node configuration file found"
            )
        )
    )
}