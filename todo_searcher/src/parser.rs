use std::{iter, str};
use std::collections::VecDeque;
/// This iterator as a state to track multi-line comments
/// depending on its state it will right different Tokens
/// 
/// main goal seperate content from comment to text 
/// because of this we can do:
///
/// // TODO (var) {
/// pub fn thing(){}
/// // }
/// now the function can be in the description of TODO






pub struct TextParser<'a>{
    state: State,
    iter: iter::Peekable<str::Chars<'a>>
}

impl<'a> TextParser<'a> {
    pub fn new(iter: iter::Peekable<str::Chars<'a>>)->Self {
        TextParser { 
            state: State::Text(String::new()),
            iter 
        } 
    }
    
   pub fn parse(&mut self)-> ParsedText {
        let mut block: Block = VecDeque::new();
        let mut parsed_text = ParsedText(VecDeque::new()); 
        while let Some(c) = self.iter.next(){
            let c2 =self.iter.peek();
            match (c,c2){
                ('/',Some('/')) | ('/', Some('*')) | ('*',Some('/'))=>{
                    let c2 = self.iter.next();

                    if let Some(t) = self.state.state_transition((c,c2)) {
                        block.push_back(t);
                    }  
                },
            
                ('"',_) =>{
                    if let Some(s) = self.state.state_transition((c,None)){
                        block.push_back(s);
                    }
                },

                ('\n',_)=>{
                    if let Some(t) = self.state.state_transition((c,None)) {
                        block.push_back(t);
                    }
                    let old = std::mem::take(&mut block); 
                    parsed_text.queue(old);                           
                },

                _=> self.state.push_c(c),
            };
        }
        //
        if let Some(s) = self.state.state_transition(('\n',None)){
            block.push_back(s);
        }
        parsed_text.queue(block);
        parsed_text
    } 
}

#[derive(Debug, PartialEq)]
pub enum BlockType {
    Comment(String),
    Code(String)
}
impl BlockType {
    
    pub fn as_str(&self) ->&str{
        match self {
            BlockType::Comment(s) |
            BlockType::Code(s) => s
        }
    }
}
enum State{
    Text(String),
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



/// reads a file line and sequentially 
/// push content of diffente types
type Block = VecDeque<BlockType>;

/// structs that seperates every lines into blocks
/// Blocks can be either code or comments
pub struct ParsedText(VecDeque<Block>);
impl ParsedText {
    pub fn dequeue(&mut self)->Option<Block>{
        self.0.pop_front()
    }
    pub fn queue(&mut self, blocks:Block){
       self.0.push_back(blocks); 
    }
    
    pub fn into_iter(self)-> IntoIterParsedText{
        IntoIterParsedText(self)
    }
}
pub struct IntoIterParsedText(ParsedText);
impl Iterator for ParsedText {
    type Item = BlockType;
    fn next(&mut self) ->Option<Self::Item> {  
        loop {
            match self.0.front_mut() {
                Some(block) if block.is_empty() => {
                    self.0.pop_front();
                }
                Some(block) => {
                   return block.pop_front(); 
                }
                None => return None,
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
