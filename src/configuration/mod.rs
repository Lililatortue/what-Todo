mod config;
use std::path::PathBuf;
use std::io::{Error,ErrorKind};

use crate::configuration::config::Rules;
///sets-up env
pub fn check_workspace()->Result<PathBuf,std::io::Error> 
{
    let mut app_path = std::env::home_dir()
        .ok_or_else(|| Error::new(
                ErrorKind::NotFound,"Couldn't find home directory"
        ))?;

    app_path.push(".todo");
    
    if !app_path.exists() 
    {
        dbg!("path wasn't found");
        setup_env(&mut app_path);
    };

    Ok(app_path)
}   


pub fn setup_env(root: &mut PathBuf)->Result<(),std::io::Error> 
{
    dbg!("creating .virtual-env");
    let env = root.join(".virtual_env");
    std::fs::create_dir_all(env)?;
    
    dbg!("creating config.toml");
    let config = root.join("config.toml");
    std::fs::write(config, Rules::default())?;

    Ok(())
}
