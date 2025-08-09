use std::{path::PathBuf};

use walkdir::{WalkDir};
use rayon::prelude::*;
use todo_searcher::todo_list::FileTodo;
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
    let files = WalkDir::new(path)
         .into_iter()
         .filter_entry(|e| {!e.file_name()
                             .to_str()
                             .map(is_hidden)
                             .unwrap_or(false)
         })
           .filter_map(Result::ok)
           .filter(|e| e.file_type().is_file())
           .map(|e| e.path().to_path_buf())
           .filter(|path| !is_binary_ext(path))
           .collect(); 
    files
}

pub fn find_dir(path:PathBuf)->Option<PathBuf>{
    let root = std::fs::canonicalize(".").unwrap();
    for directory in WalkDir::new(root)
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter(|f| f.file_type().is_dir())
    {
        if directory.file_name() == path {
            return Some(directory.into_path())
        }
    }
    None
}


pub fn parallele_file_processing(files: Vec<PathBuf>)->Vec<FileTodo> {
    let parsed_files:Vec<FileTodo> = files.par_iter().filter_map(|file| {
        let path = file.to_path_buf();
        let todos = todo_searcher::todo_list::create_list(path.clone());
        match todos {
            Ok(todo) =>Some(todo),
            Err(e)  =>{ eprintln!("{}",e); 
                        None}
            ,
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
        let directory = find_dir(PathBuf::from("project_navigator"));
        let directory = match directory {
            Some(dir) => dir,
            None      => {
                eprintln!("Error: no directory found");
                std::process::exit(1)
            },
        };
        let files = travel_filesystem(directory);
        assert_eq!(5,files.len());
    }
}




