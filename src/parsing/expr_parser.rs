
use super::input::{Input, ParseError};
use super::util::{ into
                 , parse_junk
                 , parse_list
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

fn parse_constructor_expr(input : &mut Input) -> Result<Expr, ParseError> {
    fn parse_name(input : &mut Input) -> Result<String, ParseError> {
        let rp = input.clone();
        let sym = parse_symbol(input)?;
        let first = sym.chars().nth(0)
            .expect("parse_expr::parse_constructor_expr parse_symbol somehow returned zero length string");
        if first.is_uppercase() {
            Ok(sym)
        }
        else {
            input.restore(rp);
            Err(ParseError::Error)
        }
    }

    let name = parse_name(input)?;

    match maybe(parse_list(parse_expr, input))? {
        Some(params) => Ok(Expr::Cons { name, params }),
        None => Ok(Expr::Cons {name, params: vec![]}),
    }
}

fn parse_path_pattern(_input : &mut Input) -> Result<PathPattern, ParseError> {
    /* TODO: 
           number
           bool
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
    fail("TODO")
}

fn parse_standard_pattern(_input : &mut Input) -> Result<StandardPattern, ParseError> {
    /* TODO: 
           number
           bool
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
    fail("TODO")
}

fn parse_array_pattern(_input : &mut Input) -> Result<ArrayPattern, ParseError> { // TODO maybe pass in parse_expr ?
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

pub fn parse_expr(input : &mut Input) -> Result<Expr, ParseError> {


    let ps = [ parse_bool_expr
             , parse_number_expr
             , parse_let
             , parse_constructor_expr

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
            Cons
            Cons(e,e,e)
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
    fn let_should_parse() -> Result<(), ParseError> {
        let mut input = Input::new("let x = 5 in x");
        let result = parse_expr(&mut input)?;
        assert!( matches!( result, Expr::Let { .. } ) );
        // TODO add more details 
        Ok(())
    }

    #[test]
    fn cons_should_parse_non_param_cons() -> Result<(), ParseError> {
        let mut input = Input::new("SomeCons");
        let result = parse_expr(&mut input)?;
        assert!( matches!( result, Expr::Cons { .. } ) );
        // TODO add more details 
        Ok(())
    }

    #[test]
    fn cons_should_parse_params_cons() -> Result<(), ParseError> {
        let mut input = Input::new("SomeCons(1, 2, 3)");
        let result = parse_expr(&mut input)?;
        assert!( matches!( result, Expr::Cons { .. } ) );
        // TODO add more details 
        Ok(())
    }
}