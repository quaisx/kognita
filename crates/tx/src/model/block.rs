#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::header::Header>,
    #[prost(message, repeated, tag = "2")]
    pub txs: ::prost::alloc::vec::Vec<super::tx::Transaction>,
}
