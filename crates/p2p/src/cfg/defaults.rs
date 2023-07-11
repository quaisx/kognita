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

 #![deny(warnings)]

pub const DEF_DESCRIPTION: &str = "Kognita node configuration file";
pub const DEF_VERSION: &str = "0.1";
pub const DEF_TCP_NODELAY: bool = true;
pub const DEF_TCP_PORTREUSE: bool = true;
pub const DEF_TCP_TIMEOUT: u64 = 10;
pub const DEF_QUIC_HANDSHAKE_TIMEOUT: u64 = 10;
pub const DEF_PUBSUB_HEARTBEAT_INTERVAL: u64 = 30;
pub const DEF_PUBSUB_DUPLICATE_CACHE_TIME: u64 = 1000;
pub const DEF_PUBSUB_TOPIC: &str = "kognita/tx";
pub const DEF_MDNS_QUERY_INTERVAL: u64 = 30;
pub const DEF_MDNS_TTL: u64 = 60;
pub const DEF_PING_INTERVAL: u64 = 30;
pub const DEF_KAD_QUERY_TIMEOUT: u64 = 300;
pub const DEF_RENDEZVOUS_POINT: &str = "kognita/1.0.0/12345";

