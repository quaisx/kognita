use super::super::model::tx::{Transaction, Txin, Txout};

#[derive(Default)]
pub struct TxBuilder {
    version: u8,
    txs_in_len: usize,
    txs_in: Vec<Txin>,
    txs_out_len: usize,
    txs_out: Vec<Txout>,
    //if non-zero and sequence numbers are < 0xFFFFFFFF: block height or timestamp when transaction is final    
    ulock: u32,
}

impl TxBuilder {
    pub fn new() -> TxBuilder {
        TxBuilder {
            version: 0,
            txs_in_len: 0,
            txs_in: Vec::new(),
            txs_out_len: 0,
            txs_out: Vec::new(),
            ulock: 0,
        }
    }

    pub fn set_version(mut self, v: u8) -> TxBuilder {
        self.version = v;
        self
    }

    pub fn set_txs_in(mut self, txs: Vec<Txin>) -> TxBuilder {
        self.txs_in_len = txs.len();
        self.txs_in = txs;
        self
    }

    pub fn set_txs_out(mut self, txs: Vec<Txout>) -> TxBuilder {
        self.txs_out_len = txs.len();
        self.txs_out = txs;
        self
    }

    pub fn set_ulock(mut self, ul: u32) -> TxBuilder {
        self.ulock = ul;
        self
    }

    pub fn build(self) -> Transaction {
        Transaction { 
            version: Vec::new(), 
            txs_in_len: Vec::new(), 
            txs_in: Vec::new(), 
            txs_out_len: Vec::new(), 
            txs_out: Vec::new(), 
            nlock: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_test() {
        let tx = Transaction { 
            version: Vec::new(), 
            txs_in_len: Vec::new(), 
            txs_in: Vec::new(), 
            txs_out_len: Vec::new(), 
            txs_out: Vec::new(), 
            nlock: Vec::new() };
        let tx_from_builder: Transaction = TxBuilder::new().set_version(0).build();
        assert_eq!(tx, tx_from_builder);
    }
}