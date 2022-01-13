
use super::input::{Input, ParseError};
use super::util::{ into
                 , parse_junk
                 , parse_symbol
                 , keyword
                 , punct
                 , maybe
                 , fatal
                 };
use crate::ast::Type;

fn parse_generic_type(input : &mut Input) -> Result<Type, ParseError> {
    let rp = input.clone();

    let sym = parse_symbol(input)?;

    let first = sym.chars().nth(0)
        .expect("parse_generic_type parse_symbol somehow returned zero length string");

    if first.is_lowercase() {
        Ok(Type::Generic(sym))
    }
    else {
        input.restore(rp);
        Err(ParseError::Error)
    }
}

fn parse_concrete_type(input : &mut Input) -> Result<Type, ParseError> {
    let rp = input.clone();

    let sym = parse_symbol(input)?;

    let first = sym.chars().nth(0)
        .expect("parse_concrete_type parse_symbol somehow returned zero length string");

    if first.is_uppercase() {
        Ok(Type::Concrete(sym))
    }
    else {
        input.restore(rp);
        Err(ParseError::Error)
    }
}

fn parse_fun_type(input : &mut Input) -> Result<Type, ParseError> {
    parse_junk(input)?;

    keyword(input, "fun")?;

    fatal(punct(input, "("), "fun type must have opening '('")?;

    // TODO list of Type [,] until ')'

    Err(ParseError::Fatal("TODO".to_string()))
}

fn parse_array_type(input : &mut Input) -> Result<Type, ParseError> {
    parse_junk(input)?;
    punct(input, "[")?;
    let t = Box::new(fatal(parse_type(input), "array type must have type")?);
    fatal(punct(input, "]"), "array type must have closing ']'")?;
    Ok(Type::Array(t))
}

pub fn parse_type(input : &mut Input) -> Result<Type, ParseError> {

    let ps = [ parse_fun_type // fun type probably needs to be before generic type parse
             , parse_generic_type
             , parse_concrete_type 
             , parse_array_type
             ];

    let mut t = None;
    
    for p in ps {
        match p(input) {
            Ok(e) => { t = Some(e); break; },
            e @ Err(ParseError::Fatal(_)) => return e,
            _ => { },
        }
    }

    match t {
        Some(t) => Ok(t), 
        None => Err(ParseError::Error),
    }
    // TODO need to do the after check for indexed types

    /* TODO :
            fun (T, T, T) -> T
            T<T>

    */
}

#[cfg(test)]
mod test {
    use super::*;

}