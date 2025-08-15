use std::{env,fs, path::PathBuf};

/*  what-todo folder manipulation responsible for files
 *  full file structure is 
 *  .what_todo                           /create on launch
 *  |
 *  |__ .cache/     --> open sessions    /create on launch
 *  |__ logs.log    --> allows debugging /create on launch
 *  |__ journal.txt --> query history not implemented yet /create when needed
 *
 * */

pub fn init()->std::io::Result<()> { 
    let parent = find_home().join("what_todo/");
    let cache_path = parent.join(".cache/");
    let log_path = parent.join("logs.log"); 
    let journal_path = parent.join("journal.txt");
    
    fs::create_dir_all(&cache_path)?;

    if !log_path.exists(){
        fs::File::create(log_path);
    }
    if !journal_path.exists(){
        fs::File::create(journal_path);     
    }
    Ok(())
}


fn find_home()->PathBuf {
    match env::var("HOME"){
        Ok(ok)  => PathBuf::from(ok),
        Err(_)  => {
            eprintln!("[Fatal] can't find home.");
            std::process::exit(1)
        }
    }

}
