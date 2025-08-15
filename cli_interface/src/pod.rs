use std::path::PathBuf;
//Todo (encapsulation) {remove pub and fix the outcomes}
    #[derive(Debug)]
    pub struct FileTodo {
        pub path: PathBuf,
        pub list: Vec<Todo>,
    }

    impl FileTodo {
        pub fn new(path:PathBuf,list:Vec<Todo>)-> Self{
            FileTodo {path, list: list}
        }
        pub fn get_list(&self)-> &Vec<Todo> {
            &self.list
        }
        pub fn get_path(&self)->&PathBuf{
            &self.path
        }

        pub fn into_filter<P>(self, mut predicate: P)-> Option<Self>
            where P: FnMut(&Todo)-> bool  
        {
            let list:Vec<Todo> = Vec::new();
            let mut new_self = FileTodo::new(self.path,list);
            for todo in self.list {      
                if predicate(&todo) {
                    new_self.list.push(todo);
                }
            }
            if new_self.list.is_empty(){
                return None
            };
            Some(new_self)
        }
    }


    /// Parses the &str and creates todos
    /// this section copies to allow to freely use the
    #[derive(Debug)]
    pub struct Todo {
       pub var: String,
       pub desc: String,
    }

    impl Todo {
        pub fn new(var: &str, desc: &str)->Self {
            let var = var.trim().to_string();
            let desc   = desc.trim().to_string();
            Todo {var, desc}
        }

    }
