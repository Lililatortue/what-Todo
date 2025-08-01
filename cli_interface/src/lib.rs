mod config;


use std::{collections::HashMap, path::PathBuf};
use clap::{Parser, Subcommand};


pub fn run(cli: Cli)->Result<(),&'static str> { 
    match cli.command {
        Command::Ls {long, path_priority, path}=> {
            let config = config::Config::for_ls(long,path_priority,path);
            create_all_todo(config)?; 
        },
        Command::Open {var, editor} => {
            let config = config::Config::for_open(var, editor);
            open_in_editor(config)?;
        }, 
    }
    Ok(())
}


// super unoptimal but idc for now 
// will improve through the next month as i get better
fn create_all_todo(config: config::Config) -> Result<(), &'static str> {
    let mut file_list = project_navigator::search_fs(config.path);

    let mut map:HashMap<String, Vec<(PathBuf,String)>> = HashMap::new(); 
    //go through every file
    while let Some(p) = file_list.pop() {
        //parse every todo in file
        let mut v = project_navigator::search_file(&p);
        //put it in a hashmap
        while let Some((var, desc)) = v.pop() {
            map.entry(var).or_default().push((p.to_path_buf(), desc));
        }
    }
    for (key, vector) in map {
        println!("\n{key}:");
        for (path, desc) in vector {
            println!("\t{}, {desc}",path.to_string_lossy().to_string());
        }
    }
    Ok(()) 
}
fn open_in_editor(mut config: config::Config) -> Result<(), &'static str> {
    let mut file_list = project_navigator::search_fs(config.path);
    
    let mut map:HashMap<String, Vec<(PathBuf,String)>> = HashMap::new(); 

    let var = config.var.take().expect("variable needs to be declared"); 

    //go through every file
    while let Some(p) = file_list.pop() {
        //parse every todo in file
        let mut v = project_navigator::search_file(&p);
        //put it in a hashmap
        while let Some((var, desc)) = v.pop() {
            map.entry(var).or_default().push((p.to_path_buf(), desc));
        }
    }
    let path:Vec<PathBuf> = map
        .entry(var)
        .or_default()
        .iter()
        .map(|(path, _)| path.clone())
        .collect();

    std::process::Command::new("nvim")
        .args(&path)
        .status()
        .expect("failed process back to the drawing board");

    Ok(())
}


#[derive(Parser,Debug)]
#[command(name = "todo")]
pub struct Cli {
    #[command(subcommand)]
    command:Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    // list all todos
    Ls  {
        #[arg(short)]
        long: bool, //full details
        #[arg(short)]
        path_priority: bool, //sorts base on path, defaults to variables
        #[arg(default_value = ".")]
        path: PathBuf,//can check recursively if path provided defaults to current
    },
    // open every file concerned with variable then opens it with editor
    Open {
        var: String,
        editor: String,
    },

}




