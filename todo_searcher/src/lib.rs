mod comment_parser;
mod todo_parser;
mod todo;

pub mod todo_list {
    use std::{fs,path::{PathBuf}};


    use crate::{comment_parser, todo_parser, todo::Todo}; 
    #[derive(Debug)]
    pub struct FileTodo {
        pub path: PathBuf,
        pub list: Vec<crate::todo::Todo>,
    }

    pub fn create_list(path: PathBuf) -> Result<FileTodo,&'static str> {  
        let mut file_todo = FileTodo {path:path, list:Vec::new()};

        let text = match fs::read_to_string(&file_todo.path){
            Ok(text) => text,
            Err(_) =>return Err("Error in files")
        };
         
        let parsed_text = comment_parser::parse(&text);
        let iter = match parsed_text.iter() {
            Some(content) => content,
            None => return Err("no comments in this file skipping"),
        };

        let mut builder = todo_parser::TodoStrBuilder(iter);
        
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
        Ok(file_todo) 
    }
   


    pub fn create_filtered_list_lazy<P>(
        path:PathBuf,mut predicate: P )->Result<FileTodo,&'static str>
    where 
        P: FnMut(&mut Todo)->bool
    {
        let mut file_todo = FileTodo {path:path, list:Vec::new()};

        let text = match fs::read_to_string(&file_todo.path){
            Ok(text) => text,
            Err(_) =>return Err("Error in files")
        };
         
        let parsed_text = comment_parser::parse(&text);
        let iter = match parsed_text.iter() {
            Some(content) => content,
            None => return Err("no comments in this file skipping"),
        };

        let mut builder = todo_parser::TodoStrBuilder(iter);
    
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
            let mut todo = Todo::new(var_str,desc_str);
            match predicate(&mut todo) {
                true => file_todo.list.push(todo),
                false=>(),
            }
        }
        Ok(file_todo) 
    }
}



