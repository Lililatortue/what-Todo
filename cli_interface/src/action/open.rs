use crate::cli::config::Config;
use crate::navigation;
use crate::pod::FileTodo;


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
    let all_todo = navigation::parallele_file_processing(files);
   
    //filter to todo with only the variable
    let filter = match variable {
            Some(var) =>{   
                all_todo.into_iter()
                        .filter_map(|todo|todo.into_filter(|t| t.var == var))
                        .collect::<Vec<FileTodo>>()

            }
        None => panic!("Error: to open in nvim pls insert variables"),
    };



    let workspace = workspace::VirtualWorkSpace::new(filter)
        .unwrap_or_else(|e| {
        eprintln!("Error creating workspace: {e}");
        std::process::exit(1)
        });
    
 
    let status = std::process::Command::new("nvim")
                        .arg(workspace.get_os_string())
                        .status() 
                        .map_err(|e| {
                            eprintln!("Failed to launch editor: {e}");
                            e
                        })?;


    if !status.success() {
        eprintln!("Editor exited with {:?}", status.code());
    };
    Ok(())
}



mod workspace {

use std::ffi::OsStr;
use std::path::PathBuf;
use crate::pod::FileTodo;
use std::{fs};

//deletes itself when it goes out of scope
pub struct VirtualWorkSpace(Box<PathBuf>);

impl VirtualWorkSpace {
    //todo (safe_tmp_dir) {look into how to make a safe temp dir}
    
    ///Sends a Box pointer to a path that resides in the temp file
    pub fn new(files:Vec<FileTodo>)->Result<Self, std::io::Error> {
        let tmp_dir = PathBuf::from("/home/lililatortue/.cache/what_todo");

        if tmp_dir.exists() {
            let _ = fs::remove_dir_all(&tmp_dir);
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
            fs::hard_link(file.get_path(), &link)?;
        }
        
        Ok(VirtualWorkSpace(Box::new(tmp_dir)))   
    }

    pub fn get_os_string(&self)-> &OsStr {
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



