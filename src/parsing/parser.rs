
use super::input::{Input, ParseError};
use super::util::{ into
                 , parse_junk
                 , parse_symbol
                 , parse_number
                 , parse_bool
                 , keyword
                 , punct
                 , maybe
                 , fatal
                 };
use crate::ast::{ StandardPattern
                , ArrayPattern
                , PathPattern
                , Expr
                , Type
                , Ast
                };

pub fn parse(input : &str) -> Result<Ast, ParseError> {
    let mut input = Input::new(input);

    // TODO get a list of top levels?
    let _output = parse_top_level(&mut input);


    Err(ParseError::Fatal("Problem".to_string()))
}

fn parse_let(input : &mut Input) -> Result<Expr, ParseError> {
    fn colon_and_type(input : &mut Input) -> Result<Type, ParseError> {
        parse_junk(input)?;
        punct(input, ":")?;
        parse_junk(input)?;
        parse_type(input)
    }

    parse_junk(input)?;

    keyword(input, "let")?;
    
    let name = fatal(parse_symbol(input), "let must have name")?;

    let t = maybe(colon_and_type(input))?;

    fatal(punct(input, "="), "let must have '='")?;

    let value = Box::new(fatal(parse_expr(input), "let must have value")?);

    fatal(keyword(input, "in"), "let must have 'in'")?;

    let expr = Box::new(fatal(parse_expr(input), "let must have expr")?);

    Ok(Expr::Let{name, t, value, expr})
}

fn parse_standard_pattern(_input : &mut Input) -> Result<StandardPattern, ParseError> {
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

fn parse_array_pattern(_input : &mut Input) -> Result<ArrayPattern, ParseError> {
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

fn parse_path_pattern(_input : &mut Input) -> Result<PathPattern, ParseError> {
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
             , parse_let

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

fn parse_type(_input : &mut Input) -> Result<Type, ParseError> {
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

fn parse_fun_def(input : &mut Input) -> Result<Ast, ParseError> {
    parse_junk(input)?;

    keyword(input, "fun")?;

    let _name = fatal(parse_symbol(input), "fun must have a name")?;

    fatal(punct(input, "("), "fun must have a beginning '('")?;

    // TODO ...

    let _expr = fatal(parse_expr(input), "fun must have an expr")?;

    fatal(punct(input, ";"), "fun must have an ending ';'")?;

    Err(ParseError::Fatal("TODO".to_string()))
}

fn parse_top_level(input : &mut Input) -> Result<Ast, ParseError> {

    // TODO this needs to get all top level items and not just the first one
    // maybe make that happen in the parse function?

    let ps = [ parse_fun_def 
             ];

    let mut tl = None;
    
    for p in ps {
        match p(input) {
            Ok(e) => { tl = Some(e); break; },
            e @ Err(ParseError::Fatal(_)) => return e,
            _ => { },
        }
    }

    match tl {
        Some(tl) => Ok(tl), 
        None => Err(ParseError::Error),
    }

    /* TODO :
             data X = A | B(C, D) ;
             data X<A, B, C> = A | B| C ;
             fun x(a : T, b : T, c : T) -> T = e ;
    */
}

#[cfg(test)]
mod test {
    use super::*;

}