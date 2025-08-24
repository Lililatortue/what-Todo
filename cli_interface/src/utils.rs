


/// static path nomenclature
/// get quick acces to folder
pub mod folder_structure {
    use std::{env, path::{Path, PathBuf}};


    static PATH:      &str = ".what_todo";
    static DOT_CACHE: &str = ".what_todo/.cache/";
    static CONFIG:    &str = ".what_todo/config.toml";
    
    pub fn folder()->PathBuf {
        let mut path = PathBuf::from(home());
        path.push(PATH);
        path
    }
    pub fn dot_cache()->PathBuf {
        let mut path = PathBuf::from(home());
        path.push(DOT_CACHE);
        path
    }
    pub fn config()->PathBuf {
        let mut path = PathBuf::from(home());
        path.push(CONFIG);
        path
    }

    fn home()->String {
        match env::var("HOME"){
            Ok(ok)  => ok,
            Err(_)  => {
                eprintln!("[Fatal] can't find home.");
                std::process::exit(1)
            }
        }
    }

}

