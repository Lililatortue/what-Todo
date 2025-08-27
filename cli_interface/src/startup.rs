use std::{collections::{HashMap, HashSet}, fs};
use crate::utils::folder_structure::config;
use miniregex::{graph::Graph,Parser};
use serde::Deserialize;



//creates the nfa based on the 
pub fn init_config()->(ParserConfig, CommentConfig)
{
    let content = match fs::read_to_string(config()) {
        Ok(content)=> content,
        Err(e)=> {eprintln!("{e}"); std::process::exit(1)},
    };

    let parser_rule:ParserConfig    = match toml::from_str(&content) {
        Ok(value)=> value,
        Err(e)=> {eprintln!("{e}"); std::process::exit(1)}, 
    };
    let comment_rules:CommentConfig = match toml::from_str(&content) {
        Ok(value)=> value,
        Err(e)=> {eprintln!("{e}"); std::process::exit(1)}, 
    };
    
    (parser_rule,comment_rules)
}


//for parsing parser rules in Toml
#[derive(Debug,Deserialize)]
pub struct ParserConfig {
    keyword:String,
    scopes: HashMap<String,String>,
}

impl ParserConfig {
    pub fn into_nfa(self)->ParserGraphs {

        let mut scopes: HashMap<String,Graph> = HashMap::new();
        let keyword_rule = Parser::new(&self.keyword).parse();

        for (key, value) in self.scopes {
            scopes.insert(key.clone(),Parser::new(&value).parse());  
        }
        
        if scopes.is_empty() {
            eprintln!("[WARN] no scopes in parser")
        }

         ParserGraphs {keyword: keyword_rule, scopes: scopes}
    }
}

//for parsing comment rules in toml
#[derive(Debug,Deserialize)]
struct Rule {
    comment_line : String,
    comment_block: Option<String>,
    extensions :Vec<String>
}
#[derive(Debug,Deserialize)]
pub struct CommentConfig {
    rules:Vec<Rule>,
}

impl CommentConfig {
    pub fn into_nfa(self)->CommentGraphs {
        let mut graphs= vec![];
        let mut map = HashMap::new();

        for rule in self.rules {
            let line_ptr = graphs.len();
            graphs.push(Parser::new(&rule.comment_line).parse());
             
            let Some(block) = &rule.comment_block else {
                for ext in rule.extensions {
                    map.insert(ext,(line_ptr, None));
                }
                continue
            };

            let block_ptr = graphs.len();
            graphs.push(Parser::new(block).parse());
            
            for ext in rule.extensions {
                map.insert(ext, (line_ptr, Some(block_ptr)));
            }
        }
    
        CommentGraphs {graphs: graphs, list: map}
    }
}


    
type Block = Option<usize>;
type Line  = usize;

pub struct CommentGraphs {
    graphs: Vec<Graph>,
    list: HashMap<String,(Line,Block)>, //ptr to vec graph
} 

impl CommentGraphs {
    pub fn get_graph(&self, extension:&str)->Result<(&Graph,Option<&Graph>),String>{
        let (line_ptr, block_ptr) = match self.list.get(extension) {
            Some((ptr1,ptr2)) => (ptr1,ptr2),
            None => return Err(format!("No file support for {}",extension)),
        };

        if let Some(block_ptr) = block_ptr {
            return Ok((&self.graphs[*line_ptr], Some(&self.graphs[*block_ptr])))

        } else {
            return Ok((&self.graphs[*line_ptr], None));
        } 
    }

    pub fn get_extensions(&self)->HashSet<&String> {
        let mut hs = HashSet::new();
        for (key,_) in self.list.iter() {
            hs.insert(key);
        }
        hs
    }


}



#[derive(Debug)]
pub struct ParserGraphs {
    pub keyword: Graph,
    pub scopes : HashMap<String,Graph> 
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_toml() {
    }

}



