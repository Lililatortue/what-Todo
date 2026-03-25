use regex_automata::{dfa::regex::Regex, nfa::thompson::pikevm::PikeVM};

use crate::navigation::TodoEntry;


//Regex takes three rules
//first rule is line_comments
//second rule is multiline comments
//third is String
//the Order must be respected
pub fn find_comments<'a,'b>(content:&'a str, regex: &'b Regex)->Vec<&'a str> {
    let mut comments = vec![];

    for m in regex.find_iter(content.as_bytes()) {
        let pattern_id = m.pattern().as_usize();

        match pattern_id {
            0 => comments.push(&content[m.range()]),
            1 => comments.push(&content[m.range()]),
            _ => (),
        }
    }
    comments 
}

pub fn find_todo(comments: Vec<&str>,vm: &PikeVM)-> Vec<TodoEntry> {
    let mut todos = vec![];
    let mut cache = vm.create_cache();
    
    for comment in comments.iter() {
        let bytes = comment.as_bytes();

        for caps in vm.captures_iter(&mut cache, bytes) {
            
            let category    = caps.get_group_by_name("val").map(|m| &comment[m.range()]);    
            let description = caps.get_group_by_name("desc").map(|m| &comment[m.range()]);

            if let (Some(cat), Some(desc)) = (category, description) {
                todos.push(TodoEntry{tag:cat.to_string(),content:desc.to_string()})
            }
        }
    }
    todos
} 

#[cfg(test)]
mod test {
    use super::*; 

    
    const TEXT_LINE: &str = "// todo    (value)  {desc} todo(value){desc}\ntodo(value){desc}";
    const TEXT_MULTILINE: &str = r#"/* todo    (value)  {desc} todo(value){desc}
todo(value){desc}*/"//todo(value){desc}""#;
    
    
    #[test]
    pub fn parsing_comment() 
    {
        let dfa = match Regex::new_many(&["//.*",r"(?s)/\*.*?\*/", r#"".*""#])
        {
            Ok(regex)=> regex,
            Err(msg) => panic!("invalid regex: {}",msg)
        };
        
        let results = find_comments(TEXT_MULTILINE,&dfa); 
        assert_eq!(*results,["/* todo    (value)  {desc} todo(value){desc}\ntodo(value){desc}*/"]);
        
        let vm = PikeVM::new(r"(?i)todo\s*\((?P<val>.*?)\)\s*\{(?P<desc>.*?)\}")
            .expect("bad regex err");
        
        let line = find_todo(results, &vm);
        assert_eq!(line.len(),3);
    }
    
}
