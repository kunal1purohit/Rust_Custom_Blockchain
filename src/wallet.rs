use bitcoincash_addr::{Address , HashType , Scheme};
use crypto::digest::Digest;
use crypto::ed25519;
use crypto::ripemd160::Ripemd160;
use crypto::sha2::Sha256;
use rand::RngCore;
use rand::rngs::OsRng;
use serde::{Serialize,Deserialize};
use std::collections::HashMap;
use log::info;


#[derive(Serialize,Deserialize,Debug,Clone, PartialEq)]
pub struct Wallet{
    pub secret_key:Vec<u8>,
    pub public_key:Vec<u8>
}

impl Wallet{
    fn new()->Self{
        let mut key:[u8;32] = [0;32];
        OsRng.fill_bytes(&mut key);
        let (secret_key,public_key) = ed25519::keypair(&key);
        let secret_key=secret_key.to_vec();
        let public_key=public_key.to_vec();
        Wallet{
            secret_key,
            public_key
        }
    }

    fn get_address(&self)->String{
        let mut pub_hash = self.public_key.clone();
        pub_hash_key(&mut pub_hash);
        let address = Address{
            body:pub_hash,
            scheme:Scheme::Base58,
            hash_type:HashType::Script,
            ..Default::default()
        };
        address.encode().unwrap()
    }
}

pub fn pub_hash_key(pub_key:&mut Vec<u8>){
    // let mut hasher1 = Sha256::new();
    // hasher1.input(pub_key);
    // hasher1.result(pub_key);
    // let mut hasher2 = Sha256::new();
    // hasher2.input(pub_key);
    // pub_key.resize(20,0);
    // hasher2.result(pub_key);





 // SHA256 hashing
 let mut hasher1 = Sha256::new();
 hasher1.input(pub_key);
 hasher1.result(pub_key);

 // RIPEMD160 hashing
 let mut hasher2 = Ripemd160::new();
 hasher2.input(pub_key);
 pub_key.resize(20, 0);
 hasher2.result(pub_key);




    // // Create a buffer for the first SHA256 hash (32 bytes)
    // let mut hasher1 = Sha256::new();
    // let mut sha256_hash = vec![0u8; 32];
    // hasher1.input(pub_key);
    // hasher1.result(&mut sha256_hash);

    // // Hash the result of the first hash
    // let mut hasher2 = Sha256::new();
    // hasher2.input(&sha256_hash);

    // // Resize pub_key to 32 bytes temporarily to hold the second hash
    // let mut final_hash = vec![0u8; 32];
    // hasher2.result(&mut final_hash);

    // // Only take the first 20 bytes (RIPEMD160-like truncation)
    // pub_key.clear();
    // pub_key.extend_from_slice(&final_hash[..20]);  // Only keep the first 20 bytes
}

pub struct Wallets{
    wallets:HashMap<String,Wallet>
}

use crate::error::Result;
impl Wallets{
    pub fn new()->Result<Wallets>{
        let mut wlt=Wallets{
            wallets:HashMap::<String,Wallet>::new()
        };
        let db =sled::open("data/wallets")?;
        for item in db.into_iter(){
            let i=item?;
            let address = String::from_utf8(i.0.to_vec())?;
            let wallet = bincode::deserialize(&i.1.to_vec())?;
            wlt.wallets.insert(address,wallet);
        }
        drop(db);
        Ok(wlt)
    }

    pub fn create_wallet(&mut self) -> String{
        let wallet = Wallet::new();
        let address = wallet.get_address();
        self.wallets.insert(address.clone(),wallet);
        info!("create wallet: {}" , address);
        address
    }

    pub fn get_all_addresses(&self) -> Vec<String>{
        let mut addresses = Vec::new();
        for (address,_) in &self.wallets{
            addresses.push(address.clone());
        }
        addresses
    }

    pub fn get_wallet(&self,address:&str) -> Option<&Wallet>{
        self.wallets.get(address)
    }

    pub fn save_all(&self)->Result<()>{
        let db=sled::open("data/wallets")?;
        for(address , wallet) in &self.wallets{
            let data = bincode::serialize(wallet)?;
            db.insert(address,data)?;
        }
        db.flush()?;
        drop(db);
        Ok(())
    }
}


