use std::{collections::vec_deque, str};

use crate::comment_parser::*;


/// iterator that allows to grab a char from a str
/// and return its absolute position
pub struct IterCommentStr<'a>{
    pub range: (usize,usize),
    pub iter: str::CharIndices<'a>
}

impl<'a> IterCommentStr<'a> {
    fn absolute_pos(&self, relative_pos:usize) -> usize {
        self.range.0 + relative_pos
    }
}

impl<'a> Iterator for IterCommentStr<'a> {
    type Item =(usize, char);

    fn next(&mut self) ->Option<Self::Item> {
        let (pos, ch) = self.iter.next()?;
        Some((self.absolute_pos(pos), ch))
    }
}


pub struct IntoIterCommentsQueue<'a>(CommentsQueue<'a>);

impl<'a> Iterator for IntoIterCommentsQueue<'a> {
    type Item = Comments<'a>;

    fn next(&mut self)->Option<Self::Item> {
        self.0.dequeue() 
    }
}


pub struct IterCommentsQueueStr<'a>{
    current: IterCommentStr<'a>,
    queue  : vec_deque::Iter<'a, Comments<'a>>
}

impl<'a> IterCommentsQueueStr<'a> {
    pub fn new(vec: &'a VecDeque<Comments<'a>>) -> Option<Self> {
        let mut iter = vec.iter();
        let comment = iter.next()?;
        Some(IterCommentsQueueStr { current:comment.iter(), queue: iter })
    }
}
//returns absolute position
impl<'a> Iterator for IterCommentsQueueStr<'a> {
    type Item = (usize, char);

    fn next(&mut self)->Option<Self::Item> {
        loop {
            if let Some(c) = self.current.next() {
                return Some(c);
            } else {
                match self.queue.next() {
                    Some(comment) =>{
                        self.current = comment.iter();

                    },
                    None => return None,
                }
            }

        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_iter() {
        let content = "//pub fn test(\nblock\n//x:i32 ) {}";
        let queue = parse(content);
        
        assert_eq!(2,queue.as_ref().len()); //lenght is supposed to be 2
        
        let Some(mut iter) = queue.iter() else {panic!("what") };
                    // /                            0 
                    // /                            1
        iter.next();//p                             2
        iter.next();//u                             3
        iter.next();//b                             4
        iter.next();//                              5
        iter.next();//f                             6
        iter.next();//n                             7
        iter.next();//                              8
        let letter = iter.next().unwrap(); //       9
        assert_eq!((9,'t'),letter);
        
        iter.next(); // e                           10
        iter.next(); // s                           11
        iter.next(); // t                           12
        iter.next(); // (                           13
                     // \n                          14
                     // b                           15
                     // l                           16
                     // o                           17
                     // c                           18
                     // k                           19
                     // \n                          20
                     // /                           21
                     // /                           22
        iter.next(); // x                           23
        iter.next(); // :                           24
        let letter = iter.next().unwrap(); // i     25
        assert_eq!((25, 'i'), letter);

        assert_eq!(2,queue.as_ref().len()); //since its ref lenght is supposed to stay two
    }
}
