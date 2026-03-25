pub(crate) mod command;
use clap::{Parser, Subcommand};
use std::{path::PathBuf};

use crate::{Config, action::{ls::list_todo}, cli::command::OutputType};

pub(crate)struct Cmd {
    pub silent: bool,
    pub value : Option<String>,
    pub path: PathBuf,
    pub output: OutputType
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
   //Open(command::OpenCommand), deprecated
}


pub fn run(
    config: Config,
    cli: Cli
)->Result<(),&'static str> 
{ 
    match  cli.command {
        Command::List(cmd) => list_todo(cmd.into(),config),
        //Command::Open(cmd) => {let _ = open(cmd.into(),config);},deprecated
    };
    Ok(())
}






