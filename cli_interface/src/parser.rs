mod comment_parser;
mod todo_parser;
use std::{fs,path::{PathBuf}};
use crate::pod::{FileTodo,Todo};


    /// creates a list of all todo of FileTodo
    pub fn create_list(path: PathBuf) -> Result<FileTodo,String> {  
        let mut file_todo = FileTodo {path:path, list:Vec::new()};

        let text = match fs::read_to_string(&file_todo.path){
            Ok(text) => text,
            Err(_) =>return Err(format!("Error in files {}",&file_todo.path.display()))
        };
         
        let parsed_text = comment_parser::parse(&text);
        let iter = match parsed_text.iter() {
            Some(content) => content,
            None => return Err(format!("no comments in this file skipping {}",&file_todo.path.display())),
        };

        let mut builder = todo_parser::TodoStrBuilder(iter);
// todo (UI) {for hardlink}
// todo (UI) {for hardlink}  
        loop {
            if !builder.find_todo(){
                break;
            }
            let var_str  = match builder.get_var(){
                Some(val)=>&text[val.0..val.1],
                None =>{  continue;}//find next
            };
            let desc_str = match builder.get_desc() {
                Some(val)=>&text[val.0..val.1],
                None =>{  continue;}//find next
            };
            let todo = Todo::new(var_str,desc_str);

            file_todo.list.push(todo)
        }
        match !file_todo.list.is_empty() {
            true =>Ok(file_todo), 
            false=> return Err(format!("empty file")),

        }
    }



