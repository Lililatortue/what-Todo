use std::path::PathBuf;
use std::io::{Error,ErrorKind};
use crate::configuration::regex_config::RegexConfig;

//provides path and impl to modify workspace from cmd line
//
pub struct WorkSpaceConfig{
    pub root:PathBuf,
    pub virtual_env:PathBuf,
    pub config: String,
}

impl WorkSpaceConfig {
pub fn new()->Result<Self,&'static str>{
    WorkSpaceConfig::check_workspace()
                    .map_err(|_| RegexConfig::default())
}
///sets-up env
fn check_workspace()->Result<Self,std::io::Error> 
{   
        let home = std::env::home_dir()
        .ok_or_else(|| Error::new(
                ErrorKind::NotFound,"Couldn't find home directory"
        ))?;
        //go in todo 
        let root = home.join(".todo");
        let virtual_env = root.join(".virtual_env");
        let config = root.join("config.toml");
        
        //creates the workspace
        if !root.exists() {
            dbg!("root wasn't found");
            std::fs::create_dir_all(&virtual_env)?;
            std::fs::write(&config, RegexConfig::default())?;
        };

        //get content
        let content = std::fs::read_to_string(config)?;
        let workspace = WorkSpaceConfig {
            root       : root,
            virtual_env: virtual_env,
            config     : content,
        };

        Ok(workspace)
    }
}

