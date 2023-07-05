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

#![allow(deprecated)]
#![allow(unused_imports)]

use async_std::task::JoinHandle;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender, UnboundedReceiver};
use futures::{future::Either, prelude::*, select, StreamExt};
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::{
    self, GetClosestPeersError, Kademlia, KademliaConfig, KademliaEvent, QueryResult,
};
use libp2p::{
    core::{
        multiaddr::{Multiaddr, Protocol},
        muxing::StreamMuxerBox,
        transport::OrTransport,
        upgrade,
    },
    gossipsub, identify, identity, mdns,
    mdns::Mdns,
    noise, ping,
    ping::Success,
    quic,
    swarm::NetworkBehaviour,
    swarm::{keep_alive, Swarm, SwarmBuilder, SwarmEvent, ListenError},
    tcp,
    tcp::TokioTcpTransport,
    yamux, PeerId, Transport,
};
use tonic::{transport::Server, Request, Response, Status};
use emojis;
use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher as hash_Hasher};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::time::{SystemTime, SystemTimeError};
// use futures_ticker;
use lipsum::lipsum;
use log::{debug, error, info, trace, warn};
use std::env;
use std::thread;
use std::cell::RefCell;
use super::super::cli::args::Mode;
use super::super::cli::args::NodeCliArgs;
use super::super::cfg::load::NodeConfig;
use super::super::service::post;
use super::config;
use super::p2p::P2pNode;
use super::p2p::PeerNetBehaviourEvent;
extern crate pretty_env_logger;

