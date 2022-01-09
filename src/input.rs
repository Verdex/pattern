
use std::str::CharIndices;
use std::iter::Peekable; 

#[derive(Clone)]
pub struct Input<'a> {
    cs : Peekable<CharIndices<'a>>
}

impl<'a> Input<'a> {
    pub fn new(s : &str) -> Input {
        Input { cs: s.char_indices().peekable() }
    }

    fn next(&mut self) -> Option<char> {
        match self.cs.next() {
            Some((_, c)) => Some(c),
            None => None,
        }
    }

    fn peek(&mut self) -> Option<&char> {
        match self.cs.peek() {
            Some((_, c)) => Some(c),
            None => None,
        }
    }
}