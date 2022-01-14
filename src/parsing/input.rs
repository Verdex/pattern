
use std::str::CharIndices;
use std::iter::Peekable; 

#[derive(Clone)]
pub struct Input<'a> {
    cs : Peekable<CharIndices<'a>>
}

#[derive(Debug)]
pub enum ParseError {
    Error,
    Fatal(Vec<String>),
}

impl ParseError {
    pub fn display(&self) -> String {
        match self {
            ParseError::Error => "Error".to_string(),
            ParseError::Fatal(fs) => format!( "Fatal:\n {}", fs.join("\n") ),
        }
    }
}

impl<'a> Input<'a> {
    pub fn new(s : &str) -> Input {
        Input { cs: s.char_indices().peekable() }
    }

    pub fn restore(&mut self, r : Input<'a>) {
        self.cs = r.cs;
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