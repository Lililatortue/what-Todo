use miniregex::{Nfa,LazyDfa, cursor::DfaResult};

///find's todo's

pub struct TodoParser<'a> {
    comment_dfa: LazyDfa<'a>,   
    todo_dfa:    LazyDfa<'a>,
    todo_str: Option<String>,
}

impl<'a> TodoParser<'a> {
    pub fn new(comment_nfa: &'a Nfa, todo_nfa: &'a Nfa)-> Self {
        TodoParser 
        {
            comment_dfa: comment_nfa.lazy(),
            todo_dfa:    todo_nfa.lazy(),
            todo_str: None,
        }
    }

    pub fn next(&mut self,c: &char)-> Option<String> {
        let result1 = self.comment_dfa.next(*c);   
        let result2 = self.todo_dfa.next(*c);
        //LazyDfa can return 3 results Invalid, Valid, Match
        //comment_dfa has the responsibility to check wether we are in a comment or code
        //so it doesnt matter if the string is valid or not it just needs to know when we are in
        //one 
        if result1 == DfaResult::Invalid || result1 == DfaResult::Match 
        {
            let s = self.todo_str.take();

            match result2
            {
                DfaResult::Invalid => None,
                DfaResult::Valid   => None,
                DfaResult::Match   => s,
            }
        }
        else 
        {
        //now comment_dfa is in valid state
            match result2 
            {
                DfaResult::Invalid => 
                {
                    self.todo_str = None;
                    None
                },
                DfaResult::Valid   => 
                {
                    self.todo_str.get_or_insert(String::new()).push(*c);
                    None
                },
                DfaResult::Match   => self.todo_str.take()
                                                   .map(|mut s| {s.push(*c); s}),
            }
        }
    }
}

#[cfg(test)]
mod test { 
    use miniregex::make_nfa;
    use std::str::Chars;
    use super::*;

    
    const TEXT_LINE: &str = "// todo    (value)  {desc} todo(value){desc}\ntodo(value){desc}";
    const TEXT_MULTILINE: &str = "/* todo    (value)  {desc} todo(value){desc}\ntodo(value){desc}*/todo(value){desc}";
    
    fn find_todo(parser:&mut TodoParser, iter: &mut Chars)->Option<String> {
        while let Some(c) = iter.next() { 
           let Some(str) = parser.next(&c) else { continue };
            return Some(str);
        }
        None
    }

    #[test]
    pub fn parsing_line_comment() {
        let comment_nfa = make_nfa!(r"(// .* \n) | (/\* .* \*/ )");
        let todo_nfa    = make_nfa!(r"todo \s* \( .* \) \s* \{ .* \}");

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
        let comment_nfa = make_nfa!(r"(// .* \n) | (/\* .* \*/ )");
        let todo_nfa    = make_nfa!(r"todo \s* \( .* \) \s* \{ .* \}");

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

    }

}
