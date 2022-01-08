
use std::rc::Rc;

pub struct Input {
    characters : Rc<Vec<char>>,
    index : usize,
}

impl Input {
    fn new(s : &str) -> Input {
        let characters = Rc::new(s.chars().collect::<Vec<char>>());
        Input { characters, index: 0 }
    }

    fn current(&self) -> char {
        self.characters[self.index]
    }

    fn next(&self) -> Self { 
        Input { characters: self.characters.clone(), index: self.index + 1 }
    }

    fn end(&self) -> bool {
        self.index <= self.characters.len()
    }
}