
///gets the position of '(' and ')' and '{' '}'
///as a rule if the key word todo is in between  '()' or '{}'
///it invalidates it for now invalidating means ignored 
///might add it to a special list if i see necessary
/// so format would be:
/// todo ' any amount of space' '()' 'any amount of space' '{}' all within the comments
use crate::comment_parser::iterators::{IterCommentsQueueStr};
const PATTERN:[char;3] = ['o','d','o'];


pub struct TodoStrBuilder<'a>(IterCommentsQueueStr<'a>);


impl<'a> TodoStrBuilder<'a> {
    pub fn build(mut self)->TodoStrList<'a> {
        let mut list = TodoStrList::new();
        loop {
           if self.find_todo()==true {
                break;
           };
           let Some(var) = self.get_var()   else {
                continue;
           };
           let Some(desc) = self.get_desc() else {
                continue;
           };
           list.0.push(TodoStr { var: var, desc: desc });
        }
        list
    }


    fn find_todo(&mut self)->bool {
       false
    }


    fn get_var(&mut self)-> Option<&'a str> {
       None 
    }


    fn get_desc(&mut self)-> Option<&'a str>{
        None
    }    

}

pub struct TodoStrList<'a>(Vec<TodoStr<'a>>);
impl<'a> TodoStrList<'a> {
    pub fn new()->Self {
        TodoStrList(Vec::new())
    }
}


pub struct TodoStr<'a> {
    var: &'a str,
    desc: &'a str,
}


