use std::{ fs::{self, read_to_string}, io, path::PathBuf};

/* 
 * searching all todos and listing them -> either by path or by var
 * 
 * opening all files concerned with todo
 *
 * */
use todo_searcher::TodoBuilder;



pub fn search_fs(path:PathBuf)->Vec<PathBuf> {
    let Ok((mut file_list,mut dir_list)) = search_dir(path) 
        else { 
            panic!("error in file system");
        };
        
    while let Some(list) = dir_list.pop() {
        let Ok((mut dir_file_list,mut dir_dir_list)) = search_dir(list) 
            else {
                panic!("error in file system");
            };
        file_list.append(&mut dir_file_list);
        dir_list .append(&mut dir_dir_list);
    }
    file_list
}

pub fn search_dir(path:PathBuf)-> Result<(Vec<PathBuf>, Vec<PathBuf>), io::Error> {   
    let mut file_list:Vec<PathBuf> = Vec::new();
    let mut dir_list:Vec<PathBuf>  = Vec::new();

    let entries = fs::read_dir(&path)?; 
    for entry in entries {
        let entry = entry?;
        let metadata = entry.metadata()?;
         
        if entry.file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false) {
            continue;
        };

        match metadata.is_file() {
            true =>file_list.push(entry.path()),
            false=>dir_list.push(entry.path()),
        }


    }
    Ok((file_list, dir_list))
}



pub fn search_file(path:&PathBuf) -> Vec<(String,String)> {
    let mut v = Vec::<(String,String)>::new();

    let content = match read_to_string(path){
        Ok(content) => content,
        Err(_) =>{ log::info!("Skipped unreadable file: {:?}", path); return vec![];}
    };
    let mut builder =  TodoBuilder::new(&content);    
        
    while builder.find_next_todo() {
        let var = match builder.get_var() {
            Ok(v)  => v,
            Err(_) => {log::warn!("Error: unclosed parenthesis \n file: \ton line: "); continue;} 
        };

        let desc = match builder.get_desc() {
            Ok(d) => d,
            Err(_)=> {log::warn!("Error: unclosed curly braces\n file: \ton line: "); continue;}
        };   
        v.push((var,desc)) 
    }
    v
}   

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_search_file(){
        let mut v = search_file(&PathBuf::from("folder_test/test1.txt"));
        
        let mut result = vec![
        ("cleanup".to_string(), "Remove unused imports".to_string()),
        ("optimize".to_string(), "Refactor loop to iterator".to_string()),
        ("UI".to_string(), "Fix alignment issue on mobile".to_string()),
        ("spacing".to_string(), "Valid with extra spacing".to_string()),
                        ];
        
        assert_eq!(result.pop(), v.pop());
        assert_eq!(result.pop(), v.pop());
        assert_eq!(result.pop(), v.pop());
        assert_eq!(result.pop(), v.pop());
        assert_eq!(result.pop(), v.pop());
    }

    #[test]
    pub fn test_search_dir() {
        let Ok((mut file_list,mut dir_list)) = search_dir(PathBuf::from("folder_test")) 
            else {return eprintln!("error in file system");};
        
        while let Some(list) = dir_list.pop() {
            let Ok((mut dir_file_list,mut dir_dir_list)) = search_dir(list) 
                else {
                    return eprintln!("error in file system");
                };
            file_list.append(&mut dir_file_list);
            dir_list .append(&mut dir_dir_list);
        }

        for file in file_list {
            println!("{}",file.to_str().unwrap());
        }

    }
}





