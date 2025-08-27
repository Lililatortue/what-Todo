use miniregex::graph::GraphCursor;
use std::collections::HashMap;
use crate::parser::regex_generator::{LexerGraphs, PARSER_NFA};

pub struct Parser {
    parser_lexer: LexerGraphs,
}

impl Parser {
    pub fn new()->Self {
        let target = PARSER_NFA;
        Parser{parser_lexer: PARSER_NFA} 
    }
        


}
