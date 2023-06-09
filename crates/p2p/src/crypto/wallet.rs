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

use crate::utils;

use anyhow::{bail, Result};
use secp256k1::{
    rand::{rngs, SeedableRng},
    PublicKey, SecretKey,
};
use serde::{Deserialize, Serialize};
use std::io::BufWriter;
use std::str::FromStr;
use std::{fs::OpenOptions, io::BufReader};
use tiny_keccak::{Hasher, Sha3}; // hashing funcs
use web3::types::Address; // H160 -> 20 byte long wallet address

/// ---------------------------------------------------------------------------
/// pub_key_addr: takes a public key and returns an H160(20) address
///
/// Arguments:
///
///     &PublicKey - public key to use when generating a wallet address
///
/// Returns:
///
///     Address - an H160(20) keccak256 hash
/// ---------------------------------------------------------------------------
pub fn pub_key_addr(pub_key: &PublicKey) -> Address {
    let pub_key = pub_key.serialize_uncompressed();

    debug_assert_eq!(pub_key[0], 0x04);

    let mut sha3 = Sha3::v256();
    let mut hash = [0u8; 32];
    sha3.update(&pub_key[1..]);
    sha3.finalize(&mut hash);

    Address::from_slice(&hash[12..])
}

/// ---------------------------------------------------------------------------
/// gen_keypair - generate a Secp256k1 key pair (to be compatible with web3 types)
///
/// Returns:
///
///     SecretKey, PublicKey - a Secp256k1 key pair generated by secp256k1 mod
///
/// ---------------------------------------------------------------------------     
pub fn gen_keypair() -> (SecretKey, PublicKey) {
    // prepare a new Scp256k1 context
    let secp = secp256k1::Secp256k1::new();
    // use a true number generator based on CPU jitter (inuque for each chip)
    let mut rng = rngs::StdRng::seed_from_u64(utils::gen_seed());
    // now that we have a real random seed, generate a key pair
    secp.generate_keypair(&mut rng)
}

/// Etherium compatable wallet
#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    pub secret_key: String,
    pub public_key: String,
    pub public_address: String,
}
impl Wallet {
    /// new - generate a new Eth compatable wallet
    pub fn new(secret_key: &SecretKey, public_key: &PublicKey) -> Self {
        let addr: Address = pub_key_addr(&public_key);
        Wallet {
            secret_key: secret_key.display_secret().to_string(),
            public_key: public_key.to_string(),
            public_address: format!("{:?}", addr),
        }
    }
    /// load_wallet - loads a wallet from file
    pub fn load_wallet(file_path: &str) -> Result<Wallet> {
        let file = OpenOptions::new().read(true).open(file_path)?;
        let buf_reader = BufReader::new(file);

        let wallet: Wallet = serde_json::from_reader(buf_reader)?;
        Ok(wallet)
    }
    /// save_wallet - saves a wallet to a file   
    pub fn save_wallet(&self, file_path: &str) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;
        let buf_writer = BufWriter::new(file);

        serde_json::to_writer_pretty(buf_writer, self)?;

        Ok(())
    }
    /// get_secret_key - returns a private/secret key associated with this wallet
    pub fn get_secret_key(&self) -> Result<SecretKey> {
        let secret_key = SecretKey::from_str(&self.secret_key)?;
        Ok(secret_key)
    }
    /// get_public_key - returns a public key associated with this wallet
    pub fn get_public_key(&self) -> Result<PublicKey> {
        let pub_key = PublicKey::from_str(&self.public_key)?;
        Ok(pub_key)
    }
}
