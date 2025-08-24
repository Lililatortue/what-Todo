use std::{fs};

use crate::utils::folder_structure::{self, folder};

/*  what-todo folder manipulation responsible for files
 *  full file structure is 
 *  .what_todo                           /create on launch
 *  |
 *  |__ .cache/     --> open sessions    /create on launch
 *  |__ logs.log    --> allows debugging /create on launch
 *  |__ journal.txt --> query history not implemented yet /create when needed
 *
 * */

pub fn setup()->std::io::Result<()> { 
    let folder = folder_structure::folder();
    fs::create_dir(folder)?;

    let cache  = folder_structure::dot_cache();
    fs::create_dir(cache)?;


    toml::init()?;
    journal::init();    

    Ok(())
}

pub mod toml {
use std::{fs};
use crate::utils::folder_structure;


    //default content of file
    static DEFAULT_CONTENT:&str =r"[[parser]]
keyword      = todo|note
variable     = \(.*\)+
description  = \{.*\}+
...

[[rule]]
comment_block = /\*.*\*/
comment_line  = (//.*\n)* 
extension     = [c, cpp, rs, cs, java, swift, go, typescript]

[[rule]]
comment_block = /*.*\*/
comment_line  = (--.*\n)*
extension     = [sql]

[[rule]]
comment_block = <!--.*-->
comment_line  = None
extension     = [html]";


    ///init config file if exists do nothing
    ///if not recreate it has default
    pub fn init()->Result<(),std::io::Error> {
        let config = folder_structure::config();
        if fs::exists(&config)? {
            println!("Config exist");
            Ok(())
        } else {
            let _ =fs::write(config,DEFAULT_CONTENT);
            Ok(())
        }
    }


    //checks if file exist overwrites it to default
    pub fn default()->Result<(), std::io::Error> {
        let config = folder_structure::config();
        fs::write(config,DEFAULT_CONTENT)?;
        Ok(())
    }
}

//todo (journal) {make journal after making queries}
pub mod journal {
    pub fn init(){
    }

    pub fn refresh(){

    }
}
