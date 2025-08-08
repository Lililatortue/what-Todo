use std::{collections::HashMap};


/// Parses the &str and creates todos
/// this section copies to allow to freely use the
#[derive(Debug)]
pub struct Todo {
    pub traits: HashMap<String,String>,
    pub desc: String,
}

impl Todo {
    pub fn new(traits: &str, desc: &str)->Self {
        let traits = traits.to_string();
        let desc   = desc.to_string();
        Todo { traits:Todo::parse_traits(traits), desc: desc }
    }

    fn parse_traits(traits:String)-> HashMap<String,String> {
        let mut map = HashMap::new();
        let mut i = traits.split_ascii_whitespace();

        while let Some(str) = i.next() {
            if let Some((key,value)) = str.split_once('='){
                  map.entry(key.to_string())
                     .or_insert(value.to_string());    
            } else {
                map.entry(str.to_string())
                   .or_insert("true".to_string());
            }
        }
        map
    }

    pub fn filter<P>(&mut self,mut predicate:P)->Option<Todo>
        where 
            P: FnMut(&mut Self)->Option<Todo>
    {
       predicate(self) 
    }
}



#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn build_todo() {
        let text ="todo (UI PARSER=1) {build an UI for the parser}";

        let todo = Todo::new(&text[6..17],&text[20..46]); 
    }

}
