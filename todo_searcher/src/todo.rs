use std::{collections::HashMap, path::PathBuf};


// general Idea:
// ex: (UI priorite=2  train) ==
// taits {(UI,true) , (priorite, 2), (train, true)}
//
//

pub struct Todo {
    path:PathBuf,
    traits: HashMap<String,String>,
    desc: String,
}

impl Todo {
    pub fn new(path:PathBuf, traits: String, desc: String)->Self{
        Todo { path, traits:Todo::parse_traits(traits), desc }
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
}


