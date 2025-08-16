mod cli;
mod navigation;
mod action;
mod parser;
mod pod;
mod utilz;

use crate::cli::{launch_check,config::Cli};
use clap::Parser;

/* command to implement:
 * ls { -p(to seperate by path), -v(to seperate by var) }
 * "var" creates symlink and opens it with neovim
 *
 * */
fn main() {
    env_logger::init();
    launch_check::init()
        .unwrap_or_else(|e|{eprintln!("{e}"); std::process::exit(1)});

    match cli::run(Cli::parse()) {
        Ok(ok)=> ok,
        Err(e)=>  eprintln!("{}",e),
    }
}



