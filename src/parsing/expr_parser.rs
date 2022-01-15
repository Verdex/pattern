
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
                 , fail
                 };
use super::type_parser::parse_type;
use crate::ast::{ Expr
                , Type
                };

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

pub fn parse_expr(input : &mut Input) -> Result<Expr, ParseError> {


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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn let_should_parse_let() -> Result<(), ParseError> {
        let mut input = Input::new("let x = 5 in x");
        let result = parse_let(&mut input)?;
        assert!( matches!( result, Expr::Let { .. } ) );
        // TODO this test can be more comprehensive 
        Ok(())
    }

}