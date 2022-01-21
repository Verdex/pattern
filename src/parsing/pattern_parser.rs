use super::input::{Input, ParseError};
use super::util::{ into
                 , parse_junk
                 , parse_series
                 , parse_array
                 , parse_params
                 , parse_symbol
                 , parse_number
                 , parse_bool
                 , keyword
                 , punct
                 , maybe
                 , fatal
                 , fail
                 };
use super::type_parser::parse_type;
use crate::ast::{ Expr
                , Type
                , StandardPattern
                , PathPattern
                , ArrayPattern
                };

fn parse_variable(input : &mut Input) -> Result<String, ParseError> {
    let rp = input.clone();

    let sym = parse_symbol(input)?;

    let first = sym.chars().nth(0)
        .expect("pattern parse_symbol somehow returned zero length string");

    if first.is_lowercase() {
        Ok(sym)
    }
    else {
        input.restore(rp);
        Err(ParseError::Error)
    }
}

fn parse_constructor<T, F : Fn(&mut Input) -> Result<T, ParseError>>(p : F, input : &mut Input) -> Result<(String, Vec<T>), ParseError> {
    fn parse_name(input : &mut Input) -> Result<String, ParseError> {
        let rp = input.clone();

        let sym = parse_symbol(input)?;

        let first = sym.chars().nth(0)
            .expect("pattern parse_symbol somehow returned zero length string");

        if first.is_uppercase() {
            Ok(sym)
        }
        else {
            input.restore(rp);
            Err(ParseError::Error)
        }
    }

    let name = parse_name(input)?;

    match maybe(parse_params(p, input))? {
        Some(params) => Ok( (name, params) ),
        None => Ok( (name, vec![]) ),
    }
}

fn parse_at<T, F : Fn(&mut Input) -> Result<T, ParseError>>(p : F, input : &mut Input) -> Result<(String, Box<T>), ParseError> {
    let rp = input.clone();
    let name = parse_symbol(input)?;
    match punct(input, "@") {
        Ok(_) => { },
        Err(ParseError::Error) => { input.restore(rp); return Err(ParseError::Error); },
        Err(e @ ParseError::Fatal(_)) => return Err(e),
    }
    let pattern = Box::new(fatal(p(input), "@ pattern is missing a target pattern")?);
    Ok((name, pattern))
}

// TODO:  NOTE:  parse_series( ..., [, | ) // tada

