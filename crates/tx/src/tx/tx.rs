use serde::{Serialize, Deserialize};
use std::fmt;

const CURRENT_TX_VERSION: u32 = 2;

trait Marshalling {
   fn marshal(&self) -> string;
   fn unmarshal(&mut self, json: &string);
} 

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Txin {
    pub prev_hash: [u8; 32],
    pub prev_idx_out: u64,
    pub script_length: Vec<u8>,
    pub script: Vec<u8>,
    pub sequence_no: [u8; 4],
}
impl Default for Txin {
   fn default() -> Txin {
      Self {
         prev_hash = [0; 32],
         prev_idx_out = 0,
         script_length = Vec::new(),
         script = Vec::new(),
         sequence_no = 0xFFFFFFFF,
      }
   }
}
impl Display for Txin {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, 
         "<Txin> prev_hash:{:#032x} prev_idx_out:{} script_length:{:#?} script:{:#?} sequence_no:{}",
         self.prev_hash, self.prev_idx_out, self.script_length, self.script, self.sequence_no);
   }
}
impl Marshalling for Txin {
   fn marshal(&self) -> string {
      let json = serde_json::to_string(&self).unwrap();
      json
   }
   fn unmarshal(&mut self, json: &String) {
      let other = serde_json::from_str(&json).unwrap();
      self.prev_hash = other.prev_hash.clone();
      self.prev_idx = other.prev_idx;
      self.script_length = other.script_length.clone();
      self.script = other.script.clone();
      self.sequence_no = other.sequence_no.clone();
   }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Txout {
    pub neutrinos: u64,
    pub script_length: Vec<u8>,
    pub script: Vec<u8>,
}
impl Display for Txout {
   fn default() -> Self {
      Self {
         neutrinos = 0,
         script_length = Vec::new(),
         script = Vec::new(),
      }
   }
}
impl Display for Txout {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, 
         "<Txout> neutrinos:{} script_length:{:#?} script:{:#?}", 
         self.neutrinos, self.script_length, self.script)
   }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub version: u32,
    pub txs_in_len: u64,
    pub txs_in: Vec<Txin>,
    pub txs_out_len: u64,
    pub txs_out: Vec<Txout>,
    pub nlocktime: [u8; 4],
}
impl Default for Transaction {
   fn default() -> Self {
      Self {
         version: CURRENT_TX_VERSION,
         txs_in_len: 0,
         txs_in: Vec::new(),
         txs_out_len: 0,
         txs_out: Vec::new(),
         nlocktime: [0xFF; 4]
      }
   }
}
impl Display for Transaction {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, 
         "<Transaction> version:{}, txs_in_len:{}, txs_in:{:#?}, txs_out_len:{}, txs_out:{:#?}, nlocktime={:#04}",
         self.version, self.txs_in_len, self.txs_in, self.txs_out_len, self.txs_out, self.nlocktime)
   }
}
