use crate::configuration::workspace_config::WorkSpaceConfig;
use crate::navigation::{Todos};
use crate::{Config, action::filter_todo, cli::Cmd, navigation::parallel_file_processing};
use std::process::Command;

pub fn open(cmd: Cmd,config: Config)->Result<(),&'static str> {
    let workspace = config.workspace?;
    let todos = parallel_file_processing(config.regex, &cmd.path);
    let filtered = filter_todo(todos, &cmd);

    let env = VirtualWorkSpace::new(filtered, workspace)
        .unwrap_or_else(|e| {
            eprintln!("Error creating workspace: {e}");
            std::process::exit(1)
        });
     
    Command::new("nvim")
            .arg(env.0.as_os_str())
            .status() 
            .map_err(|e| {
                panic!("Failed to launch editor: {e}");
            })?;

    Ok(())
}





use std::path::PathBuf;
use std::{fs};

//deletes itself when it goes out of scope
struct VirtualWorkSpace(Box<PathBuf>);

impl VirtualWorkSpace {
    ///Sends a Box pointer to a path that resides in the temp file
    pub fn new(files:Vec<Todos>, config: WorkSpaceConfig)->Result<Self, std::io::Error> {

        println!("{}",config.virtual_env.to_string_lossy()); 
        if config.virtual_env.exists() {
            let _ = fs::remove_dir_all(&config.virtual_env);
            std::fs::create_dir(&config.virtual_env)?;
        }
        for file in files{
            //create file path
            let link = config.virtual_env
                .join(file.path
                .file_name()
                .unwrap_or_else(||std::ffi::OsStr::new("unamed")
                )
            );
            fs::hard_link(file.path, &link)?;
        }
        
        Ok(VirtualWorkSpace(Box::new(config.virtual_env)))   
    }

}

impl Drop for VirtualWorkSpace {
    
    fn drop(&mut self) {
        match fs::remove_dir_all(&**self.0){
            Err(_) => eprintln!("[FATAL] Error tmp/workspace not deleted"),
            Ok(_)  => println!("Virtual workspace deleted"),
        }
    }
}



