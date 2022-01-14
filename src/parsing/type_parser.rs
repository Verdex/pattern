
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
    fn param_list(input : &mut Input) -> Result<Vec<Type>, ParseError> {
        fatal(punct(input, "("), "fun type must have opening '('")?;
        match punct(input, ")") {
            Ok(_) => return Ok(vec![]),
            Err(ParseError::Error) => { },
            Err(e @ ParseError::Fatal(_)) => return Err(e),
        }
        let mut ts = vec![];
        loop {
            ts.push(parse_type(input)?);
            match punct(input, ",") {
                Ok(_) => continue,
                Err(ParseError::Error) => { },
                Err(e @ ParseError::Fatal(_)) => return Err(e),
            }
            match punct(input, ")") {
                Ok(_) => break,
                Err(ParseError::Error) => return Err(ParseError::Fatal("fun type parameters must have ending ')'".to_string())),
                Err(e @ ParseError::Fatal(_)) => return Err(e),
            }
        }

        Ok(ts)
    }

    parse_junk(input)?;

    keyword(input, "fun")?;

    let i = fatal(param_list(input), "fun type must have param list")?;

    fatal(punct(input, "->"), "fun type must have '->'")?;

    let output = Box::new(fatal(parse_type(input), "fun type must have output type")?);

    Ok(Type::Fun{ input: i, output})
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