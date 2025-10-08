pub mod parser;

use std::{path::{PathBuf},fs};
use crate::pod::{FileTodo};


    /// creates a list of all todo of FileTodo
    pub fn create_list(path: PathBuf) -> Result<FileTodo,String> {  
        
        let text = match fs::read_to_string(path){
            Ok(text) => text,
            Err(_) =>return Err(format!("Error in files {}",path.display()))
        };
        
        parser::(text);

        Ok(FileTodo { path: PathBuf::from("d"), list: vec![] })
    }



