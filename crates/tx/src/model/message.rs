/// Kognita Transaction In
/// Field	Description	Size
/// Previous Transaction hash	doubled SHA256-hashed of a (previous) to-be-used transaction	32 bytes
/// Previous Txout-index	non negative integer indexing an output of the to-be-used transaction	4 bytes
/// Txin-script length	non negative integer
/// Txin-script / scriptSig	Script many bytes
/// sequence_no	normally 0xFFFFFFFF; irrelevant unless transaction's lock_time is > 0	4 bytes
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Txin {
    #[prost(bytes = "vec", tag = "1")]
    pub prev_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub prev_idx_out: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub script_length: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub script: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub sequence_no: ::prost::alloc::vec::Vec<u8>,
}
/// Kognita Transaction Out
/// Field	Description	Size
/// value	non negative integer giving the number of neutrino(NC/10^9) to be transfered	8 bytes
/// Txout-script length	non negative integer	1 - 9 bytes VI = VarInt
/// Txout-script / scriptPubKey	Script	<out-script length>-many bytes
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Txout {
    #[prost(bytes = "vec", tag = "1")]
    pub neutrinos: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub script_length: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub script: ::prost::alloc::vec::Vec<u8>,
}
/// Field	Description	Size
/// version no - currently 0 / do not anticipate more than 2 bytes
/// in_counter	positive integer giving the number of txs_in
/// list of inputs	txs_in qty with variable length per input
/// out_counter	positive integer giving the number of txs_out
/// list of outputs txs_out qty with variable length per output
/// nLocktime	if non-zero and sequence numbers are < 0xFFFFFFFF: block height or timestamp when transaction is final	4 bytes
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(bytes = "vec", tag = "1")]
    pub version: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub counter_in: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "3")]
    pub txs_in: ::prost::alloc::vec::Vec<Txin>,
    #[prost(bytes = "vec", tag = "4")]
    pub counter_out: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "5")]
    pub txs_out: ::prost::alloc::vec::Vec<Txout>,
    #[prost(bytes = "vec", tag = "6")]
    pub nlock: ::prost::alloc::vec::Vec<u8>,
}
