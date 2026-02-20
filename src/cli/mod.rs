pub(crate) mod command;
use clap::{Parser, Subcommand};
use std::{path::PathBuf};



pub(crate)struct Config {
    pub silent: bool,
    pub value : Option<String>,
    pub path: PathBuf,
}


#[derive(Parser,Debug)]
#[command(name = "todo")]
pub struct Cli {
    #[command(subcommand)]
    pub command:Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    List(command::ListCommand),
    Open(command::OpenCommand),
}


pub fn run(cli: Cli)->Result<(),&'static str> { 
    let _ = match  cli.command {
        Command::List(cmd) => todo!(),//ls::list(cmd.build()),
        Command::Open(cmd) => todo!(),//open::open(cmd.build()),
    };
}




/*
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
}*/





