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
     gossipsub,
     gossipsub::{ Topic, Sha256Topic } , identity, identify, mdns,
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
 use std::hash::{Hash, Hasher};
 use std::sync::Arc;
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
 
 extern crate pretty_env_logger;

// Custom Swarm Network behaviors
#[derive(NetworkBehaviour)]
pub struct PeerNetBehaviour {
   pub identify: identify::Behaviour,
   pub gossipsub: gossipsub::Behaviour,      // to handle pubsub events
   pub mdns: libp2p::mdns::tokio::Behaviour, // to handle mDSN discovery events
   pub ping: ping::Behaviour,
   pub keep_alive: keep_alive::Behaviour,
   pub kad: kad::Kademlia<MemoryStore>,
}

impl From<Box<PeerNetBehaviour>> for PeerNetBehaviour {
   fn from(b: Box<PeerNetBehaviour>) -> Self { 
      PeerNetBehaviour { 
         identify: (b.identify), 
         gossipsub: (b.gossipsub), 
         mdns: (b.mdns), 
         ping: (b.ping), 
         keep_alive: (b.keep_alive), 
         kad: (b.kad) 
      } 
   }

}
 pub struct P2pNode {
   pub node_name: String,
   pub peer_id: PeerId,
   pub node_config: NodeConfig,
   pub key_pair: libp2p::identity::Keypair,
   //pub behaviour: Option<Box<PeerNetBehaviour>>,
   pub topic: Option<Sha256Topic>,
 }

 impl P2pNode {

   pub fn new() -> Box<P2pNode> {
      Box::new(P2pNode {
         node_name: String::new(),
         peer_id: PeerId::random(),
         node_config: NodeConfig::new(),
         key_pair: libp2p::identity::Keypair::generate_ed25519(),
         //behaviour: None,
         topic: None,
      })
   }

   pub fn init(&mut self, args: &NodeCliArgs, node_config: Box<NodeConfig>)
            -> Result<Swarm<PeerNetBehaviour>, Box<dyn Error>> {
      self.node_name = args.node.clone();
      self.node_config = *node_config;

      // Let us generate crypto secure keys
      self.peer_id = PeerId::from_public_key(&self.key_pair.public());

      // Set up an encrypted DNS-enabled TCP Transport over the yamux protocol.
      let tcp_transport_config = tcp::Config::default()
         .nodelay(self.node_config.net().tcp_nodelay())
         .port_reuse(self.node_config.net().tcp_portreuse());
         let tcp_transport = tcp::async_io::Transport::new(tcp_transport_config)
         .upgrade(upgrade::Version::V1Lazy)
         .authenticate(noise::Config::new(&self.key_pair)?)
         .multiplex(yamux::Config::default())
         .timeout(Duration::from_secs(self.node_config.net().tcp_timeout()))
         .boxed();

      let mut quic_transport_config = quic::Config::new(&self.key_pair);
      quic_transport_config.handshake_timeout = Duration::from_secs(self.node_config.net().quic_handshake_timeout());
      let quic_transport = quic::async_std::Transport::new(
         quic_transport_config,
      );

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
      .heartbeat_interval(Duration::from_secs(self.node_config.peer().pubsub_heartbeat_interval())) // This is set to aid debugging by not cluttering the log space
      .duplicate_cache_time(Duration::from_millis(self.node_config.peer().pubsub_duplicate_cache_time()))
      .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
      .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
      .build()?;

      // build a gossipsub network behaviour
      let mut gossipsub = gossipsub::Behaviour::new(
         gossipsub::MessageAuthenticity::Signed(self.key_pair.clone()),
         gossipsub_config,
      )?;
      // // Create a Gossipsub topic
      // self.topic = Some(gossipsub::Sha256Topic::new(self.node_config.peer().pubsub_topic()));
      // // subscribes to our topic
      // gossipsub.subscribe(&self.topic.unwrap())?;

      // Create a Swarm to manage peers and events
      // _swarm = {
      //let mdns = mdns::async_io::Behaviour::new(mdns::Config::default(), peer_id)?;
      let mut mdns_config = libp2p::mdns::Config::default();
      mdns_config.query_interval = Duration::from_secs(self.node_config.peer().mdns_query_interval());
      mdns_config.ttl = Duration::from_secs(self.node_config.peer().mdns_ttl());
      let mdns = libp2p::mdns::tokio::Behaviour::new(mdns_config, self.peer_id)?;
      let ping = ping::Behaviour::new(
      ping::Config::new().with_interval(Duration::from_secs(self.node_config.peer().ping_interval())),
      );
      let identify = identify::Behaviour::new(
      identify::Config::new(
      self.node_config.peer().rendezvous_point().to_string(),
      self.key_pair.public(),
      ));
      let mut cfg = KademliaConfig::default();
      cfg.set_query_timeout(Duration::from_secs(self.node_config.peer().kad_query_timeout()));
      let store = MemoryStore::new(self.peer_id);
      let kad = Kademlia::with_config(self.peer_id, store, cfg);

      // self.behaviour = Some(
      //    Box::new(
      //       PeerNetBehaviour{
      //          identify,
      //          gossipsub,
      //          mdns,
      //          ping,
      //          keep_alive: keep_alive::Behaviour,
      //          kad,
      //       }
      //    )
      // );
      let behaviour = PeerNetBehaviour{
         identify,
         gossipsub,
         mdns,
         ping,
         keep_alive: keep_alive::Behaviour,
         kad,
      };
      Ok(
         SwarmBuilder::with_tokio_executor(
            transport, 
            behaviour,
            self.peer_id
         )
         .build()
      )
   }
}