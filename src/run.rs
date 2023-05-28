
#![allow(deprecated)]
#![allow(unused_imports)]

use futures::{future::Either, prelude::*, select, StreamExt};
use libp2p::{
    core::{muxing::StreamMuxerBox, transport::OrTransport, upgrade},
    gossipsub, identity, mdns, noise, ping,
    swarm::NetworkBehaviour,
    swarm::{keep_alive, SwarmBuilder, SwarmEvent},
    tcp, tcp::TokioTcpTransport, yamux, quic, PeerId, Transport
};
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use std::time::{SystemTime, SystemTimeError};
use rand::Rng;
use emojis;
use futures_ticker;
use lipsum::lipsum;
use log::{debug, error, info, trace, warn};
use std::env;

extern crate pretty_env_logger;


const HEARTBEAT_INTERVAL: u64 = 15; // gossibsub hb interval in seconds
const PUBSUB_TOPIC: &str = "kognita/tx";

// Custom Swarm Network behaviors
#[derive(NetworkBehaviour)]
struct PeerNetBehaviour {
    gossipsub: gossipsub::Behaviour, // to handle pubsub events
    mdns: mdns::async_io::Behaviour, // to handle mDSN discovery events
    ping: ping::Behaviour,
    keep_alive: keep_alive::Behaviour
}

pub async fn run() -> Result<(), Box<dyn Error>> {
   let e_rocket = emojis::get_by_shortcode("rocket").unwrap();

   let args: Vec<String> = env::args().collect();
   if args.len() < 2 {
       println!("Usage: {} <node-name>", args[0]);
       return Ok(());
   }
   pretty_env_logger::init();

   let node_name = &args[1];
   info!("{e_rocket}  ~ Running on {node_name}");
   // Let us generate crypto secure keys
   let key_pair = identity::Keypair::generate_ed25519();
   let pub_key = key_pair.public();
   let peer_id = PeerId::from_public_key(&pub_key);
   let e = emojis::get_by_shortcode("identification_card").unwrap();
   let e_warn = emojis::get_by_shortcode("warning").unwrap();
   let e_ping = emojis::get_by_shortcode("ping_pong").unwrap();
   let e_intr = emojis::get_by_shortcode("left_right_arrow").unwrap();
   let e_disc = emojis::get_by_shortcode("globe_with_meridians").unwrap();
   let e_event = emojis::get_by_shortcode("incoming_envelope").unwrap();
   info!("{e} PEER ID: {peer_id}");


   // Set up an encrypted DNS-enabled TCP Transport over the yamux protocol.
   let tcp_transport = tcp::async_io::Transport::new(tcp::Config::default().nodelay(true))
       .upgrade(upgrade::Version::V1Lazy)
       .authenticate(noise::Config::new(&key_pair)?)
       .multiplex(yamux::Config::default())
       .timeout(std::time::Duration::from_secs(20))
       .boxed();
   
   let quic_transport = quic::async_std::Transport::new(
       quic::Config::new(&key_pair)
   );

   let transport = OrTransport::new(
       quic_transport, 
       tcp_transport)
       .map(|either_output, _| match either_output {
           Either::Left(
               (peer_id, muxer)
           ) => (peer_id, StreamMuxerBox::new(muxer)),
           Either::Right(
               (peer_id, muxer)
           ) => (peer_id, StreamMuxerBox::new(muxer)),
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
       .heartbeat_interval(Duration::from_secs(HEARTBEAT_INTERVAL)) // This is set to aid debugging by not cluttering the log space
       .duplicate_cache_time(Duration::from_millis(100))
       .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
       .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
       .build()?;

   // build a gossipsub network behaviour
   let mut gossipsub = gossipsub::Behaviour::new(
       gossipsub::MessageAuthenticity::Signed(key_pair),
       gossipsub_config,
   )?;
   // Create a Gossipsub topic
   let topic = gossipsub::IdentTopic::new(PUBSUB_TOPIC);
   // subscribes to our topic
   gossipsub.subscribe(&topic)?;

   // Create a Swarm to manage peers and events
   let mut swarm = {
       let mdns = mdns::async_io::Behaviour::new(mdns::Config::default(), peer_id)?;
       let ping = ping::Behaviour::new(ping::Config::new().with_interval(Duration::from_secs(30)));
       let behaviour = PeerNetBehaviour { gossipsub, mdns, ping, keep_alive: keep_alive::Behaviour };
       SwarmBuilder::with_async_std_executor(transport, behaviour, peer_id).build()
   };

   // Listen on all ipv4 interfaces and ephemeral ports
   swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
   swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

   let num = rand::thread_rng().gen_range(1..10);
   let mut tcr = futures_ticker::Ticker::new_with_next(Duration::from_secs(num), Duration::from_secs(10)).fuse();
   // Infinite loop
   loop {
       // ---> SELECT EVENT
       select! {
           // Handle user input - take input as a txt message and publish to the topic
           _ = tcr.select_next_some() => {
               let num_words = rand::thread_rng().gen_range(3..15);
               let t = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
               let payload = format!("[{t}] - {}", lipsum(num_words));
               if let Err(e) = swarm
                   .behaviour_mut().gossipsub
                   .publish(
                       topic.clone(), 
                       payload.as_bytes()) 
               {
                   error!("{e_warn}  ~ <PUBLISH> error: {e:?}");
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
                   info!("{e_ping}  ~ <PING> to {} ", peer);
               },
               // mDNS discovery event
               SwarmEvent::Behaviour(
                   PeerNetBehaviourEvent::Mdns(
                       mdns::Event::Discovered(list)
                   )
               ) => {
                   for (peer_id, _multiaddr) in list {
                       info!("{e_disc}  ~ <mDNS> discovered a new peer: {peer_id}");
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
                       warn!("{e_warn}  ~ <mDNS> discover peer has expired: {peer_id}");
                       swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                   }
               },
               //PeerNetBehaviourEvent: Subscribed { peer_id: PeerId("12D3KooWNNS1ijqqnjLEhfS5q5uJw4K7BhjAY9o9A6v4727vREd5"), topic: TopicHash { hash: "kognita/tx" } })
               SwarmEvent::Behaviour(
                  PeerNetBehaviourEvent::Gossipsub(
                     gossipsub::Event::Subscribed { peer_id, topic }
                  )
               ) => info!(
                  "{e_event} ~ <SUBSCRIBE> {peer_id} subscribed to {topic}"
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
                       "{e_event}  ~ <MESSAGE>: '{}' with id: {id} from peer: {peer_id}",
                       String::from_utf8_lossy(&message.data),
                   ),
               // New address found event swarm will listen on
               SwarmEvent::NewListenAddr { address, .. } => {
                   info!("{e_intr}  ~ <NET> Local node is listening on {address}");
               },
               other_event => {
                   warn!("{e_warn} ~ <UNHANDLED> {:?}", other_event);
               }

           }
       }
   }
}