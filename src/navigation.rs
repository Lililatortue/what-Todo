use std::collections::BTreeMap;
use std::{path::PathBuf};
use regex_automata::dfa::regex::Regex;
use serde::Serialize;
use walkdir::{WalkDir};
use rayon::prelude::*;
use crate::configuration::regex_config::RegexConfig;
use crate::parser::file_automata::{find_comments,find_todo};

type Result = ( PathBuf,Vec<(String,String)> );


#[derive(Serialize)]
pub struct Todos{
    pub path: PathBuf,
    pub entries: Vec<TodoEntry>
}
#[derive(Serialize)]
pub struct TodoEntry {
    pub tag : String,
    pub content: String
}
/* 
 * searching all todos and listing them -> either by path or by var
 * 
 * opening all files concerned with todo
 *
 * */
pub fn parallel_file_processing(
    config: RegexConfig,
    path  : &PathBuf
)->Vec<Todos> {
    let files = travel_filesystem(&config.supported_ext, &path);
    
    files
        .par_iter()
        .filter_map(|file| {

        let ext   = file.extension()?.to_str()?;
        let rule  = config.supported_ext.get(ext)?;

        let content  = std::fs::read_to_string(file).ok()?;
        let comments = find_comments(&content, &rule);

        let result = Todos {
            path: file.clone(),
            entries: find_todo(comments, &config.todo_nfa)
        };
        Some(result)
    })
    .collect::<Vec<Todos>>() 
}

fn travel_filesystem(
    supported_ext: &BTreeMap<String,Regex>, 
    path: &PathBuf
)->Vec<PathBuf> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())                          // check if user can open it
        .filter(|e| 
            e.file_type().is_file() &&
            e.path().extension()
             .and_then(|ext| ext.to_str())
             .map(|ext| supported_ext.contains_key(ext)) // check if regex is available
             .unwrap_or(false)
        )
        .map(|e| e.path().to_path_buf())
        .collect()   
}









