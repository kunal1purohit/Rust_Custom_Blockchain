use crate::blockchain::Blockchain;
use crate::error::*;
use crate::transaction::Transaction;
use bitcoincash_addr::Address;
use clap::Command; 
use clap::arg;
use std::process::exit;
use crate::wallet::Wallets;
use crate::utxoset::UTXOSet;

pub struct Cli{
    //bc:Blockchain,
}

impl Cli{
    pub fn new()->Result<Cli>{
        Ok(Cli{
           // bc :Blockchain::new()?,
        })
    }

    pub fn run(&mut self)->Result<()>{
        let matches = Command::new("blockchain rust demo")
        .version("0.1")
        .author("kunal1purohit@gmail.com")
        .about("blockchain in rust :: for self learning")
        .subcommand(Command::new("printchain").about("print all the blocks in chain"))
        .subcommand(Command::new("createwallet").about("create a wallet"))
        .subcommand(Command::new("listaddresses").about("list all addresses"))
        .subcommand(Command::new("reindex").about("reindex UTXO"))
        .subcommand(Command::new("getbalance")
    .about("get balance in the blockchain")
.arg(arg!(<ADDRESS>"'the address it gets balance for'")))
.subcommand(Command::new("create").about("create new blockchain")
.arg(arg!(<ADDRESS>"'The address to send genesis block reward to'")))
.subcommand(Command::new("send")
.about("send in the blockchain")
.arg(arg!(<FROM>"'source wallet address'"))
.arg(arg!(<TO>"'destination wallet address'"))
.arg(arg!(<AMOUNT>"'amount willing to send'")))
       // .subcommand(Command::new("addblock").about("add a block").arg(arg!(<DATA>"'the blockchain data'"))
        //)
        .get_matches();

        if let Some(ref matches) = matches.subcommand_matches("create"){
            if let Some(address) = matches.get_one::<String>("ADDRESS"){
                let address = String::from(address);
                let bc = Blockchain::create_blockchain(address.clone())?;
                let utxo_set = UTXOSet{blockchain:bc};
                utxo_set.reindex()?;
                println!("create blockchain")
            };
            // else{
            //     println!("not printing testing lists");
            // }
        }

        if let Some(ref matches) = matches.subcommand_matches("createwallet"){
            let mut ws = Wallets::new()?;
            let address = ws.create_wallet();
            ws.save_all()?;
            println!("success : address {}",address);
        }

        if let Some(_) = matches.subcommand_matches("reindex"){
            let bc = Blockchain::new()?;
            let utxo_set = UTXOSet{blockchain:bc};
            utxo_set.reindex()?;
            let count = utxo_set.count_transactions()?;
            println!("Done! there are {} transactions in the UTXO set.", count);
            
        }

        if let Some(ref matches) = matches.subcommand_matches("listaddresses"){
            let mut ws = Wallets::new()?;
            let addresses = ws.get_all_addresses();
            println!("addresses: ");
            for ad in addresses{
                println!("{}",ad);
            }
        }

        if let Some(ref matches) = matches.subcommand_matches("getbalance"){
            if let Some(address) = matches.get_one::<String>("ADDRESS"){
                // let address = String::from(address);
                // let bc=Blockchain::new()?;
                // let utxos = bc.find_UTXO(&address);
                let pub_key_hash = Address::decode(address).unwrap().body;
                let bc = Blockchain::new()?;
              //  let utxos = bc.find_UTXO(&pub_key_hash);
              let utxo_set = UTXOSet{blockchain:bc};
              let utxos = utxo_set.find_UTXO(&pub_key_hash)?;
                let mut balance=0;
                for out in utxos.outputs{
                    balance+=out.value;
                }
                
                println!("balance of {} is {}",address,balance)
            }
            // else{
            //     println!("not printing testing lists");
            // }
        }

        if let Some(ref matches) = matches.subcommand_matches("send"){
            let from = if let Some(address) = matches.get_one::<String>("FROM"){
                address
            }
            else{
                println!("from not supply!: usage");
                exit(1)
            };

            let to = if let Some(address) = matches.get_one::<String>("TO"){
                address
            }
            else{
                println!("from not supply!: usage");
                exit(1)
            };

            let amount: i32 = if let Some(amount) = matches.get_one::<String>("AMOUNT"){
                amount.parse()?
            }
            else{
                println!("from not supply!: usage");
                exit(1)
            };

            // let mut bc=Blockchain::new()?;
            // let tx = Transaction::new_UTXO(from, to, amount, &bc)?;
            // bc.add_block(vec![tx])?;

            let bc=Blockchain::new()?;
            let mut utxo_set = UTXOSet{blockchain:bc};
            let tx = Transaction::new_UTXO(from, to, amount, &utxo_set)?;
            let cbtx  = Transaction::new_coinbase(from.to_string() ,  String::from("reward!"))?;
            let new_block = utxo_set.blockchain.add_block(vec![cbtx,tx])?;

            utxo_set.update(&new_block);
            println!("success!");

        }

        if let Some(_) = matches.subcommand_matches("printchain"){
            let bc = Blockchain::new()?;
            for b in &mut bc.iter() {
                        println!("block: {:#?}", b);
                    }
        }



        Ok(())
    }

    // fn addblock(&mut self, data: String) -> Result<()>{
    //     self.bc.add_block(vec![]);
    //     Ok(())
    // }

    // fn printchain(&self) -> Result<()> {
    //     for item in self.bc.iter() {
    //         println!("block: {:?}", item);
    //     }
    //     Ok(())
    // }

}
