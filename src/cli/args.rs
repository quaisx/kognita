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

#![allow(unused_imports)]
#![allow(dead_code)]
use clap::{arg, command, value_parser, ArgAction, Command, Parser};
use libp2p::core::multiaddr::{Multiaddr, Protocol};
use serde::de::value;
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Parser)]
#[command(name = "Kognita")]
#[command(author = "Qua Is X")]
#[command(version = "1.0")]
#[command(
    about = "Kognita crypto platform",
    long_about = "Kognita is an open source project implemented in Rust"
)]
pub struct NodeCliArgs {
    pub node: String,
    pub mode: Mode,
    pub config: Option<String>,
    pub server_address: Option<Vec<Multiaddr>>,
    pub grpc_server_port: Option<u16>,
    pub debug: u8,
    pub port: Option<u16>,
}
impl Display for NodeCliArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut srv_msg: String = "<invalid>".to_string();
        match self.mode {
            Mode::Client => {
                if let Some(a) = &self.server_address {
                    for x in a {
                        srv_msg.push_str(&x.to_string());
                        srv_msg.push_str(" ");
                    }
                }
            }
            Mode::Server => srv_msg = "n/a".to_string(),
        }
        let mut grpc_port: u16 = 0;
        if let Some(p) = &self.grpc_server_port {
            grpc_port = *p;
        }
        let mut port: u16 = 0;
        if let Some(p) = self.port {
            port = p;
        }
        match self.mode {
            Mode::Client => {
                write!(
                    f,
                    "node:{}, mode:{}, server_address:{}, debug:{}",
                    self.node, self.mode, srv_msg, self.debug
                )
            }
            Mode::Server => {
                write!(
                    f,
                    "node:{}, mode:{}, port:{}, grpc-port:{}, debug:{}",
                    self.node, self.mode, port, grpc_port, self.debug
                )
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Parser)]
pub enum Mode {
    Server,
    Client,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Client => {
                write!(f, "client")
            }
            Mode::Server => {
                write!(f, "server")
            }
        }
    }
}

impl FromStr for Mode {
    type Err = String;
    fn from_str(mode: &str) -> Result<Self, Self::Err> {
        match mode {
            "server" => Ok(Mode::Server),
            "client" => Ok(Mode::Client),
            _ => Err("Expected args: 'server' or 'client'".to_string()),
        }
    }
}

pub fn parse_args() -> NodeCliArgs {
    NodeCliArgs::parse()
}

pub fn parse_cli() -> NodeCliArgs {
    let matches = command!() // requires `cargo` feature
        .arg(arg!([node] "Node name to operate on"))
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(
                -d --debug ... "Enable debug level logs"
        ))
        .arg(
            arg!(
                --grpc_port <PORT>)
            .value_parser(value_parser!(u16))
            .default_value("50551")
            .required(true),
        )

        .subcommand(
            Command::new("client")
                .about("run this node as a client")
                .arg(
                    arg!(
                        -s --server_address <SRV_ADDR> "multi-address of a node that runs as a server"
                    )
                .required(true)
                .value_parser(value_parser!(String)),
            ),
        )
        .subcommand(
            Command::new("server")
            .about("run this node as a server")
            .arg(
                arg!(
                    -p --port <PORT>)
                .value_parser(value_parser!(u16))
                .default_value("0")
                .required(false),
            )
        )
        .get_matches();
    
    let mut _config: Option<String> = None;
    if let Some(c) = matches.get_one::<String>("config") {
        _config = Some(c.clone());
    }
    let mut _grpc_port: Option<u16> = None;
    if let Some(p) = matches.get_one::<u16>("grpc_port") {
            _grpc_port = Some(*p);
    }

    let mut _node: String = String::from("");
    // You can check the value provided by positional arguments, or option arguments
    if let Some(node) = matches.get_one::<String>("node") {
        _node = node.clone();
    } else {
        panic!("Node name must be provided");
    }
    let _config_path: PathBuf;
    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        _config_path = config_path.clone();
    }
    let _debug: u8;
    match matches
        .get_one::<u8>("debug")
        .expect("Count's are defaulted")
    {
        0 => {
            _debug = 0;
        }
        1 => {
            _debug = 1;
        }
        _ => {
            _debug = 2;
        }
    }
    let mut _mode: Mode = Mode::Client;
    let mut _server_addresses: Option<Vec<Multiaddr>> = None;
    if let Some(sub_matches) = matches.subcommand_matches("client") {
        _mode = Mode::Client;
        if let Some(server_addrs) = sub_matches.get_one::<String>("server_address") {
            let mut addrs = vec![];
            for addr in server_addrs.trim().split(",") {
                let r = Multiaddr::from_str(&addr);
                if r.is_ok() {
                    addrs.push(r.unwrap());
                }
            }
            if addrs.len() > 0 {
                _server_addresses = Some(addrs);
            }
        }
    }

    let mut _port: Option<u16> = None;
    if let Some(sub_matches) = matches.subcommand_matches("server") {
        _mode = Mode::Server;
        if let Some(port) = sub_matches.get_one::<u16>("port") {
            _port = Some(*port);
        }
    }

    NodeCliArgs {
        node: _node,
        mode: _mode,
        config: _config,
        server_address: _server_addresses,
        grpc_server_port: _grpc_port,
        debug: _debug,
        port: _port,
    }
}
