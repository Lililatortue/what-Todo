mod cli;
mod parser;
mod configuration;

use cli::Cli;
use clap::Parser;

fn main()->Result<(),&'static str> {

    let workspace = configuration::check_workspace()
        .map_err(|err| err.to_string());
        
    cli::run(Cli::parse())?;
    Ok(())
}



