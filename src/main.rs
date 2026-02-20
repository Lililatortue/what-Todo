mod cli;
mod parser;

//mod navigation;
//mod pod;

use crate::cli::Cli;
use clap::Parser;

fn main() {
    env_logger::init();

    match cli::run(Cli::parse()) {
        Ok(ok)=> ok,
        Err(e)=>  eprintln!("{}",e),
    }
}



