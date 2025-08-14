pub mod config;

use crate::action::{ls,open};
use crate::cli::config::{Command,Cli};

pub fn run(cli: Cli)->Result<(),&'static str> { 
     let _ =match  cli.command {
        Command::Ls(arg)=> {
            let config = arg.build_config();
            match ls::list_todo(config) {
                Ok(_)  =>Ok(()),
                Err(e) => Err(e)
            }
        },
        //not supported yet working on it
        Command::Open (_arg) => { Ok(())
            //let config = arg.build_config();
            //open_in_editor(config)?;
        }, 
    };
    Ok(())
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





