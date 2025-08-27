use std::{path::PathBuf};
use clap::{Args,Parser, Subcommand};


#[derive(Parser,Debug)]
#[command(name = "todo")]
pub struct Cli {
    #[command(subcommand)]
    pub command:Command,
}


/// struct with the sole purpose of handling default behavior  
/// for inputs
#[derive(Args,Debug)]
pub struct LowArgsConfig {
    #[arg(short)]
    pub long: bool,
    #[arg(short)]
    pub path_priority: bool,
    #[arg(long,default_value=None)]
    pub variable : Option<String>,
    #[arg(long,default_value=None)]
    pub path: Option<PathBuf>,
}


#[derive(Debug,Subcommand)]
pub enum Command {
    Ls(LowArgsConfig),
    Open(LowArgsConfig)
}





impl LowArgsConfig {
    pub fn build_ls_config(self)->LsConfig {
        LsConfig {
            detail:self.long,
            path_priority:self.path_priority,
            variable: self.variable,
            path: self.path
        }
    }
    
    pub fn build_open_config(self)->OpenConfig {
        OpenConfig {
            variable: self.variable,
            path: self.path
        }
    }

}

//------------------POD-------------------//
pub struct OpenConfig {
    pub variable: Option<String>,
    pub path: Option<PathBuf>
}

pub struct LsConfig{
    pub detail:bool,
    pub path_priority:bool,
    pub variable:Option<String>,
    pub path:Option<PathBuf>
}



#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_ls_args(){
        let arg = LowArgsConfig {
            long: false,
            path_priority: false,
            path: Some(PathBuf::from("./src/lib.rs")),
            variable:None
       };
       let config = arg.build_ls_config();

       assert_eq!(false, config.detail);
       assert_eq!(false,config.path_priority);
       assert_eq!(None, config.variable);
    }

    #[test]
    pub fn test_open_args(){
       let arg = LowArgsConfig {long = variable: Some("apple".to_string()), path: None};
       let config = arg.build_Open_config();

       assert_eq!(false, config.detail);
       assert_eq!(false,config.path_priority);
       assert_eq!(Some("apple"), config.var.as_deref());
    }

}






