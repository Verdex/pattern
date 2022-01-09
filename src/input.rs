
use std::str::CharIndices;
use std::iter::Peekable; 

#[derive(Clone)]
pub struct Input<'a> {
    cs : Peekable<CharIndices<'a>>
}

pub enum ParseError {
    Error,
    Fatal(String),
}

impl<'a> Input<'a> {
    pub fn new(s : &str) -> Input {
        Input { cs: s.char_indices().peekable() }
    }

    pub fn next(&mut self) -> Result<char, ParseError> {
        match self.cs.next() {
            Some((_, c)) => Ok(c),
            None => Err(ParseError::Error),
        }
    }

    pub fn peek(&mut self) -> Result<char, ParseError> {
        match self.cs.peek() {
            Some((_, c)) => Ok(*c),
            None => Err(ParseError::Error),
        }
    }
}