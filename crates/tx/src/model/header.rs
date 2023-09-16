/// Kognita Block header
/// prev_hash - previous block merkel root hash
/// nonce - a block is found when a miner successfully discovers a value that generates a hash less than the difficulty target
/// root_hash - tx merkel tree root hash
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    #[prost(bytes = "vec", tag = "1")]
    pub prev_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub nonce: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub root_hash: ::prost::alloc::vec::Vec<u8>,
}
