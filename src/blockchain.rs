use crate::block::Block;
use crate::error::Result;
use log::info;
use crate::transaction::Transaction;

const TARGET_HEXT:usize =4;

#[derive(Debug,Clone)]
pub struct Blockchain{
    current_hash : String,
    db : sled::Db,
}

pub struct BlockchainIter<'a>{
    current_hash:String,
    bc: &'a Blockchain
}

impl Blockchain{
    pub fn new() -> Result<Blockchain>{
        info!("open blockchain");
        let db = sled::open("data/blocks")?;
        let hash=db.get("LAST")?.expect("must create a new blockchain database first");
        info!("found blockchain database");
        let lasthash = String::from_utf8(hash.to_vec())?;
        Ok(Blockchain { current_hash: lasthash.clone(), db })
    }

    pub fn create_blockchain(address:String)->Result<Blockchain>{
        info!("creating new blockchain");
        let db = sled::open("data/blocks")?;
        info!("creating new block database");
        let cbtx = Transaction::new_coinbase(address , String::from("GENESIS_COIN"))?;
        let genesis = Block::new_genesis_block(cbtx);
        db.insert(genesis.get_hash(), bincode::serialize(&genesis)?)?;
        db.insert("LAST", genesis.get_hash().as_bytes())?;
        let bc = Blockchain{
            current_hash:genesis.get_hash(),
            db,
        };
        bc.db.flush()?;
        Ok(bc)
    }

    pub fn add_block(&mut self, data:String)->Result<()>{
        let lasthash = self.db.get("LAST")?.unwrap();
        let new_block = Block::new_block(data,String::from_utf8(lasthash.to_vec())?,TARGET_HEXT)?;
        self.db.insert(new_block.get_hash(),bincode::serialize(&new_block)?)?;
        self.db.insert("LAST",new_block.get_hash().as_bytes())?;
        self.current_hash = new_block.get_hash();        
        Ok(())
    }

    pub fn iter(&self) -> BlockchainIter{
        BlockchainIter{
            current_hash:self.current_hash.clone(),
            bc:&self,
        }
    }
}

impl <'a> Iterator for BlockchainIter<'a>{
    type Item = Block;
    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(encode_block) = self.bc.db.get(&self.current_hash){
            return match encode_block{
                Some(b)=>{
                    if let Ok(block) = bincode::deserialize::<Block>(&b){
                        self.current_hash = block.get_prev_hash();
                        Some(block)
                    }
                    else{
                        None
                    }

                }
                None=>None
            }
        }
        None
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_add_block(){
        let mut b = Blockchain::new().unwrap();
        b.add_block("data".to_string());
        b.add_block("data2".to_string());
        b.add_block("data3".to_string());
        for item in b.iter() {
            println!("item {:?}",item)
        }
    }
}