pub fn parse_path_pattern(parse_expr : fn(&mut Input) -> Result<Expr, ParseError>, input : &mut Input) -> Result<PathPattern, ParseError> {
    fn parse_number_pattern(_ : fn(&mut Input) -> Result<Expr, ParseError>, input : &mut Input) -> Result<PathPattern, ParseError> {
        into(input, parse_number, |n| PathPattern::Number(n))
    }
    
    fn parse_bool_pattern(_ : fn(&mut Input) -> Result<Expr, ParseError>, input : &mut Input) -> Result<PathPattern, ParseError> {
        into(input, parse_bool, |b| PathPattern::Bool(b))
    }

    fn parse_var_pattern(_ : fn(&mut Input) -> Result<Expr, ParseError>, input : &mut Input) -> Result<PathPattern, ParseError> {
        into(input, parse_variable, |v| PathPattern::Variable(v))
    }

    fn parse_cons_pattern(parse_expr : fn(&mut Input) -> Result<Expr, ParseError>, input : &mut Input) -> Result<PathPattern, ParseError> {
        into(input, |i| parse_constructor(|x| parse_path_pattern(parse_expr, x), i), |(name, params)| PathPattern::Cons{name, params})
    }

    fn parse_at_pattern(parse_expr : fn(&mut Input) -> Result<Expr, ParseError>, input : &mut Input) -> Result<PathPattern, ParseError> {
        into(input, |i| parse_at(|x| parse_path_pattern(parse_expr, x), i), |(name, pattern)| PathPattern::At{name, pattern})
    }

    fn parse_wildcard_pattern(_ : fn(&mut Input) -> Result<Expr, ParseError>, input : &mut Input) -> Result<PathPattern, ParseError> {
        punct(input, "_")?;
        Ok(PathPattern::Wildcard)
    }

    fn parse_next_pattern(_ : fn(&mut Input) -> Result<Expr, ParseError>, input : &mut Input) -> Result<PathPattern, ParseError> {
        punct(input, "!")?;
        into(input, |i| maybe(parse_number(i)), |number| PathPattern::Next(number))
    }

    fn parse_and_pattern(_ : fn(&mut Input) -> Result<Expr, ParseError>, input : &mut Input) -> Result<PathPattern, ParseError> {
        punct(input, "&")?;
        let name = fatal(parse_symbol(input), "& pattern must have a path name")?;
        fatal(punct(input, ":"), "& pattern must have a ':'")?;
        let output = fatal(parse_symbol(input), "& pattern must have an output")?;
        Ok(PathPattern::And{ name, output })
    }

    fn parse_next_and_pattern(_ : fn(&mut Input) -> Result<Expr, ParseError>, input : &mut Input) -> Result<PathPattern, ParseError> {
        let rp = input.clone();
        punct(input, "!")?;
        let order = maybe(parse_number(input))?;
        match punct(input, "&") {
            Ok(_) => { }, 
            Err(ParseError::Error) => { input.restore(rp); return Err(ParseError::Error); },
            Err(e @ ParseError::Fatal(_)) => return Err(e),
        }
        let name = fatal(parse_symbol(input), "& pattern must have a path name")?;
        fatal(punct(input, ":"), "& pattern must have a ':'")?;
        let output = fatal(parse_symbol(input), "& pattern must have an output")?;
        Ok(PathPattern::NextAnd{ order, name, output })
    }

    let ps = [ parse_number_pattern
             , parse_bool_pattern
             , parse_cons_pattern
             , parse_at_pattern
             , parse_wildcard_pattern
             , parse_and_pattern        // Need to do in this order parse_and_pattern, parse_next_and_pattern, parse_next_pattern
             , parse_next_and_pattern
             , parse_next_pattern

             , parse_var_pattern// This should probably be last to avoid eating up keywords, etc
             ];

    let mut pattern = None;
    
    for p in ps {
        match p(parse_expr, input) {
            Ok(e) => { pattern = Some(e); break; },
            e @ Err(ParseError::Fatal(_)) => return e,
            _ => { },
        }
    }

    let pattern = match pattern {
        Some(pattern) => pattern, 
        None => return Err(ParseError::Error),
    };

    match keyword(input, "if") {
        Ok(_) => { },
        Err(ParseError::Error) => return Ok(pattern),
        Err(e @ ParseError::Fatal(_)) => return Err(e),
    }

    let predicate = fatal(parse_expr(input), "pattern must have expression after if")?;

    Ok(PathPattern::If { pattern: Box::new(pattern), predicate })
    /* TODO: 
           []
           [p, p, p]
           [p | p] (tail)
    */
}

pub fn parse_standard_pattern(parse_expr : fn(&mut Input) -> Result<Expr, ParseError>, _input : &mut Input) -> Result<StandardPattern, ParseError> {
    /* TODO: 
           number
           bool
           variable
           Cons(p*)
           x @ p
           p if bool-expr
           _
           []
           [p, p, p]
           [p | p] (tail)
    */
    fail("TODO")
}

