mod iteration;

use crate::iteration::TodoIterator;
pub struct TodoBuilder<'a>{
    iter: TodoIterator<'a>,
}

impl<'a> TodoBuilder<'a> {

    pub fn new(s:&'a str)-> Self {
       TodoBuilder{ iter: TodoIterator::new(s)}
    }

    pub fn find_next_todo(&mut self)->bool {
        let mut found = false; 

        while let Some(c) = self.iter.next(){
            match c {
                't'|'T' => { let word = std::iter::once(c)
                                    .chain(
                                        self.iter.by_ref()
                                        .take_while(|c| *c != ' ' && *c != '\n')
                                    ).collect::<String>();
                        if word.trim().eq_ignore_ascii_case("todo") {
                            found = true;
                            break;
                        } else {continue}
                },
                _      => continue,
            }
        }
        found
    }

    pub fn get_var(&mut self)-> Result<String,()> {
        let mut var = None;
        while let Some(s) = self.iter.next() {
            match s { 
                '('       =>{var = self.iter.take_while_strict(|c| c != ')')
                                       .map(|s| s.trim().to_string());
                             break;
                            },
                ' '|'\n'  => continue,
                 _        => break,
            } 
        }
        match var {
            Some(v) => Ok(v),
            None   => Err(())
        }

    }
    pub fn get_desc(&mut self)-> Result<String,()> {
        let mut desc = None;    
        while let Some(s) = self.iter.next() {
            match s { 
                '{'       =>{desc= self.iter.take_while_strict(|c| c != '}')
                                       .map(|s| s.trim().to_string());
                             break;
                            },
                ' '|'\n'  => continue,
                 _        => break,
            } 
        }
        match desc {
            Some(d) => Ok(d),
            None   => Err(())
        }
    } 
}    

#[cfg(test)]
mod test {

    use super::*;
    
    #[test]
    pub fn test_build() {//this gets triggered after search function find string 
        let mut v = Vec::<(String, String)>::new();                      //word.to_lowercase == todo
        let content = "\
todo (testing 1){description 1 }
todo (testing 3) {
todo ( testing 2){description 2}

//";
        let mut builder = TodoBuilder::new(&content); 
        while builder.find_next_todo() {
            let var = match builder.get_var() {
                Ok(v)  => v,
                Err(_) => {log::warn!("Error: unclosed parenthesis on line: "); continue;} 
            };

            let desc = match builder.get_desc() {
                Ok(d) => d,
                Err(_)=> {log::warn!("Error: unclosed curly braces on line: "); continue;}
            };
            
            v.push((var,desc))
        }
        
        let todo = v.pop().unwrap();
        assert_eq!("testing 2",todo.0); 
        assert_eq!("description 2",todo.1); 

        let todo = v.pop().unwrap();
        assert_eq!("testing 1", todo.0); 
        assert_eq!("description 1", todo.1); 

        assert_eq!(None, v.pop());
    }
}

