use std::collections::{BTreeMap};
use regex_automata::{dfa::regex::Regex, nfa::thompson::pikevm::PikeVM};
use toml;
use serde::Deserialize;

use crate::configuration::error::ConfigError;

#[derive(Deserialize)]
struct TomlConfig{ 
    method: MethodSection,
    comments: BTreeMap<String,Vec<String>>
}
#[derive(Deserialize)]
struct MethodSection {todo: String}

//represents data in toml
//for now doesn't support custom todo rules
//but it does support custom comment rules
pub struct RegexConfig{
    pub todo_nfa     : PikeVM, 
    pub supported_ext: BTreeMap<String, Regex>,
    pub warnings     : Vec<ConfigError> //banal errors
}

impl RegexConfig {
    pub fn new(config : &str)-> Result<Self,ConfigError> {
        //extract toml data
        let config:TomlConfig = toml::from_str(config)
            .map_err(|e| ConfigError::bad_config(e.to_string()))?;

        //create todo rules
        let vm = PikeVM::new(&config.method.todo)
            .map_err(|e| ConfigError::bad_regex("todo".to_string(), e.to_string()))?;
        

        //create comment rules
        let (ok, err): (Vec<_>, Vec<_>) = config.comments
            .into_iter()
            .map(|(k,v)| { 
                let patterns: Vec<&str> = v.iter().map(|s| s.as_str()).collect();     
                Regex::builder()
                    .build_many(&patterns)
                    .map(|re| (k.clone(), re))
                    .map_err(|e| ConfigError::bad_regex(k.clone(),e.to_string()))
            })
            .partition(|res|res.is_ok());
        
        let map:BTreeMap<String,Regex> = ok
            .into_iter()
            .filter_map(Result::ok)
            .collect();


        //for now always display bad regex
        let warning = err.into_iter()
                         .filter_map(Result::err)
                         .collect(); 


        Ok(RegexConfig{ 
            todo_nfa     : vm, 
            supported_ext: map,
            warnings     : warning, 
        })
    }


    pub fn default()->&'static str {
        let config = r#"
        [method]
        todo = '(?i)todo\s*\((?P<val>.*?)\)\s*\{(?P<desc>.*?)\}'

        [comments]
        rs   = ['//.*',  '(?s)/\*.*?\*/',  '(?s)".*"']
        cs   = ['//.*',  '(?s)/\*.*?\*/',  '(?s)".*"']
        java = ['//.*',  '(?s)/\*.*?\*/',  '(?s)".*"']
        kt   = ['//.*',  '(?s)/\*.*?\*/',  '(?s)".*"']
        txt  = ['.*']   
        "#;
        config
    }
}

#[cfg(test)]
mod test {
    use crate::configuration::regex_config::RegexConfig;

    #[test]
    pub fn test_conversion(){
        let str = RegexConfig::default();
        
        let rules= RegexConfig::new(str).expect("bad rules");
        
        assert_eq!(rules.supported_ext.get("rs").unwrap().pattern_len(),3);
        assert_eq!(rules.supported_ext.get("cs").unwrap().pattern_len(),3);
        assert_eq!(rules.supported_ext.get("java").unwrap().pattern_len(),3);
        assert_eq!(rules.supported_ext.get("kt").unwrap().pattern_len(),3);
        assert_eq!(rules.supported_ext.get("txt").unwrap().pattern_len(),1);
    }
}
