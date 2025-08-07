pub mod iterators;
use std::{collections::{VecDeque}, str};

use crate::comment_parser::iterators::*;


pub struct CommentsQueue<'a>(VecDeque<Comments<'a>>);

impl<'a> CommentsQueue<'a> {
    pub fn new()-> Self {
        CommentsQueue(VecDeque::new())
    }
    
    pub fn dequeue(&mut self)->Option<Comments<'a>> {
        self.0.pop_front()
    }
    
    pub fn queue(&mut self,comment:Comments<'a>) {
       self.0.push_back(comment); 
    }

    pub fn as_ref(&self)->&VecDeque<Comments<'a>> {
       &self.0 
    }

    pub fn iter(&'a self)-> Option<IterCommentsQueueStr<'a>> {
        let iter = IterCommentsQueueStr::new(&self.0)?;
        Some(iter)
    }

}




#[derive(Debug,PartialEq)]
pub struct Comments<'a> {
    range: (usize, usize),
    str: &'a str,
}
impl<'a> Comments<'a> {
    pub fn as_str(&self)-> &str {
        self.str
    }

    pub fn as_absolute_position(&self, i:usize)->usize {
        self.range.0 + i 
    }


    pub fn iter(&self) -> IterCommentStr {
        IterCommentStr {range: self.range, iter: self.as_str().char_indices()}
    }
}



    
pub fn parse<'a>(text: &'a str) -> CommentsQueue<'a> {
    let mut parsed_text = CommentsQueue::new();
    let mut iter = text.char_indices().peekable();


    while let Some((_, ch)) = iter.next() {
        let ch2 = iter.by_ref().peek();
        match(ch, ch2){
            ('/',Some(&(start_pos,'/')))=>{
                let start = start_pos+1;
                let mut end = start_pos;
                for _ in iter.by_ref().take_while(|(_,c)| *c!='\n') {
                    end += 1;
                }
                let com = Comments {
                    range: (start,end),
                    str:&text[start..end]
                };
                parsed_text.queue(com);
            },
            ('/',Some(&(start_pos,'*')))=>{
                let start = start_pos + 1;
                let mut end = start;
                loop {
                    for _ in iter.by_ref().take_while(|(_,c)|*c!='*') {
                        end += 1; 
                    };
                    match iter.peek() {
                        Some((_,'/'))=>break,
                        None => break,
                        _ => continue,
                    }
                }                
                let com = Comments {
                    range: (start,end),
                    str:&text[start..end]
                };
                parsed_text.queue(com);
            },
            _ =>(),
        }
    }
        parsed_text
}
 

#[cfg(test)]
mod test {
    use super::*; 
    #[test]
    pub fn parsed_comment_line() {
        let content = "pub fn test(/*block\nblock2*/ x:i32 ) {}";

        let mut list = parse(content);     

        let comment = list.dequeue().unwrap();    
        assert_eq!("block\nblock2", comment.as_str());
 
        let comment = list.dequeue();   
        assert_eq!(None, comment);       
    }
    
    #[test]
    pub fn parsed_comment_block() {
        let content = "//pub fn test(\nblock\n //x:i32 ) {}";
        let mut list = parse(content);

        let comment = list.dequeue().unwrap();
        assert_eq!("pub fn test(",comment.as_str());

        let comment = list.dequeue().unwrap();
        assert_eq!("x:i32 ) {}",comment.as_str());
    }

}

