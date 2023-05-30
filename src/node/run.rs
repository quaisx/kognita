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
    noise, 
    ping, ping::Success, 
    quic,
    swarm::NetworkBehaviour,
    swarm::{keep_alive, SwarmBuilder, SwarmEvent},
    tcp,
    tcp::TokioTcpTransport,
    yamux, PeerId, Transport,
};

use emojis;
use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use std::time::{SystemTime, SystemTimeError};
// use futures_ticker;
use lipsum::lipsum;
use log::{debug, error, info, trace, warn};
use std::env;

use super::super::cli::args::Mode;
use super::super::cli::args::NodeCliArgs;
use super::config;
extern crate pretty_env_logger;

const PUBSUB_TOPIC: &str = "kognita/tx";

// Custom Swarm Network behaviors
#[derive(NetworkBehaviour)]
struct PeerNetBehaviour {
    identify: identify::Behaviour,
    gossipsub: gossipsub::Behaviour,      // to handle pubsub events
    mdns: libp2p::mdns::tokio::Behaviour, // to handle mDSN discovery events
    ping: ping::Behaviour,
    keep_alive: keep_alive::Behaviour,
    kad: kad::Kademlia<MemoryStore>,
}

pub async fn run(args: &NodeCliArgs) -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let node_name = &args.node;
    info!("{}  ~ Running on {node_name}", config::E_ROCK.clone());
    // Let us generate crypto secure keys
    let key_pair = identity::Keypair::generate_ed25519();
    let pub_key = key_pair.public();
    let peer_id = PeerId::from_public_key(&pub_key);
    info!(
        "{}  ~ {} Id: {}",
        config::E_ID.clone(),
        &node_name,
        &peer_id
    );

    // Set up an encrypted DNS-enabled TCP Transport over the yamux protocol.
    let tcp_transport = tcp::async_io::Transport::new(tcp::Config::default().nodelay(true))
        .upgrade(upgrade::Version::V1Lazy)
        .authenticate(noise::Config::new(&key_pair)?)
        .multiplex(yamux::Config::default())
        .timeout(std::time::Duration::from_secs(20))
        .boxed();

    let quic_transport = quic::async_std::Transport::new(quic::Config::new(&key_pair));

    let transport = OrTransport::new(quic_transport, tcp_transport)
        .map(|either_output, _| match either_output {
            Either::Left((peer_id, muxer)) => (peer_id, StreamMuxerBox::new(muxer)),
            Either::Right((peer_id, muxer)) => (peer_id, StreamMuxerBox::new(muxer)),
        })
        .boxed();

    // To content-address message, we can take the hash of message and use it as an ID.
    let message_id_fn = |message: &gossipsub::Message| {
        let mut s = DefaultHasher::new();
        message.data.hash(&mut s);
        gossipsub::MessageId::from(s.finish().to_string())
    };

    // Set a custom gossipsub configuration
    let gossipsub_config = gossipsub::ConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(config::HEARTBEAT_INTERVAL)) // This is set to aid debugging by not cluttering the log space
        .duplicate_cache_time(Duration::from_millis(100))
        .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
        .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
        .build()?;

    // build a gossipsub network behaviour
    let mut gossipsub = gossipsub::Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(key_pair.clone()),
        gossipsub_config,
    )?;
    // Create a Gossipsub topic
    let topic = gossipsub::IdentTopic::new(PUBSUB_TOPIC);
    // subscribes to our topic
    gossipsub.subscribe(&topic)?;

    // Create a Swarm to manage peers and events
    let mut swarm = {
        //let mdns = mdns::async_io::Behaviour::new(mdns::Config::default(), peer_id)?;
        let mut mdns_config = libp2p::mdns::Config::default();
        mdns_config.query_interval = Duration::from_secs(15);
        mdns_config.ttl = Duration::from_secs(15);
        let mdns = libp2p::mdns::tokio::Behaviour::new(mdns_config, peer_id)?;
        let ping = ping::Behaviour::new(
            ping::Config::new().with_interval(Duration::from_secs(config::PING_INTERVAL)),
        );
        let identify = identify::Behaviour::new(identify::Config::new(
            config::STR_RENDEZVOUS_POINT.to_string(),
            key_pair.public(),
        ));
        let mut cfg = KademliaConfig::default();
        cfg.set_query_timeout(Duration::from_secs(5 * 60));
        let store = MemoryStore::new(peer_id);
        let kad = Kademlia::with_config(peer_id, store, cfg);

        let behaviour = PeerNetBehaviour {
            identify,
            gossipsub,
            mdns,
            ping,
            keep_alive: keep_alive::Behaviour,
            kad,
        };
        SwarmBuilder::with_tokio_executor(transport, behaviour, peer_id).build()
    };

    match args.mode {
        Mode::Server => {
            // Listen on all ipv4 interfaces and ephemeral ports
            swarm.listen_on(format!("/ip4/0.0.0.0/udp/{}/quic-v1", args.port).parse()?)?;
            swarm.listen_on(format!("/ip4/0.0.0.0/tcp/{}", args.port).parse()?)?;
        }
        Mode::Client => {
            if let Some(srv_addr) = &args.server_address {
                let mut prot_stack = srv_addr.protocol_stack();
                let ma: Multiaddr;
                _ = prot_stack.next();
                match prot_stack.next() {
                    Some(x) => match x {
                        "udp" => {
                            ma = srv_addr.clone().with(Protocol::QuicV1);
                            //ma = srv_addr.clone();
                        }
                        _ => ma = srv_addr.clone(),
                    },
                    _ => {
                        panic!("failed to format server address")
                    }
                }

                swarm.dial(ma).unwrap();
            }
        }
    }
    let num = rand::thread_rng().gen_range(5..10);
    // let mut tcr = futures_ticker::Ticker::new_with_next(Duration::from_secs(num), Duration::from_secs(10)).fuse();

    let mut tcr = tokio::time::interval(Duration::from_secs(num));
    // Infinite loop
    loop {
        // ---> SELECT EVENT
        tokio::select! {
            // Handle user input - take input as a txt message and publish to the topic
            _ = tcr.tick() => {
                let num_words = rand::thread_rng().gen_range(3..15);
                let t = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
                let payload = format!("[{t}] - {}", lipsum(num_words));
                if let Err(e) = swarm
                    .behaviour_mut().gossipsub
                    .publish(
                        topic.clone(),
                        payload.as_bytes())
                {
                    error!("{}  ~ <PUBLISH> error: {e:?}", &config::E_WARN.clone());
                }
            },
            // Handle swarm events
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(
                    PeerNetBehaviourEvent::Ping(
                        ping::Event {
                            peer,
                            ..
                })) if peer != peer_id => {
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
                        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
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
                        swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
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
                ) => info!(
                        "{}  ~ <MESSAGE>: '{}' with id: {id} from peer: {peer_id}",
                        config::E_EVT.clone(),
                        String::from_utf8_lossy(&message.data),
                 ),
                // New address found event swarm will listen on
                SwarmEvent::NewListenAddr { address, .. } => {
                    info!("{}  ~ <NET> [{}]:{} is listening on {address}",
                     config::E_INTR.clone(),
                     &node_name,
                     &peer_id
                 );
                },
                SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
                 log::info!("{}  ~ <NET> Connected to {}:{}", config::E_PLUG.clone(), peer_id, endpoint.get_remote_address());
                 },
                 SwarmEvent::ConnectionClosed { peer_id, endpoint, .. } => {
                     log::info!("{}  ~ <NET> Disconnected from {peer_id}:{}", config::E_PLUG.clone(), endpoint.get_remote_address());
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
                 SwarmEvent::IncomingConnectionError {
                    local_addr, send_back_addr, error
                 } => {
                    error!(
                        "{}  ~ <NET> Connection {} -> {} error: {error}",
                        config::E_ERR.clone(),
                        send_back_addr,
                        local_addr,
                        error = error
                    );
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
                 other_event => {
                    warn!("{}  ~ <UNHANDLED> {:#?}", config::E_EVT.clone(), other_event);
                }
            }
        }
    }
}
