use std::{iter, str};
use std::collections::VecDeque;


pub struct TextParser<'a>{
    state: State,
    iter : iter::Peekable<str::CharIndices<'a>>,
}

impl<'a> TextParser<'a> {

    pub fn new(s:&'a str)->Self{
        TextParser{state: State::Text ,iter: s.char_indices().peekable()}
    }
    pub fn parse(mut self) -> ParsedText {
        let text: ParsedText;
        while let Some((pos, ch)) = self.iter.next() {
            let ch2 = self.iter.peek();
            match(ch, ch2){
                ('/',Some((_,'/')))=>{},
                ('/',Some((_,'*')))=>{


                },
                ('"',_)=>{



                },
            }

        }
        text
    }

}

enum State{
    Text,
    Line(String),
    Block(String),
    String(String),
}

impl State {
    /// disgusting state machine 
    /// assures transition of states and moves the string out of the function 
    /// to improve and make more readable
    fn state_transition(&mut self,pattern: (char ,Option<char>))->Option<BlockType> {        
        match self {
            State::Text(text) if pattern == ('/',Some('*'),) =>{
                let t = std::mem::take(text);
                *self = State::Block(String::new());
                match t.is_empty() {
                    false =>Some(BlockType::Code(t)),
                    true =>None,
                }
           },
            State::Text(text) if pattern == ('/',Some('/')) =>{
                let t = std::mem::take(text);
                *self = State::Line(String::new());
                match t.is_empty() {
                    false =>Some(BlockType::Code(t)),
                    true =>None,
                }
            },
            State::Text(text) if pattern == ('"',None) =>{
                let t = std::mem::take(text);
                *self = State::String(String::new());
                match t.is_empty() {
                    false =>Some(BlockType::Code(t)),
                    true =>None,
                }
            },
            State::Text(text) if pattern == ('\n',None) =>{
                let t = std::mem::take(text);
                *self = State::Text(String::new());
                match t.is_empty() {
                    false =>Some(BlockType::Code(t)),
                    true =>None,
                }
            },

            State::Line(text) if pattern == ('\n',None) =>{
                let t = std::mem::take(text);
                *self = State::Text(String::new());
                match t.is_empty() {
                    false =>Some(BlockType::Comment(t)),
                    true=>None,
                }
            },

            State::Block(text) if pattern == ('*',Some('/')) =>{
                let t = std::mem::take(text);
                *self = State::Text(String::new());
                match t.is_empty() {
                    false =>Some(BlockType::Comment(t)),
                    true=>None,
                }
            },
            State::Block(text) if pattern == ('\n',None) =>{
                let t = std::mem::take(text);
                *self = State::Block(String::new());
                match t.is_empty() {
                    false =>Some(BlockType::Comment(t)),
                    true=>None,
                }
            },

            State::String(text) if pattern == ('"',None) =>{
                let t = std::mem::take(text);
                *self = State::Text(t);
                None
            },
            State::String(text) if pattern == ('\n',None) =>{
                let t = std::mem::take(text);
                *self = State::String(String::new());
                match t.is_empty() {
                    false =>Some(BlockType::Code(t)),
                    true =>None,
                }
            },
            State::String(text) =>{
                let mut t = std::mem::take(text);
                t.push(pattern.0);
                if let Some(c) =pattern.1 {
                    t.push(c);
                }
                *self = State::Text(t);
                None
            },

            _ => None
        }
    }


    fn push_c(&mut self, c: char) {
        match self {
            State::Text(s)
            | State::Line(s)
            | State::Block(s)
            | State::String(s) => {
                s.push(c);
            }
        }
    }
}





#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    pub fn parsed_comment_line() {
        let content = "pub fn test(/*block*/ x:i32 ) {}";

        let mut parser = TextParser::new(content.chars().peekable());      
        let mut text = parser.parse();
 
        let mut line = text.dequeue().unwrap();
        let token = line.pop_front().unwrap();
        assert_eq!(BlockType::Code("pub fn test(".to_string()), token);

        let token = line.pop_front().unwrap();
        assert_eq!(BlockType::Comment("block".to_string()), token);       
        
        let token = line.pop_front().unwrap();
        assert_eq!(BlockType::Code(" x:i32 ) {}".to_string()), token); 
    }
    
    #[test]
    pub fn parsed_comment_block() {
        let content = "/*pub fn test(\nblock\n x:i32 ) {}*/";

        let mut parser = TextParser::new(content.chars().peekable());      
        let mut text = parser.parse();
 
        let mut line = text.dequeue().unwrap();
        let token = line.pop_front().unwrap();
        assert_eq!(BlockType::Comment("pub fn test(".to_string()), token);

        let mut line = text.dequeue().unwrap();
        let token = line.pop_front().unwrap();
        assert_eq!(BlockType::Comment("block".to_string()), token);       

        let mut line = text.dequeue().unwrap();
        let token = line.pop_front().unwrap();
        assert_eq!(BlockType::Comment(" x:i32 ) {}".to_string()), token); 
    }
}
