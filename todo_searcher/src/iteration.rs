mod state;
use std::{iter, str};

pub struct TodoIterator<'a> {
    buffer: Option<char>,
    iter: iter::Peekable<str::Chars<'a>>,
}


impl<'a> Iterator for TodoIterator<'a> {
    type Item = char;

    fn next(&mut self)->Option<Self::Item>{
        if let Some(c) = self.buffer.take() {
           return Some(c)
        }
        self.buffer.take()
    }
}


impl<'a> TodoIterator<'a> {
    
    pub fn new(s:&'a str)-> Self {
        TodoIterator{ buffer:None, iter: s.chars().peekable() }
    }
 
    pub fn take_while_strict<P>(&mut self, mut predicate: P)-> Option<c>
        where P:FnMut(char)->bool
    {
        let &mut str;
        while let Some(c) = self.iter.next() {
            if c == 't' || c =='T' { 
                if self.check_if("odo") {
                    self.buffer = Some(c);
                    break;
                }
            }
            if predicate(c) {
                s.push(c);
            } else {
                return Some(s) 
            }
        }
        None
    }
    fn check_if(&mut self, s:&str)->bool {
        let mut i = self.iter.clone();
        s.chars().zip(&mut i).all(|(i,a)| i==a) 
    }
}