pub fn parse_array_pattern(parse_expr : fn(&mut Input) -> Result<Expr, ParseError>, _input : &mut Input) -> Result<ArrayPattern, ParseError> { // TODO maybe pass in parse_expr ?
    /* TODO: 
           number
           bool
           variable
           Cons(p*)
           x @ p
           p if bool-expr
           _{number-expr}
           _* 
           _
           []
           [p, p, p]
           [p | p] (tail)
           p; p; p
    */
    fail("TODO")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn path_pattern_var_should_parse() -> Result<(), ParseError> {
        let mut input = Input::new("a");
        let result = parse_path_pattern(|i| Err(ParseError::Error), &mut input)?;
        assert!( matches!( result, PathPattern::Variable(_) ) );
        // TODO add more details 
        Ok(())
    }
    
    #[test]
    fn path_pattern_number_should_parse() -> Result<(), ParseError> {
        let mut input = Input::new("100");
        let result = parse_path_pattern(|i| Err(ParseError::Error), &mut input)?;
        assert!( matches!( result, PathPattern::Number(_) ) );
        // TODO add more details 
        Ok(())
    }

    #[test]
    fn path_pattern_bool_should_parse() -> Result<(), ParseError> {
        let mut input = Input::new("true");
        let result = parse_path_pattern(|i| Err(ParseError::Error), &mut input)?;
        assert!( matches!( result, PathPattern::Bool(_) ) );
        // TODO add more details 
        Ok(())
    }

    #[test]
    fn path_pattern_cons_should_parse() -> Result<(), ParseError> {
        let mut input = Input::new("Cons(A, A)");
        let result = parse_path_pattern(|i| Err(ParseError::Error), &mut input)?;
        assert!( matches!( result, PathPattern::Cons { .. } ) );
        // TODO add more details 
        Ok(())
    }

    #[test]
    fn path_pattern_at_should_parse() -> Result<(), ParseError> {
        let mut input = Input::new("x @ Cons(A, A)");
        let result = parse_path_pattern(|i| Err(ParseError::Error), &mut input)?;
        assert!( matches!( result, PathPattern::At { .. } ) );
        // TODO add more details 
        Ok(())
    }

    #[test]
    fn path_pattern_at_should_parse_recursive() -> Result<(), ParseError> {
        let mut input = Input::new("x @ y @ Cons(A, A)");
        let result = parse_path_pattern(|i| Err(ParseError::Error), &mut input)?;
        assert!( matches!( result, PathPattern::At { .. } ) );
        // TODO add more details 
        Ok(())
    }

    #[test]
    fn path_pattern_wildcard_should_parse() -> Result<(), ParseError> {
        let mut input = Input::new("_");
        let result = parse_path_pattern(|i| Err(ParseError::Error), &mut input)?;
        assert!( matches!( result, PathPattern::Wildcard ) );
        // TODO add more details 
        Ok(())
    }

    #[test]
    fn path_pattern_next_should_parse() -> Result<(), ParseError> {
        let mut input = Input::new("!");
        let result = parse_path_pattern(|i| Err(ParseError::Error), &mut input)?;
        assert!( matches!( result, PathPattern::Next(None)) );
        Ok(())
    }

    #[test]
    fn path_pattern_next_with_order_should_parse() -> Result<(), ParseError> {
        let mut input = Input::new("!2");
        let result = parse_path_pattern(|i| Err(ParseError::Error), &mut input)?;
        assert!( matches!( result, PathPattern::Next(Some(2)) ) );
        Ok(())
    }

    #[test]
    fn path_pattern_and_should_parse() -> Result<(), ParseError> {
        let mut input = Input::new("&path:output");
        let result = parse_path_pattern(|i| Err(ParseError::Error), &mut input)?;
        assert!( matches!( result, PathPattern::And { .. } ) );
        // TODO add more details 
        Ok(())
    }

    #[test]
    fn path_pattern_next_and_should_parse() -> Result<(), ParseError> {
        let mut input = Input::new("!&path:output");
        let result = parse_path_pattern(|i| Err(ParseError::Error), &mut input)?;
        assert!( matches!( result, PathPattern::NextAnd {..} ) );
        // TODO add more details 
        Ok(())
    }

    #[test]
    fn path_pattern_next_and_with_order_should_parse() -> Result<(), ParseError> {
        let mut input = Input::new("!2&path:output");
        let result = parse_path_pattern(|i| Err(ParseError::Error), &mut input)?;
        assert!( matches!( result, PathPattern::NextAnd {..} ) );
        // TODO add more details 
        Ok(())
    }

    #[test]
    fn path_pattern_if_should_parse() -> Result<(), ParseError> {
        let mut input = Input::new("x if true");
        let result = parse_path_pattern(|i| Ok(Expr::Bool(true)), &mut input)?;
        assert!( matches!( result, PathPattern::If {..} ) );
        // TODO add more details 
        Ok(())
    }
}