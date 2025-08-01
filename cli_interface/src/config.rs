use std::path::PathBuf;

/// struct with the sole purpose of handling default behavior  
/// for inputs
///
#[allow(dead_code)]
pub struct Config {
    pub var : Option<String>,
    pub path: PathBuf,
    pub multi_threading: bool,
    pub details: bool,
    pub sort_by_path: bool,
}

impl Config {
    
    pub fn for_ls(long:bool, path_priority:bool, path: PathBuf)->Self {
        // if no path is provided, it assumes current_dir();
        let (p,m) = match path.is_dir() {
            true => (path, true),
            false=> (path, false), 
        };    
     
        Config {
            var: None,
            path: p,
            multi_threading: m,
            details:long,
            sort_by_path: path_priority,
        }
    }

   pub fn for_open(var: String, _editor: String )-> Config {

         Config {
            var: Some(var),
            path: PathBuf::from("."),
            multi_threading: false,
            details: false,
            sort_by_path: false,
        }   

   }

}

#[cfg(test)]
mod test {
        

}