pub async fn run(args: &NodeCliArgs, node_config: Box<NodeConfig>) -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let mut node = P2pNode::new();
    let mut my_swarm = node.init(args, node_config).unwrap();

    match args.mode {
        Mode::Server => {
            // Listen on all ipv4 interfaces and ephemeral ports
            my_swarm.listen_on(format!("/ip4/0.0.0.0/udp/{}/quic-v1", args.port.unwrap()).parse()?)?;
            my_swarm.listen_on(format!("/ip4/0.0.0.0/tcp/{}", args.port.unwrap()).parse()?)?;
        }
        Mode::Client => {
            if let Some(srv_addrs) = &args.server_address {
                for a in srv_addrs {
                    let mut prot_stack = a.protocol_stack();
                    let ma: Multiaddr;
                    _ = prot_stack.next();
                    match prot_stack.next() {
                        Some(x) => match x {
                            "udp" => {
                                ma = a.clone().with(Protocol::QuicV1);
                            }
                            _ => ma = a.clone(),
                        },
                        _ => {
                            panic!("failed to format server address")
                        }
                    }
                    my_swarm.dial(ma).unwrap();
                }
            }
        }
    }
    let grpc_port = args.grpc_server_port;
    let (chs, mut chr) = unbounded_channel::<String>();
    let z = Arc::new(Mutex::new(chs.clone()));
    let handle = tokio::spawn(async move {
        post::run(grpc_port, z).await
    });

    let num = rand::thread_rng().gen_range(5..10);
    // let mut tcr = futures_ticker::Ticker::new_with_next(Duration::from_secs(num), Duration::from_secs(10)).fuse();

    let mut tcr = tokio::time::interval(Duration::from_secs(num));
      // Create a Gossipsub topic
    let topic = gossipsub::Sha256Topic::new(node.node_config.peer().pubsub_topic());
      // subscribes to our topic
    my_swarm.behaviour_mut().gossipsub.subscribe(&topic)?;
    
    // Infinite loop
    loop {
        // ---> SELECT EVENT
        tokio::select! {
            r = chr.recv() => {
                if let Some(msg) = r {
                    if let Err(e) = my_swarm
                        .behaviour_mut()
                        .gossipsub
                        .publish(
                            topic.clone(),
                            msg.as_bytes())
                    {
                        error!("{}  ~ <PUBLISH> error: {e:?}", &config::E_WARN.clone());
                    }
                }
            },
            // Handle user input - take input as a txt message and publish to the topic
            _ = tcr.tick() => {
                let num_words = rand::thread_rng().gen_range(3..15);
                let t = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
                let payload = format!("[{t}] - {}", lipsum(num_words));
                if let Err(e) = my_swarm
                    .behaviour_mut()
                    .gossipsub
                    .publish(
                        topic.clone(),
                        payload.as_bytes())
                {
                    error!("{}  ~ <PUBLISH> error: {e:?}", &config::E_WARN.clone());
                }
            },
            // Handle swarm events
            event = my_swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(
                    PeerNetBehaviourEvent::Ping(
                        ping::Event {
                            peer,
                            ..
                })) if peer != node.peer_id => {
                    info!("{}  ~ <PING> to {} ", &config::E_PING.clone(), peer);
                },
                // mDNS discovery event
                SwarmEvent::Behaviour(
                    PeerNetBehaviourEvent::Mdns(
                        mdns::Event::Discovered(list)
                    )
                ) => {
                    for (peer_id, ma) in list {
                        info!("{}  ~ <mDNS> discovered a new peer: {peer_id}:{ma}", &config::E_DISC.clone());
                        my_swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    }
                },
                // mDNS expiry event
                SwarmEvent::Behaviour(
                    PeerNetBehaviourEvent::Mdns(
                        mdns::Event::Expired(list)
                    )
                ) => {
                    for (peer_id, _multiaddr) in list {
                        warn!("{}  ~ <mDNS> discover peer has expired: {peer_id}", config::E_WARN.clone());
                        my_swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                    }
                },
                //PeerNetBehaviourEvent: Subscribed { peer_id: PeerId("12D3KooWNNS1ijqqnjLEhfS5q5uJw4K7BhjAY9o9A6v4727vREd5"), topic: TopicHash { hash: "kognita/tx" } })
                SwarmEvent::Behaviour(
                   PeerNetBehaviourEvent::Gossipsub(
                      gossipsub::Event::Subscribed { peer_id, topic }
                   )
                ) => info!(
                   "{} ~ <SUBSCRIBE> {peer_id} subscribed to {topic}", config::E_EVT.clone()
                ),
                // incoming sub event
                SwarmEvent::Behaviour(
                    PeerNetBehaviourEvent::Gossipsub(
                        gossipsub::Event::Message {
                            propagation_source: peer_id,
                            message_id: id,
                            message,
                        }
                    )
                ) => {
                    info!(
                        "{}  ~ <MESSAGE>: '{}' with id: {id} from peer: {peer_id}",
                        config::E_EVT.clone(),
                        String::from_utf8_lossy(&message.data),
                    );
                    let payload = String::from_utf8_lossy(&message.data);
                    if payload == "QUIT" {
                        break;
                    }
                },
                // New address found event swarm will listen on
                SwarmEvent::NewListenAddr { address, .. } => {
                    info!("{}  ~ <NET> [{}]:{} is listening on {address}",
                     config::E_INTR.clone(),
                     &node.node_name,
                     &node.peer_id
                 );
                },
                SwarmEvent::Dialing (pid) => {
                    info!("{}  ~ <NET> Dialing {pid}", config::E_DIAL.clone());
                },
                SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
                    info!("{}  ~ <NET> Connected to {}:{}", config::E_PLUG.clone(), peer_id, endpoint.get_remote_address());
                },
                SwarmEvent::ConnectionClosed { peer_id, endpoint, .. } => {
                     info!("{}  ~ <NET> Disconnected from {peer_id}:{}", config::E_PLUG.clone(), endpoint.get_remote_address());
                },
                 SwarmEvent::IncomingConnection {
                     local_addr, send_back_addr
                 } => {
                     info!("{}  ~ <NET> Incoming Connection from {}->{}",
                         config::E_PLUG.clone(),
                         send_back_addr,
                         local_addr
                     );
                 },
                 SwarmEvent::OutgoingConnectionError { peer_id, error, .. } => {
                    info!("{}  ~ <NET> Outgoing connection error to {:?}: {:#?}",
                        config::E_ERR.clone(), 
                        peer_id, 
                        error);
                }
                SwarmEvent::IncomingConnectionError {
                    local_addr, send_back_addr, error
                 } => {
                    match error {
                        ListenError::Transport(tv) => {
                            error!(
                                "{}  ~ <NET> Connection {} -> {} error: {:#?}",
                                config::E_ERR.clone(),
                                send_back_addr,
                                local_addr,
                                tv
                            );       
                        },
                        _ => {
                            error!(
                                "{}  ~ <NET> Connection {} -> {} error: {error}",
                                config::E_ERR.clone(),
                                send_back_addr,
                                local_addr,
                                error = error
                            );        
                        }
                    }
                 },
                 SwarmEvent::Behaviour(
                     PeerNetBehaviourEvent::Kad (
                         kad::KademliaEvent::OutboundQueryProgressed {
                             result: QueryResult::GetClosestPeers(result),
                             ..
                         }
                     )
                 ) => {
                     match result {
                         Ok(ok) => {
                             if !ok.peers.is_empty() {
                                 info!("<KAD Q> closest peers: {:#?}", ok.peers)
                             } else {
                                 warn!("<KAD Q> no closest peers found")
                             }
                         }
                         Err(GetClosestPeersError::Timeout { peers, .. }) => {
                             if !peers.is_empty() {
                                 warn!("<KAD Q> Query timed out: closest peers: {peers:#?}")
                             } else {
                                 error!("<KAD Q> Query timed out: no closest peers");
                             }
                         }
                     };
                 },
                 SwarmEvent::Behaviour(
                     PeerNetBehaviourEvent::Ping (
                         ping::Event {
                            peer, result
                         }
                     )
                 ) => {
                    if let Ok(x) = result {
                        match x {
                            Success::Pong => {
                                info!("{}  ~ <PING RCVD> {peer} is alive", config::E_PING.clone());
                            },
                            _ => {
                                info!("{}  ~ <PING SENT> {peer}", config::E_PING.clone());
                            },
                        }
                    }
                 },
                 SwarmEvent::Behaviour(
                    PeerNetBehaviourEvent::Identify(
                        identify::Event::Sent { peer_id: pid })
                ) => {
                    info!("{}  ~ <ID> Sent identity data to {pid}", config::E_ID.clone());
                },
                SwarmEvent::Behaviour(
                    PeerNetBehaviourEvent::Identify(
                        identify::Event::Received {
                            peer_id: pid,
                            info: identify::Info { listen_addrs, protocols, .. },
                        }
                    )
                ) => {
                    info!("{}  ~ <ID> Received identity from {pid}: {:#?}, {:#?}",
                        config::E_ID.clone(),
                        listen_addrs,
                        protocols
                    );
                }
                 other_event => {
                    warn!("{}  ~ <UNHANDLED> {:#?}", config::E_EVT.clone(), other_event);
                }
            }
        }
    }
    let _grpc_stop = handle.await.expect("Failed to join gRPC thread");
    Ok(())
}
