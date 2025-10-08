use std::{fs, path::PathBuf};
use rayon::str::CharIndices;

use miniregex::graph::{CursorResult, FSA};
use crate::{startup::{CommentGraphs, LexerGraphs}};



enum Token {
    Keyword,
    Scope,
    Left,
    Right,
}




pub struct Lexer<'a> {
    comment_rules: FSA,
    parser_rules:  &'a LexerGraphs
}
impl<'a> Lexer<'a> {
    pub fn new(ext: &str, comment: &'a CommentGraphs, lexer: &'a LexerGraphs)->Option<Self> {

        let Ok(rule) = comment.get_graph(ext) else {
                eprintln!("WARN enable to make lexer");
                return None
        };
    
        Some(Lexer {
            comment_rules: rule,
            parser_rules:lexer
        })
    }
    //checks all character if it hits the end of the file in a valid state it will count it as
    //valid
    pub fn get_lexic<'b>(self,str:&'b str)->Comments<'b>{
        //--------------get comments----------------------//
        let mut iter = str.char_indices();
        let mut comment_cursor = self.comment_rules.cursor();
        
        let mut comments = vec![];      
        let mut start = None;

        while let Some((pos, c)) = iter.next() {
            match comment_cursor.match_eq(c){
                CursorResult::Valid  => {
                    start.is_none().then(||start = Some(pos));
                }
                CursorResult::Invalid=> {
                    start = None;
                    comment_cursor = self.comment_rules.cursor();
                }
                CursorResult::Match  => {
                    let Some(start) = start else {
                        unreachable!("[FATAL] unreachable code start should be initiated before reaching match");
                    };

                    comments.push(Comment {
                        abs_start:start,
                        abs_end  :pos,
                        comment  :&str[start..=pos]
                    });
                    comment_cursor = self.comment_rules.cursor();
                }
            }
        }
        if let Some(start) = start { //if in a valid state grabs last character
            let end = str.len();
            comments.push(Comment {
                        abs_start:start,
                        abs_end  :end,
                        comment  :&str[start..end]
                    });
        }
        Comments(comments.into_iter()) 
    }
}




//data with logic to make manipulation easy
struct Comment<'a> {
    pub abs_start :usize,
    pub abs_end   :usize,
    pub comment  :&'a str,
}
struct Comments<'a> (std::vec::IntoIter<Comment<'a>>);
impl<'a> Comments<'a> {
    fn iter(self) {
       let iter = self.0.iter(); 
    }
}





