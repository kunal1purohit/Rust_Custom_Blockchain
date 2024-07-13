use crate::blockchain::Blockchain;
use crate::error::*;
use clap::Command; 
use clap::arg;

pub struct Cli{
    bc:Blockchain,
}

impl Cli{
    pub fn new()->Result<Cli>{
        Ok(Cli{
            bc :Blockchain::new()?,
        })
    }

    pub fn run(&mut self)->Result<()>{
        let matches = Command::new("blockchain rust demo")
        .version("0.1")
        .author("kunal1purohit@gmail.com")
        .about("blockchain in rust :: for self learning")
        .subcommand(Command::new("printchain").about("print all blocks"))
        .subcommand(Command::new("addblock").about("add a block").arg(arg!(<DATA>"'the blockchain data'"))
        ).get_matches();

        if let Some(ref matches) = matches.subcommand_matches("addblock"){
            if let Some(c) = matches.get_one::<String>("DATA"){
                self.addblock(String::from(c))?;
            }else{
                println!("not printing testing lists");
            }
        }

        if let Some(_) = matches.subcommand_matches("printchain"){
            self.printchain();
        }

        Ok(())
    }

    fn addblock(&mut self, data: String) -> Result<()>{
        self.bc.add_block(data);
        Ok(())
    }

    fn printchain(&self) -> Result<()> {
        for item in self.bc.iter() {
            println!("block: {:?}", item);
        }
        Ok(())
    }

}
