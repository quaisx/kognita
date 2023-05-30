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

May 3- add Kademlia DHT support;



