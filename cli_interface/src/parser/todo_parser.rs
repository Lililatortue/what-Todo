///gets the position of '(' and ')' and '{' '}'
///as a rule if the key word todo is in between  '()' or '{}'
///it invalidates it for now invalidating means ignored 
///might add it to a special list if i see necessary
/// so format would be:
/// todo ' any amount of space' '()' 'any amount of space' '{}' all within the comments
use crate::parser::comment_parser::iterators::{IterCommentsQueueStr};
const PATTERN:[char;3] = ['o','d','o'];


pub struct TodoStrBuilder<'a>(pub IterCommentsQueueStr<'a>);


impl<'a> TodoStrBuilder<'a> {
    pub fn find_todo(&mut self)->bool {
        while let Some(ch) = self.0.buffered_next(){
            match ch.1 {
                't' | 'T' =>{
                    //does consume buffer
                    if self.0.check_pattern(|iter|iter.buffered_next(), PATTERN.to_vec()) {
                        self.0.clear_buffer();
                        return true;
                    }

                }
                _=>continue,
            }
        }
        false
    }


    pub fn get_var(&mut self)-> Option<(usize, usize)> {
        while let Some(start) = self.0.buffered_next() {
            match start.1 {
                '(' => { 
                    while let Some(end) = self.0.buffered_next() {
                        match end.1 {
                            't' | 'T' =>{
                                //consumes buffer
                                if self.0.check_pattern(|iter|iter.look_forward(),PATTERN.to_vec()){ 
                                    self.0.put_front_buffer(end);
                                    return None;
                                }
                            },
                            ')' =>{return Some((start.0+1,end.0))},
                             _  => continue,
                        }
                      
                    } 
                },

                |'\t'|' '|'\n' => continue,

                _ => break,   
            }
        }  
        None
    }


    pub fn get_desc(&mut self)-> Option<(usize, usize)> {
        while let Some(start) = self.0.buffered_next() {
            match start.1 {
                '{' => {
                    while let Some(end) = self.0.buffered_next() {
                        match end.1 {
                            't' | 'T' =>{
                                //consumes the buffer 
                                if self.0.check_pattern(|iter|iter.look_forward(),PATTERN.to_vec()){
                                    self.0.put_front_buffer(end);
                                    return None
                                }
                            },
                            '}' => return Some((start.0+1,end.0)),
                             _  => continue,
                        }
                    }
                },

                ' '|'\n' => continue,

                _ => break,   
            }
        }  
        None
    }  
}




#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::{comment_parser};
    /// 
    #[test]
    pub fn todo_parser() {
let text = r#"
// TODO(UI) {implement parser}
let x = 5;
// This one should also match: TODO(helper) {clean up logic}
/* TODO(math) {optimize equation} */

// ttodo(bad) {should not match}
// TODO() {no variable} —-> valid
// TODO(missing desc) — invalid
// TODO{missing parens} — invalid
// just some comment without todo
/* random block with TODO but no structure */
// tOdO(typo) {ok} — valid if case-insensitive and structure ok
// TODO  (   spaced   )   {  spaced out description  }
// Todo (UI) { please change the name of this function:
    pub fn CHANGE_MY_NAME(x:i32, y:i32)->i32{
        x+y
    }  
// }
let y = (TODO); // not inside comment → invalid
"#;


        let comments = comment_parser::parse(text);

        let iter = match comments.iter() {
            Some(content) => content,
            None =>{ eprintln!("empty text");
                     return;},
        };
        
        let mut builder = TodoStrBuilder(iter);
        loop {
        
              if !builder.find_todo() {
                break;
              }
         
            let var = match builder.get_var(){
                Some(content)=> content,
                None  =>continue,
            };
            let desc = match builder.get_desc(){
                Some(content)=>content,
                None  =>continue,
            };
              println!("todo:\n var= {}\n desc= {}\n",&text[var.0..=var.1],&text[desc.0..=desc.1]);
        }
    }

}



