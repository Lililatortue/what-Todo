use std::{collections::HashMap, path::PathBuf};

use project_navigator;



use crate::Config;

pub fn list_todo(config:Config)->Result<(),Box<(dyn std::error::Error+ 'static)>> {
    let Config{ 
        detail:detail,              //do we include desc
        path_priority: priority,    //sort with path instead of var?
        var: variable,              //lazy filter with variable
        path: path                  //is it path specifique
    } = config; 
        
        
        let path = match path {
            Some(path) =>project_navigator::find_fs_location(path)?,
            None       =>std::fs::canonicalize(".")?,
        };
        let files = project_navigator::travel_filesystem(path);
        let mut all_todo = project_navigator::parallele_file_processing(files);
        
        //filter to todo with only the variable
        if let Some(var) = variable {
            for t in all_todo.iter_mut() {
                t.filter_content(|t| t.traits.contains_key(&var));
            }
        }
        //build hashmap
        match priority {
            true => {//var , desc
                let mut map: HashMap<&PathBuf, Vec<(String, &String)>> = HashMap::new();
            
                for file_todo in all_todo.iter() {
                    for todo in file_todo.list.iter() {
                        map.entry(&file_todo.path).or_default().push(
                            (hashmap_to_string(&todo.traits),&todo.desc)
                        )
                    }
                }
                if detail {
                    for (key,value) in map{
                        println!("\nPath: {} [\n",key.display()); 
                        for (traits, desc) in value {
                            if &traits == variable {
                                println!("\tVariable: ({}), \n\tdescription{{\n\t\t{}\n\t}}\n]",traits,desc)
                            }
                        }
                    }   

                } else {
                    for (key,value) in map{
                        println!("Path: {},\n\t amount of todos to do: {}",key.display(), value.len()); 
                    }
                }
            }
            false => {
                let mut map: HashMap<&String, Vec<(&PathBuf, &String)>> = HashMap::new();

                for file_todo in all_todo.iter() {
                    for todo in file_todo.list.iter() {
                        for (key,_) in todo.traits.iter() {
                            map.entry(key).or_default().push((&file_todo.path,&todo.desc));
                        }
                    }
                }
                if detail {
                    for (key,value) in map{
                        println!("\nVariable: {}:[\n",key); 
                        for (path, desc) in value {
                            println!("Path: {}, \n\tdescription{{\n\t\t{}\n\t}}\n]",path.display(),desc)
                        }
                    }              
                } else {
                    for (key,value) in map{
                        println!("Variable: {},\n\t amount of todos to do: {}",key, value.len()); 
                    }
                }

            }
        }
        Ok(())  
}



fn hashmap_to_string<K: ToString, V: ToString>(map: &HashMap<K, V>) -> String {
    map.iter()
        .map(|(k, v)| format!("{}: {}", k.to_string(), v.to_string()))
        .collect::<Vec<_>>()
        .join(", ")
}


