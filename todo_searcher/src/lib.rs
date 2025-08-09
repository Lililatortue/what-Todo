mod comment_parser;
mod todo_parser;

pub mod todo_list {
    use std::{fs,path::{PathBuf}};
    use std::{collections::HashMap};


    /// Parses the &str and creates todos
    /// this section copies to allow to freely use the
    #[derive(Debug)]
    pub struct Todo {
        pub traits: HashMap<String,String>,
        pub desc: String,
    }

    impl Todo {
        pub fn new(traits: &str, desc: &str)->Self {
            let traits = traits.to_string();
            let desc   = desc.to_string();
            Todo { traits:Todo::parse_traits(traits), desc: desc }
        }

        fn parse_traits(traits:String)-> HashMap<String,String> {
            let mut map = HashMap::new();
            let mut i = traits.split_ascii_whitespace();

            while let Some(str) = i.next() {
                if let Some((key,value)) = str.split_once('='){
                    map.entry(key.to_string())
                        .or_insert(value.to_string());    
                } else {
                    map.entry(str.to_string())
                        .or_insert("true".to_string());
                }
            }
            map
        }

        pub fn filter<P>(&mut self,mut predicate:P)->Option<Todo>
            where 
                P: FnMut(&mut Self)->Option<Todo>
        {
            predicate(self) 
        }
    }

    use crate::{comment_parser, todo_parser }; 
    #[derive(Debug)]
    pub struct FileTodo {
        pub path: PathBuf,
        pub list: Vec<Todo>,
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



