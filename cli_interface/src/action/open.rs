use crate::cli::config::Config;
use crate::navigation;



pub fn open_in_editor(config: Config)->Result<(),Box<(dyn std::error::Error+ 'static)>> {
       let Config{ 
        detail:_,              
        path_priority: _,    //sort with path instead of var?
        var: variable,              //lazy filter with variable
        path: p,
    } = config;

    let path = match p {
        Some(path) =>navigation::find_fs_location(path)?,
        None       =>std::fs::canonicalize(".")?,
    };

    let files = navigation::travel_filesystem(path);
    let mut all_todo = navigation::parallele_file_processing(files);
        
    //filter to todo with only the variable
    match variable {
                Some(var) =>{   
                    for t in all_todo.iter_mut() {
                        t.filter(|t| t.var == var);
                    }
                }
                None => eprintln!("Error: to open in nvim pls insert variables"),
    };
    let workspace = match workspace::VirtualWorkSpace::new(&all_todo) {
        Ok(ok)=> ok,
        Err(_)=>{
            eprintln!("error in creation of symlink");
            std::process::exit(1)
        },
    };

    std::process::Command::new("nvim")
         .arg(workspace.get_osString())
        .status()?;
    Ok(())
}



mod workspace {

use std::ffi::OsStr;
use std::path::PathBuf;
use crate::pod::FileTodo;
use std::os::unix::fs::{self as unix_fs};
use std::{env,fs};

//deletes itself when it goes out of scope
pub struct VirtualWorkSpace(Box<PathBuf>);

impl VirtualWorkSpace {
    //todo (safe_tmp_dir) {look into how to make a safe temp dir}
    
    ///Sends a Box pointer to a path that resides in the temp file
    pub fn new(files:&[FileTodo])->Result<Self, std::io::Error> {
        let tmp_dir = env::temp_dir().join("what_todo");

        if tmp_dir.exists() {
            let _ = fs::remove_dir(&tmp_dir);
        }

        match fs::create_dir(&tmp_dir) {
            Err(_)=>eprintln!("[FATAL] Failing to create a virtual fallback"),
            Ok(_) =>println!("Creating a virtual workspace"),
        }
        
        for file in files{
            //create file path
            let link = tmp_dir.join(file.get_path()
                                        .file_name()
                                        .unwrap_or_else(
                                            ||std::ffi::OsStr::new("unamed")
                                        )
            );
            unix_fs::symlink(file.get_path(), link)?;
        }
        
        Ok(VirtualWorkSpace(Box::new(tmp_dir)))      
    }

    pub fn get_osString(&self)-> &OsStr {
        self.0.as_os_str()
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

}



