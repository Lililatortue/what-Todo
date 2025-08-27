pub mod utils;
mod startup;
mod fs_management;
mod cli;
mod navigation;
mod action;
mod parser;
mod pod;

use crate::cli::{config::Cli};
use clap::Parser;

/* command to implement:
 * ls { -p(to seperate by path), -v(to seperate by var) }
 * "var" creates symlink and opens it with neovim
 *
 * */
fn main() {
    env_logger::init();

    match cli::run(Cli::parse()) {
        Ok(ok)=> ok,
        Err(e)=>  eprintln!("{}",e),
    }
}



