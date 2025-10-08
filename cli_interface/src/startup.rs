use std::{collections::{HashMap, HashSet}, fs};
use crate::utils::folder_structure::config;
use miniregex::{Parser,make_fsa};
use miniregex::graph::FSA;
use serde::Deserialize;

// Facade 
// Handles creation of config of LexerGraph 
pub fn init_graphs()->(LexerGraphs, CommentGraphs)
{
    let content = match fs::read_to_string(config()) {
        Ok(content)=> content,
        Err(e)=> {eprintln!("{e}"); std::process::exit(1)},
    };

    let parser_rule:LexerConfig     = match toml::from_str(&content) {
        Ok(value)=> value,
        Err(e)=> {eprintln!("{e}"); std::process::exit(1)}, 
    };
    let comment_config:CommentConfig = match toml::from_str(&content) {
        Ok(value)=> value,
        Err(e)=> {eprintln!("{e}"); std::process::exit(1)}, 
    };

    (parser_rule.into_nfa(), comment_config.into_nfa())
}


//for parsing parser rules in Toml
#[derive(Debug,Deserialize)]
pub struct LexerConfig {
    keyword:String,
    scopes: HashMap<String,String>,
}

impl LexerConfig {
    pub fn into_nfa(self)->LexerGraphs {

        let mut scopes: HashMap<String,FST> = HashMap::new();
        let keyword_rule = Parser::new(&self.keyword).parse();

        for (key, value) in self.scopes {
            scopes.insert(key.clone(),Parser::new(&value).parse());  
        }
        
        if scopes.is_empty() {
            eprintln!("[WARN] no scopes in parser")
        }

         LexerGraphs {keyword: keyword_rule, scopes: scopes}
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

            //block doesnt exist just create line graph
            let Some(block) = &rule.comment_block else { 
                graphs.push(make_fsa!(&rule.comment_line));
                
                for ext in rule.extensions {
                    map.insert(ext,line_ptr);
                }
                continue
            };
            // if block exist add alternation to create one graph
            graphs.push(make_fsa!(block, &rule.comment_line));
            
            for ext in rule.extensions {
                map.insert(ext,line_ptr);
            }
        }  
        CommentGraphs {graphs: graphs, list: map}
    }
}


    
type Ptr = usize;
pub struct CommentGraphs {
    graphs: Vec<FSA>,
    list: HashMap<String,Ptr>, //ptr to vec graph
} 

impl CommentGraphs {
    pub fn get_graph(&self, extension:&str)->Result<&FSA,String> {

        let line_ptr = match self.list.get(extension) {
            Some(ptr) => ptr,
            None => return Err(format!("No file support for {}",extension)),
        };

        return Ok(&self.graphs[*line_ptr])
    }

    pub fn get_extensions(&self)->HashSet<&str> {
        let mut hash_set:HashSet<&str> = HashSet::new();
        for (key,_) in self.list.iter() {
            hash_set.insert(key);
        }
        hash_set
    }
}



#[derive(Debug)]
pub struct LexerGraphs {
    pub keyword: ,
    pub scopes : HashMap<String,Graph> 
}





