use crate::cli::Cli;
mod block;
mod error;
mod blockchain;
mod cli;
use crate::error::Result;

fn main() ->Result<()>{
    println!("Hello, world!");

    let mut cli = Cli::new()?;
    cli.run()?;
    Ok(())
}