pub(crate) mod command;
use clap::{Parser, Subcommand};
use std::{path::PathBuf};



pub(crate)struct Config {
    pub silent: bool,
    pub value : Option<String>,
    pub path: PathBuf,
}


#[derive(Parser,Debug)]
#[command(name = "todo")]
pub struct Cli {
    #[command(subcommand)]
    pub command:Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    List(command::ListCommand),
    Open(command::OpenCommand),
}


pub fn run(cli: Cli)->Result<(),&'static str> { 
    let _ = match  cli.command {
        Command::List(cmd) => todo!(),//ls::list(cmd.build()),
        Command::Open(cmd) => todo!(),//open::open(cmd.build()),
    };
}






