use std::{env, path::PathBuf};
use clap::{Args,Parser, Subcommand};
/// struct with the sole purpose of handling default behavior  
/// for inputs
///
#[allow(dead_code)]
pub struct Config {
    pub detail: bool,
    pub path_priority: bool,
    pub var : Option<String>,
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
    // list all todos
    Ls(LsArgs),
    // open every file concerned with variable then opens it with editor
    Open(OpenArgs),

}

#[derive(Args,Debug)]
pub struct LsArgs {
        #[arg(short)]
        long: bool, //full details
        #[arg(short)]
        path_priority: bool, //sorts base on path, defaults to variables
        #[arg(default_value = ".")]
        path: PathBuf,//can check recursively if path provided defaults to current
}
use crate::config;


impl LsArgs {
    /// Checks if path is a directory or file,
    /// doesnt support symlink for now 
    /// since its binary nature dir makes the bool in config 
    /// be true and a file make it false the rest terminates the program
    pub fn build_config(self) -> config::Config {
        let (path, _is_dir); // for now no use for if dir or not, in futur will dictate strategy on
                             // how to parse
        if self.path.is_dir() {
            (path,_is_dir) = (self.path, true); 
        } else if self.path.is_file(){
            (path,_is_dir) = (self.path, false);
        } else {
            eprintln!(
                "Error: {} is a non supported file type selected. \
                Please make sure the path choosen is a file or a directory", 
                self.path.display()
            );
            std::process::exit(1);
        }
        config::Config {
            detail: self.long,
            path_priority: self.path_priority,
            path  : path,
            var: None,
        }
    }
}



#[derive(Args, Debug)]
pub struct OpenArgs {
    path: Option<PathBuf>,
    var : Option<String>,
}

impl OpenArgs {
    ///takes a var and a path this function doesnt check if var exist
    ///only if arguments are provided
    pub fn build_config(mut self)-> config::Config {
        let var = match self.var.take() {
            Some(s) => s,
            None    => {
                eprintln!(
                    "Error: lack of variable. Please add a variable. \
                    \n Expected Format: todo open <VAR> <Optional(PATH)>");
                std::process::exit(1);
            }
        };

        let path = match self.path.take() {
            Some(p) => p,
            None    => env::current_dir().expect("Error: Can't find current directory"),
        };

        config::Config {
            detail: false,
            path_priority: false,
            path: path,
            var : Some(var),
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_ls_args(){
       let arg = LsArgs {long: false, path_priority: false, path: PathBuf::from("./src/lib.rs")};
       let config = arg.build_config();

       assert_eq!(false, config.detail);
       assert_eq!(false,config.path_priority);
       assert_eq!(PathBuf::from("./src/lib.rs"),config.path);
       assert_eq!(None, config.var);
    }

    #[test]
    pub fn test_open_args(){
       let arg = OpenArgs {var: Some("apple".to_string()), path: None};
       let config = arg.build_config();

       assert_eq!(false, config.detail);
       assert_eq!(false,config.path_priority);
       assert_eq!(PathBuf::from("/home/lililatortue/Dev/Rust/projects/What-Todo/cli_interface"),config.path);
       assert_eq!(Some("apple"), config.var.as_deref());
    }

}






