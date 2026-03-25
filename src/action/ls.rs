use crate::action::{filter_todo};
use crate::cli::command::OutputType;
use crate::{cli::Cmd};
use crate::{Config, navigation::*}; 
use comfy_table::{Table, presets::UTF8_FULL, ContentArrangement};






pub fn list_todo(cmd: Cmd, config: Config) {
    let todos = parallel_file_processing(config.regex, &cmd.path);
    let filtered = filter_todo(todos, &cmd);
    //show result
    match cmd.output {
        OutputType::Visual =>show(&filtered, &cmd), 
        OutputType::Json   =>json(&filtered),
        _ => (),
    };
}

fn show(todos: &Vec<Todos>, cmd: &Cmd) {
   
    let mut table = Table::new();
    table.load_preset(UTF8_FULL)
         .set_content_arrangement(ContentArrangement::Dynamic)
         .set_header(build_header(cmd));
    
    for todo in todos {
        let path_str = todo.path.to_string_lossy();
        for entry in &todo.entries {
            if cmd.silent {
               table.add_row(vec![path_str.as_ref(),&entry.tag]); 
            } else {
               table.add_row(vec![path_str.as_ref(),&entry.tag, &entry.content]); 
            }
        }
    }
    println!("{table}")
}

#[inline]
fn build_header(cmd: &Cmd)->Vec<&'static str> {
    if cmd.silent {
        vec!["path","variable"]
    } else {
       vec!["path","variable","description"]
    }
}

fn json(todos: &Vec<Todos>) {
    match serde_json::to_string_pretty(todos) {
        Ok(json_output) => {
            println!("{}", json_output);
        }
        Err(e) => {
            eprintln!("Error: Failed to serialize TODOs to JSON: {}", e);
        }
    }
}
