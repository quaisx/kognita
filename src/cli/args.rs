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
use std::path::PathBuf;
use std::fmt::Display;
use clap::{command, arg, Command, value_parser, Parser, ArgAction};
use serde::de::value;
use std::str::FromStr;
use libp2p::{
    core::{
        multiaddr::{
            Multiaddr, 
            Protocol
        }
    }
};

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
    pub server_address: Option<Multiaddr>,
    pub debug: u8,
}
impl Display for NodeCliArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut srv_msg: String = "<invalid>".to_string();
        match self.mode {
            Mode::Client => { 
                if let Some(a) = &self.server_address {
                    srv_msg = a.to_string();
                }
            }
            Mode::Server => { srv_msg = "n/a".to_string()}
        }
        write!(f, 
        "node:{}, mode:{}, server_address:{}, debug:{}"
        , self.node, self.mode, srv_msg, self.debug
        )
    }
}

#[derive(Clone, Debug, PartialEq, Parser)]
pub enum Mode {
    Server,
    Client,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result  {
        match self {
            Mode::Client => { write!(f, "client") }
            Mode::Server => { write!(f, "server") }
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
        .subcommand(
            Command::new("client")
                .about("run this node as a client")
                .arg(
                    arg!(
                        -s --server_address <SRV_ADDR> "multi-address of a node that runs as a server"
                    )
                    .required(true)
                    .value_parser(value_parser!(String))
                )
        )
        .subcommand(
            Command::new("server")
            .about("run this node as a server")
        )
        .get_matches();
    let mut _node: String = String::from("");
    // You can check the value provided by positional arguments, or option arguments
    if let Some(node) = matches.get_one::<String>("node") {
        println!("Node name: {node}");
        _node = node.clone();
    } else {
        panic!("Node name must be provided");
    }
    let _config_path: PathBuf;
    if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        println!("Config file to use: {}", config_path.display());
        _config_path = config_path.clone();
    }
    let _debug: u8;
    match matches
        .get_one::<u8>("debug")
        .expect("Count's are defaulted")
    {
        0 => { 
            println!("Debug mode is off");
            _debug = 0;
        },
        1 => {
            println!("Debug mode is on");
            _debug = 1;
        },
        2 => { 
            println!("Trace mode is on");
            _debug = 2;
        },
        _ => {
            println!("Trace mode is on");
            _debug = 2;
        },
    }
    let mut _mode: Mode = Mode::Client;
    let mut _srv_addr: Multiaddr = Multiaddr::empty();
    let mut _server_address: Option<Multiaddr> = None;
    if let Some(matches) = matches.subcommand_matches("client") {
        _mode = Mode::Client;
        if let Some(server_addr) = matches.get_one::<String>("server_address") {
            println!("Printing testing lists...{server_addr}");
            _srv_addr = Multiaddr::from_str(&server_addr).expect("Server address must be a valid multiaddress");
            _server_address = Some(_srv_addr);
        }
    }

    if let Some(_) = matches.subcommand_matches("server") {
        _mode = Mode::Server;
    }
    NodeCliArgs { node: _node, mode: _mode, server_address: _server_address, debug: _debug }
}