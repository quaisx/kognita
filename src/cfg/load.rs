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

use super::defaults;

static KOGNITA_CONFIG: &'static str = "KOGNITA_CONFIG";

#[derive(Debug, Deserialize)]
pub struct NodeConfig {
    pub description: Option<String>,
    pub version: Option<String>,
    pub net: Option<NetConfig>,
    pub peer: Option<PeerConfig>,
}

impl NodeConfig {
    pub fn new() -> NodeConfig {
        NodeConfig {
            description: Some(defaults::DEF_DESCRIPTION.to_string()),
            version: Some(defaults::DEF_VERSION.to_string()),
            net: Some(NetConfig::new()),
            peer: Some(PeerConfig::new()),
        }
    }
    pub fn description(&self) -> &String {
        self.description.as_ref().unwrap()
    }
    pub fn version(&self) -> &String {
        self.version.as_ref().unwrap()
    }
    pub fn net(&mut self) -> &NetConfig {
        self.net.as_ref().unwrap()
    }
    pub fn peer(&mut self) -> &PeerConfig {
        self.peer.as_ref().unwrap()
    }
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

impl NetConfig {
    pub fn new() -> NetConfig {
        NetConfig {
            tcp_nodelay: Some(defaults::DEF_TCP_NODELAY),
            tcp_portreuse: Some(defaults::DEF_TCP_PORTREUSE),
            tcp_timeout: Some(defaults::DEF_TCP_TIMEOUT),
            quic_handshake_timeout: Some(defaults::DEF_QUIC_HANDSHAKE_TIMEOUT),
        }
    }
    pub fn tcp_nodelay(&self) -> bool {
        self.tcp_nodelay.unwrap_or(defaults::DEF_TCP_NODELAY)
    }
    pub fn tcp_portreuse(&self) -> bool {
        self.tcp_portreuse.unwrap_or(defaults::DEF_TCP_PORTREUSE)
    }
    pub fn tcp_timeout(&self) -> u64 {
        self.tcp_timeout.unwrap_or(defaults::DEF_TCP_TIMEOUT)
    }
    pub fn quic_handshake_timeout(&self) -> u64 {
        self.quic_handshake_timeout.unwrap_or(defaults::DEF_QUIC_HANDSHAKE_TIMEOUT)
    }
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
    pub rendezvous_point: Option<String>,
}

impl PeerConfig {
    pub fn new() -> PeerConfig {
        PeerConfig {
            pubsub_heartbeat_interval: None,
            pubsub_duplicate_cache_time: None,
            pubsub_topic: Some(String::from(defaults::DEF_PUBSUB_TOPIC)),
            mdns_query_interval: None,
            mdns_ttl: None,
            ping_interval: None,
            kad_query_timeout: None,
            rendezvous_point: Some(String::from(defaults::DEF_RENDEZVOUS_POINT)),
        }
    }
    pub fn pubsub_heartbeat_interval(&self) -> u64 {
        self.pubsub_heartbeat_interval.unwrap_or(defaults::DEF_PUBSUB_HEARTBEAT_INTERVAL)
    }
    pub fn pubsub_duplicate_cache_time(&self) -> u64 {
        self.pubsub_duplicate_cache_time.unwrap_or(defaults::DEF_PUBSUB_DUPLICATE_CACHE_TIME)
    }
    pub fn pubsub_topic(&self) -> &String {
        self.pubsub_topic.as_ref().unwrap()
    }
    pub fn mdns_query_interval(&self) -> u64 {
        self.mdns_query_interval.unwrap_or(defaults::DEF_MDNS_QUERY_INTERVAL)
    }
    pub fn mdns_ttl(&self) -> u64 {
        self.mdns_ttl.unwrap_or(defaults::DEF_MDNS_TTL)
    }
    pub fn ping_interval(&self) -> u64 {
        self.ping_interval.unwrap_or(defaults::DEF_PING_INTERVAL)
    }
    pub fn kad_query_timeout(&self) -> u64 {
        self.kad_query_timeout.unwrap_or(defaults::DEF_KAD_QUERY_TIMEOUT)
    }
    pub fn rendezvous_point(&self) -> &String {
        self.rendezvous_point.as_ref().unwrap()
    }
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