# History of changes

May 25 - add the first commit
Specifics: a basic node that supports mDNS discovery and does gossip pubsub exchange of randomly generated text.

May 26 - add some utility functions for random seeding and a crypto wallet.

May 26 - start using tokio async; gradual switch;
    - update wallet to use updated crate functions for key generation

May 27 - replace asyc_std with tokio

May 28 - add rendezvous discovery; use mdns tokio for mdns discovery;
    introduce clap cli argument parsing; reorganize into modules;

May 29 - cli args with clap;

May 30 - add Kademlia DHT support;

Jun 4 - add handling of incoming connection errors; set handshake timeout to allow for enough time to handle connection negotiations;

Jun 5 - replace const values used in net and peer layers with defaults and config file node.toml; the app uses the defaults/config values from now on.

Jun 9 - add simple gRPC service to inject messages instead of generating them randomly; use tonic crate to generate gRPC artifacts;
