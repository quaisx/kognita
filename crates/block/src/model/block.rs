/// Block message structure
/// Field	Description	Size
/// Magic no	Data field indicating that this data packet contains a BitcoinSV block. Value always 0xD9B4BEF9	4 bytes
/// Block size	number of bytes remaining in the packet up to end of block	4 bytes
/// Block header	consists of 6 items	80 bytes
/// Transaction counter	positive integer VI = VarInt	1 - 9 bytes
/// Transactions	the (non empty) list of transactions	<Transaction counter>-many transactions
/// Source ![Block message structure](<https://wiki.bitcoinsv.io/index.php/Block#Block_message_structure>)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(bytes = "vec", tag = "1")]
    pub family: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub padding: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub header: ::core::option::Option<Header>,
    #[prost(bytes = "vec", tag = "4")]
    pub tx_count: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "6")]
    pub txs: ::prost::alloc::vec::Vec<super::tx::Transaction>,
}
/// Block Header
/// Field	Purpose	Updated when...	Size (Bytes)
/// Version	Block version		4
/// hashPrevBlock	256-bit hash of the previous block header	A new block comes in	32
/// hashMerkleRoot	256-bit hash based on all of the transactions in the block	An updated merkle tree is completed	32
/// Time	Current block timestamp as seconds since 1970-01-01T00:00 UTC	Every few seconds	4
/// Bits	Current target in compact format	The difficulty is adjusted (approx 2 weeks)	4
/// Nonce	32-bit number (starts at 0)	A hash is tried (increments)	4
/// Source ![Block header](<https://wiki.bitcoinsv.io/index.php/Block_header>)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    #[prost(bytes = "vec", tag = "1")]
    pub version: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub hash_prev_block: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// Block header time stamp !\[timestamp\](<https://wiki.bitcoinsv.io/index.php/Block_timestamp>)
    #[prost(bytes = "vec", tag = "4")]
    pub timestamp: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub difficulty: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "6")]
    pub nonce: ::prost::alloc::vec::Vec<u8>,
}
