use regex_automata::{dfa::regex::Regex, nfa::thompson::pikevm::PikeVM};


pub fn comments(content:& str, regex: Regex)->Vec<& str> {
    let mut comments = vec![];

    for m in regex.find_iter(content.as_bytes()) {
        let pattern_id = m.pattern().as_usize();

        match pattern_id {
            0 => comments.push(&content[m.range()]),
            1 => {}, // String - ignore
            _ => (),
        }
    }
    comments 
}

pub fn find_todo(comments: Vec<&str>,vm:PikeVM) {
    let mut cache = vm.create_cache();
    
    for comment in comments.iter() {
        let bytes = comment.as_bytes();

        for caps in vm.captures_iter(&mut cache, bytes) {
            
            let category = caps.get_group(1).map(|m| &comment[m.range()]);    
            let task     = caps.get_match(2).map(|m| &comment[m.range()]);

            if let (Some(cat), Some(tsk)) = (category, task) {
                println!("Found -> [Category: {}] [Task: {}]", cat, tsk);
            }
        }


    }
} 

#[cfg(test)]
mod test { 

    
    const TEXT_LINE: &str = "// todo    (value)  {desc} todo(value){desc}\ntodo(value){desc}";
    const TEXT_MULTILINE: &str = "/* todo    (value)  {desc} todo(value){desc}\ntodo(value){desc}*/todo(value){desc}";
    
 /*   fn find_todo(parser:&mut TodoParser, iter: &mut Chars)->Option<String> 
    {
        while let Some(c) = iter.next() 
        { 
           let Some(str) = parser.next(&c) else { continue };
            return Some(str);
        }
        None
    }
    #[test]
    pub fn parsing_line_comment() 
    {
        let mut parser = TodoParser::new(&comment_nfa, &todo_nfa);
        let mut iter = TEXT_LINE.chars();

        let mut todo = find_todo(&mut parser, &mut iter);
        assert_eq!(Some("todo    (value)  {desc}"),todo.as_ref().map(|t| t.as_str()));
        
        todo = find_todo(&mut parser, &mut iter);
        assert_eq!(Some("todo(value){desc}"),todo.as_ref().map(|t| t.as_str()));

        todo = find_todo(&mut parser, &mut iter);
        assert_eq!(None,todo.as_ref().map(|t| t.as_str()));
    }

    #[test]
    pub fn parsing_multiline_comment() {
        let mut parser = TodoParser::new(&comment_nfa, &todo_nfa);
        let mut iter = TEXT_MULTILINE.chars();

        let mut todo = find_todo(&mut parser, &mut iter);
        assert_eq!(Some("todo    (value)  {desc}"),todo.as_ref().map(|t| t.as_str()));
        
        todo = find_todo(&mut parser, &mut iter);
        assert_eq!(Some("todo(value){desc}"),todo.as_ref().map(|t| t.as_str()));
 
        todo = find_todo(&mut parser, &mut iter);
        assert_eq!(Some("todo(value){desc}"),todo.as_ref().map(|t| t.as_str()));

        todo = find_todo(&mut parser, &mut iter);
        assert_eq!(None,todo.as_ref().map(|t| t.as_str()));
    }*/

}
