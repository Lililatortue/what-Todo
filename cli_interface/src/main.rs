mod cli;
mod navigation;
mod action;
mod parser;
mod pod;


use crate::cli::{lauch_check,config::Cli};
use clap::Parser;

/* command to implement:
 * ls { -p(to seperate by path), -v(to seperate by var) }
 * "var" creates symlink and opens it with neovim
 *
 * */
fn main() {
    env_logger::init();
    cli::lauch_check::init();

    match cli::run(Cli::parse()) {
        Ok(ok)=> ok,
        Err(e)=>  eprintln!("{}",e),
    }
}



