
use crate::input::{Input, ParseError};
use crate::ast::{ StandardPattern
                , ArrayPattern
                , PathPattern
                , Expr
                , Ast
                };

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

fn parse_number(input : &mut Input) -> Result<i64, ParseError> {
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
            _ if negative => return Ok(cs.into_iter().collect::<String>().parse::<i64>().expect("Internal Rust Parse Error") * -1),
            _ => return Ok(cs.into_iter().collect::<String>().parse::<i64>().expect("Internal Rust Parse Error")),
        }
    }
}

fn parse_bool(input : &mut Input) -> Result<bool, ParseError> {
    parse_junk(input)?;

    let rp = input.clone();

    match parse_symbol(input) {
        Ok(sym) if sym == "true" => Ok(true),
        Ok(sym) if sym == "false" => Ok(false),
        Err(e @ ParseError::Fatal(_)) => Err(e),
        _ => { input.restore(rp); Err(ParseError::Error)},
    }
}

fn parse_let(input : &mut Input) -> Result<Expr, ParseError> {
    parse_junk(input)?;

    Err(ParseError::Fatal("TODO".to_string()))
}

fn parse_standard_pattern(input : &mut Input) -> Result<StandardPattern, ParseError> {
    /* TODO: 
           number
           variable
           p | p
           Cons(p*)
           x @ p
           p if bool-expr
           _
           []
           [p, p, p]
           [p | p] (tail)
    */
    Err(ParseError::Fatal("TODO".to_string()))
}

fn parse_array_pattern(input : &mut Input) -> Result<ArrayPattern, ParseError> {
    /* TODO: 
           number
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
    Err(ParseError::Fatal("TODO".to_string()))
}

fn parse_path_pattern(input : &mut Input) -> Result<PathPattern, ParseError> {
    /* TODO: 
           number
           variable
           Cons(p*)
           x @ p
           p if bool-expr
           !
           !N
           &path_pattern_symbol_name:output_symbol
           !&path_pattern_symbol_name:output_symbol
           !N&path_pattern_symbol_name:output_symbol
           []
           [p, p, p]
           [p | p] (tail)
           p; p; p
    */
    Err(ParseError::Fatal("TODO".to_string()))
}

fn parse_expr(input : &mut Input) -> Result<Expr, ParseError> {

    fn parse_bool_expr(input : &mut Input) -> Result<Expr, ParseError> {
        into(input, parse_bool, |b| Expr::Bool(b))
    }

    fn parse_number_expr(input : &mut Input) -> Result<Expr, ParseError> {
        into(input, parse_number, |n| Expr::Number(n))
    }

    fn parse_variable_expr(input : &mut Input) -> Result<Expr, ParseError> {
        let rp = input.clone();

        let sym = parse_symbol(input)?;

        let first = sym.chars().nth(0)
            .expect("parse_expr::parse_variable_expr parse_symbol somehow returned zero length string");

        if first.is_lowercase() {
            Ok(Expr::Variable(sym))
        }
        else {
            input.restore(rp);
            Err(ParseError::Error)
        }
    }

    let ps = [ parse_bool_expr
             , parse_number_expr

             , parse_variable_expr // This should probably be last to avoid eating up keywords, etc
             ];

    let mut expr = None;
    
    for p in ps {
        match p(input) {
            Ok(e) => { expr = Some(e); break; },
            e @ Err(ParseError::Fatal(_)) => return e,
            _ => { },
        }
    }

    match expr {
        Some(expr) => Ok(expr), 
        None => Err(ParseError::Error),
    }

    // TODO:  Will need to figure out how to do after expressions (like . and ())

    /* TODO :
            || e
            |x, y, z| e
            |x : T, y : T, z : T| -> T  e
            let x = y in z
            let x : T = y in z
            [e, e, e]
            {p, p, p}
            <p, p, p>
            match e {
                p => e,
                p => e,
                p => e,
            }

            x(y, z)
            y.x(z)

    */
}

fn parse_type(input : &mut Input) -> Result<(), ParseError> {
    /* TODO :
            fun (T, T, T) -> T
            [T]
            T<T>

            concrete types are upper case
            generic types are lower case
            anon types exist but are not parsable (atm)
            path_pattern<anon>
            array_pattern<anon>
    */

    Err(ParseError::Fatal("TODO".to_string()))
}

fn parse_top_level(input : &mut Input) -> Result<(), ParseError> {
    /* TODO :
             data X = A | B(C, D) ;
             data X<A, B, C> = A | B| C ;
             fun x(a : T, b : T, c : T) -> T = e ;
    */
    Err(ParseError::Fatal("TODO".to_string()))
}

fn into<T, A>(input : &mut Input, p : fn(&mut Input) -> Result<T, ParseError>, map : fn(T) -> A) -> Result<A, ParseError> {
    match p(input) {
        Ok(v) => Ok(map(v)),
        Err(x) => Err(x),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test] 
    fn should_parse_positive_int() -> Result<(), ParseError> {
        let mut input = Input::new("1234");
        let result = parse_number(&mut input)?;

        assert_eq!( result, 1234 );

        Ok(())
    }

    #[test] 
    fn should_parse_negative_int() -> Result<(), ParseError> {
        let mut input = Input::new("-1234");
        let result = parse_number(&mut input)?;

        assert_eq!( result, -1234 );

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

    #[test]
    fn should_parse_true() -> Result<(), ParseError> {
        let mut input = Input::new("true");
        let result = parse_bool(&mut input)?;

        assert_eq!( result, true );

        Ok(())
    }

    #[test]
    fn should_parse_false() -> Result<(), ParseError> {
        let mut input = Input::new("false");
        let result = parse_bool(&mut input)?;

        assert_eq!( result, false );

        Ok(())
    }

    #[test]
    fn should_not_consume_non_bool() -> Result<(), ParseError> {
        let mut input = Input::new("false_");
        let result = parse_bool(&mut input);

        assert!( matches!( result, Err(_) ) );

        let result = parse_symbol(&mut input)?;

        assert_eq!( result, "false_" );

        Ok(())
    }
}