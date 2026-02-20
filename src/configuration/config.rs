use std::collections::{BTreeMap};
use miniregex::{Nfa, make_nfa};
use toml;
use serde::Deserialize;

#[derive(Deserialize)]
struct TomlConfig{
    comments: BTreeMap<String,String>
}

//represents data in toml
//for now doesn't support custom todo rules
//but it does support custom comment rules
pub struct Rules{
    todo_nfa    : Nfa, 
    comment_nfas: BTreeMap<String, Nfa>
}

impl Rules {
    pub fn new(config : &str)-> Self {
        let config:TomlConfig = toml::from_str(config).expect("[Bad config]");
        let map = config.comments
                        .into_iter()
                        .map(|(k,v)| {
                            (k, make_nfa!(&v))           
                        }).collect();
        Rules      
        {
            todo_nfa: make_nfa!(r"todo \s* \(.*\) \s* \{ .* \}"),
            comment_nfas:map,
        }
    }


    pub fn default()->&'static str {
        let config = r#"[comments]
        rs   = '// .* \n | /\* .* \*/'
        cs   = '// .* \n | /\* .* \*/'
        java = '// .* \n | /\* .* \*/'
        kt   = '// .* \n | /\* .* \*/'
        txt  = '.*'   
        "#;
        config
    }
}

#[cfg(test)]
mod test {
    use crate::configuration::config::Rules;

    #[test]
    pub fn test_conversion(){
        let str = Rules::default();
        
        let rules= Rules::new(str);

        assert!(rules.comment_nfas.get("rs").is_some());
        assert!(rules.comment_nfas.get("cs").is_some());
        assert!(rules.comment_nfas.get("java").is_some());
        assert!(rules.comment_nfas.get("kt").is_some());
        assert!(rules.comment_nfas.get("txt").is_some());
    }
}
