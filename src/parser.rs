
use crate::input::{Input, ParseError};
use crate::ast::{Expr, Ast};

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

fn parse_symbol(input : &mut Input) -> Result<String, ParseError> {
    parse_junk(input)?;

    let mut cs = vec![];

    match input.peek() {
        Ok(c) if c.is_alphabetic() || c == '_' => { cs.push(c); input.next(); },
        Err(e @ ParseError::Fatal(_)) => return Err(e),
        _ => return Err(ParseError::Error),
    }

    loop {
        match input.peek() {
            Ok(c) if c.is_alphanumeric() || c == '_' => { cs.push(c); input.next(); },
            Err(e @ ParseError::Fatal(_)) => return Err(e),
            _ => return Ok(cs.into_iter().collect::<String>()),
        }
    }
}

fn parse_number(input : &mut Input) -> Result<Expr, ParseError> {
    parse_junk(input)?;

    let mut cs = vec![];
    let mut negative = false;

    match input.peek() {
        Ok(c) if c.is_ascii_digit() => { cs.push(c); input.next(); },
        Ok(c) if c == '-' => { negative = true; input.next(); },
        Err(e @ ParseError::Fatal(_)) => return Err(e),
        _ => return Err(ParseError::Error),
    }

    loop {
        match input.peek() {
            Ok(c) if c.is_ascii_digit() => { cs.push(c); input.next(); },
            Err(e @ ParseError::Fatal(_)) => return Err(e),
            _ if cs.len() < 1 => return Err(ParseError::Fatal("encountered single '-'".to_string())),
            _ if negative => return Ok(Expr::Number(cs.into_iter().collect::<String>().parse::<i64>().expect("Internal Rust Parse Error") * -1)),
            _ => return Ok(Expr::Number(cs.into_iter().collect::<String>().parse::<i64>().expect("Internal Rust Parse Error"))),
        }
    }
}

fn parse_literal(input : &mut Input) -> Result<(), ParseError> {
    Err(ParseError::Fatal("Problem".to_string()))
}

fn parse_expr(input : &mut Input) -> Result<(), ParseError> {
    Err(ParseError::Fatal("Problem".to_string()))
}

fn parse_top_level(input : &mut Input) -> Result<(), ParseError> {
    Err(ParseError::Fatal("TODO".to_string()))
}

#[cfg(test)]
mod test {
    use super::*;

    use std::num::ParseIntError;

    #[test] 
    fn should_parse_positive_int() -> Result<(), ParseError> {
        let mut input = Input::new("1234");
        let result = parse_number(&mut input)?;

        assert!( matches!( result, Expr::Number(1234) ) );

        Ok(())
    }

    #[test] 
    fn should_parse_negative_int() -> Result<(), ParseError> {
        let mut input = Input::new("-1234");
        let result = parse_number(&mut input)?;

        assert!( matches!( result, Expr::Number(-1234) ) );

        Ok(())
    }

    #[test]
    fn should_parse_single_underscore_symbol() -> Result<(), ParseError> {
        let mut input = Input::new("_");
        let result = parse_symbol(&mut input)?;

        assert_eq!( result, "_" );

        Ok(())
    }

    #[test]
    fn should_parse_single_character_symbol() -> Result<(), ParseError> {
        let mut input = Input::new("a");
        let result = parse_symbol(&mut input)?;

        assert_eq!( result, "a" );

        Ok(())
    }

    #[test]
    fn should_parse_symbol() -> Result<(), ParseError> {
        let mut input = Input::new("blah_1234");
        let result = parse_symbol(&mut input)?;

        assert_eq!( result, "blah_1234" );

        Ok(())
    }
}