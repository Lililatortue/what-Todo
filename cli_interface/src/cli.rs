pub mod config;

use crate::action::{ls,open};
use crate::cli::config::{Command,Cli};

pub fn run(cli: Cli)->Result<(),&'static str> {
    
    // Making sure project file system is well implemented 
    if let Err(e) = crate::fs_management::setup() {
        eprintln!("{e}");
        std::process::exit(1);
    }
    // inits lexer config
    let toml_config = crate::startup::init_graphs();
   


    match cli.command {
        Command::Ls(arg)=> {
            let ls_config = arg.build_ls_config();
            match ls::list_todo(ls_config,toml_config) {
                Ok(_)  => Ok(()),
                Err(e) => return Err(&format!("{}",e))
            }
        },
        //not supported yet working on it
        Command::Open(arg) => {
            let config = arg.build_open_config();

            match open::open_in_editor(config) {
                Ok(_) => Ok(()),
                Err(e)=> return Err(&format!("{}",e))

            }
        }, 
    };
    Ok(())
}


