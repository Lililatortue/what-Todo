use project_navigator;

use crate::Config;

pub fn list_todo(config:Config)->Result<(),Box<(dyn std::error::Error+ 'static)>> {
    let Config{ 
        detail:details,              //do we include desc
        path_priority: priority,    //sort with path instead of var?
        var: variable,              //lazy filter with variable
        path: p,                  //is it path specifique
    } = config; 
        
        
        let path = match p {
            Some(path) =>project_navigator::find_fs_location(path)?,
            None       =>std::fs::canonicalize(".")?,
        };
        let files = project_navigator::travel_filesystem(path);
        let mut all_todo = project_navigator::parallele_file_processing(files);
        
        //filter to todo with only the variable
        if let Some(var) = variable {
            for t in all_todo.iter_mut() {
                t.filter_content(|t| t.traits == var);
            }
        }
        //build hashmap
        match (priority, details) {
            (true,true)  => { 
               let t = table::build_path_detail_table(&all_todo);
               print!("{}",t);
            }
            (true,false) => {
                let t = table::build_path_table(&all_todo);
                print!("{}",t);
            }
            (false,true) => {
                let t = table::build_variable_detail_table(&all_todo);
                print!("{}",t);
            }
            (false,false)=> {
                let t = table::build_variable_table(&all_todo);
                print!("{}",t); 
            }
        }
        
        Ok(())
}

//super repetitive
mod table {
use std::{collections::HashMap,path::{Path, PathBuf}};
use comfy_table::{Table, presets::UTF8_FULL, ContentArrangement};
use todo_searcher::todo_list::FileTodo;

fn shorten_path(path: &Path, depth: usize) -> String {
    let comps: Vec<_> = path.components().map(|c| c.as_os_str()).collect();
    let start = comps.len().saturating_sub(depth);
    PathBuf::from_iter(&comps[start..]).display().to_string()
}




pub fn build_path_detail_table(all_todo: &[FileTodo]) -> Table {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["path", "variable", "description"]);

    for file in all_todo.iter() {
        let path_disp = shorten_path(&file.path, 3); 
        // header row for this file
        table.add_row(vec![path_disp.as_str(), "", ""]);

        // detail rows
        for item in file.list.iter() {
            table.add_row(vec!["", item.traits.as_str(), item.desc.as_str()]);
        }
    }

    table
}




pub fn build_path_table(all_todo: &[FileTodo]) -> Table {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["path", "number of todos"]);

    for file in all_todo.iter() {
        let path_disp = shorten_path(&file.path, 3);
        let lenght = file.list.len();

        table.add_row(vec![path_disp.as_str(),&lenght.to_string()]);
    }
    table
}



pub fn build_variable_detail_table(all_todo: &[FileTodo]) -> Table {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["variable", "path", "description"]);
   

    let mut map: HashMap<&str, Vec<(String,&str)>> = HashMap::new();
    for file in all_todo.iter() {
        let path_disp = shorten_path(&file.path, 3);
        for todo in file.list.iter() {
            map.entry(&todo.traits).or_default().push((path_disp.clone(), &todo.desc))
        }
        
    }

    for (key,value) in map.iter() {
        table.add_row(vec![key,"",""]);
        for v in value {
            table.add_row(vec!["",v.0.as_str(),v.1]);
        }
    }
    table
}



pub fn build_variable_table(all_todo: &[FileTodo]) -> Table {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["variable", "number of todos"]);
   

    let mut map: HashMap<&str, Vec<(String,&str)>> = HashMap::new();
    for file in all_todo.iter() {
        let path_disp = shorten_path(&file.path, 3);
        for todo in file.list.iter() {
            map.entry(&todo.traits).or_default().push((path_disp.clone(), &todo.desc))
        }
        
    }

    for (key,value) in map.iter() {
        table.add_row(vec![*key,&value.len().to_string()]);
    }
    table
}


}


