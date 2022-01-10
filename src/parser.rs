
use crate::input::{Input, ParseError};
use crate::ast::Ast;

pub fn parse(input : Input) -> Result<Ast, ParseError> {
    Err(ParseError::Fatal("Problem".to_string()))
}

fn parse_junk(input : &mut Input) -> Result<(), ParseError> {

    let mut comment = false;

    loop {

        match (comment, input.peek()) {
            (true, Ok('\n')) => { comment = false; input.next(); }
            (true, Ok('\r')) => { comment = false; input.next(); }
            (true, Ok(_)) => { input.next(); }
            (false, Ok(c)) if c.is_whitespace() => { input.next(); },
            (false, Ok('#')) => { comment = true; input.next(); },
            (false, Ok(c)) => return Ok(()), 
            (_, Err(ParseError::Error)) => return Ok(()),
            (_, Err(e @ ParseError::Fatal(_))) => return Err(e),
        }

    }

}

fn parse_literal(input : &mut Input) -> Result<(), ParseError> {
    Err(ParseError::Fatal("Problem".to_string()))
}

fn parse_expr(input : &mut Input) -> Result<(), ParseError> {
    Err(ParseError::Fatal("Problem".to_string()))
}