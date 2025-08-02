use cli_interface::config::Cli;
use clap::Parser;
/* command to implement:
 * ls { -p(to seperate by path), -v(to seperate by var) }
 * "var" creates symlink and opens it with neovim
 *
 * */
fn main() {
    env_logger::init();

    match cli_interface::run(Cli::parse()) {
        Ok(ok)=> ok,
        Err(e)=>  eprintln!("{}",e),
    }
}



