use std::path::PathBuf;

//Todo (encapsulation) {remove pub and fix the outcomes}
    #[derive(Debug)]
    pub struct FileTodo {
        pub path: PathBuf,
        pub list: Vec<Todo>,
    }

    impl FileTodo {
        pub fn get_list(&self)-> &Vec<Todo> {
            &self.list
        }
        pub fn get_path(&self)->&PathBuf{
            &self.path
        }

        pub fn filter<P>(&mut self, mut predicate: P)
            where P: FnMut(&Todo)-> bool  
        {
            self.list.retain(|todo| predicate(todo))
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
