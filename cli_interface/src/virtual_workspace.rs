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
}

impl Drop for VirtualWorkSpace {
    
    fn drop(&mut self) {
        match fs::remove_dir(&**self.0){
            Err(_) => eprintln!("[FATAL] Error tmp/workspace not deleted"),
            Ok(_)  => println!("Virtual workspace deleted"),
        }
    }
}
