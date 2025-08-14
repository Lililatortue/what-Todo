use std::{path::PathBuf};

use ignore::WalkBuilder;
use walkdir::{WalkDir};
use rayon::prelude::*;
use crate::parser;
use crate::pod::FileTodo;
/* 
 * searching all todos and listing them -> either by path or by var
 * 
 * opening all files concerned with todo
 *
 * */
fn is_hidden(name: &str)-> bool {
    name.starts_with('.')
}

fn is_binary_ext(path:& PathBuf)-> bool {
    matches!(path.extension().and_then(|ext| ext.to_str()),
              Some("exe" | "dll" | "so" | "bin" | "class" | "o" | "a"
            | "jpg" | "jpeg" | "png" | "gif" | "pdf" | "zip" | "tar" | "gz")   
            )
}

pub fn travel_filesystem(path: PathBuf)->Vec<PathBuf> {
    let walk = WalkBuilder::new(path)
        .hidden(true)       
        .ignore(true)               
        .git_ignore(true)           
        .git_exclude(true)          
        .git_global(true)           
        .follow_links(false)
        .build();

        walk.filter_map(Result::ok)                                  
        .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
        .map(|e| e.into_path())
        .filter(|p| !is_binary_ext(p))                          
        .collect()
}

pub fn find_fs_location(path:PathBuf)->Result<PathBuf,&'static str>{
    let root = std::fs::canonicalize(".").unwrap();
    for directory in WalkDir::new(root)
                    .into_iter()
                    .filter_map(Result::ok)
    {
        if directory.file_name() == path {
            return Ok(directory.into_path())
        }
    }
    Err("Error: non-existant path {}")
}


pub fn parallele_file_processing(files: Vec<PathBuf>)->Vec<FileTodo> {
    let parsed_files:Vec<FileTodo> = files.par_iter().filter_map(|file| {
        let path = file.to_path_buf();
        let todos = parser::create_list(path.clone());
        match todos {
            Ok(todo) =>Some(todo),
            Err(e)  =>{ log::warn!("{}",e); 
                        None},
        }
    }).collect();
    parsed_files 
}




#[cfg(test)]
mod test {
    use super::*; 
    #[test]
    pub fn test_navigation_current_path(){
        let root = std::fs::canonicalize(".").unwrap();
        let files = travel_filesystem(root);
        assert_eq!(5,files.len());
        //for debug purposes use -- --nocapture to look at what it prints
        for file in files {
            println!("{}",file.display());
        }
    }

    #[test]
    pub fn test_navigation_path(){
        let directory = find_fs_location(PathBuf::from("project_navigator"));
        let directory = match directory {
            Ok(dir) => dir,
            Err(e)      => {
                eprintln!("{}",e);
                std::process::exit(1)
            },
        };
        let files = travel_filesystem(directory);
        assert_eq!(5,files.len());
    }
}




