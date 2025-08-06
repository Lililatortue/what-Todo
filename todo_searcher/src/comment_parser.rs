use std::{collections::VecDeque, str};

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
    pub fn as_ref(&'a self)-> &VecDeque<Comments<'a>>{
        &self.0
    }
    pub fn iter(&'a self)->IterCommentsQueue<'a> {
        IterCommentsQueue{pos: 0, queue:&self.0}
    }
}

pub struct IterCommentsQueue<'a> { 
    pos: usize,
    queue: &'a VecDeque<Comments<'a>>
}

impl<'a> Iterator for IterCommentsQueue<'a> {
    type Item = &'a Comments<'a>;

    fn next(&mut self)->Option<Self::Item> {
        if self.pos >= self.queue.len() {
            return None 
        }
        let ptr = &self.queue[self.pos]; 
        self.pos +=1;
        Some(ptr)
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



    #[test]
    pub fn test_iter() {
        let content = "\
//pub fn test(
block
//x:i32 ) {}";
        let queue = parse(content);
        
        assert_eq!(2,queue.as_ref().len()); //lenght is supposed to be 2
        
        let mut iter = queue.iter();
        let comment = iter.next().unwrap();
        assert_eq!("pub fn test(",comment.as_str());

        let comment = iter.next().unwrap();
        assert_eq!("x:i32 ) {}",comment.as_str());
        
        let comment = iter.next();
        assert_eq!(None,comment);

        assert_eq!(2,queue.as_ref().len()); //since its ref lenght is supposed to stay two
    }
}

