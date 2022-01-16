
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
                , FunParam
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

    match maybe(parse_params(parse_expr, input))? {
        Some(params) => Ok(Expr::Cons { name, params }),
        None => Ok(Expr::Cons {name, params: vec![]}),
    }
}

fn parse_lambda(input : &mut Input) -> Result<Expr, ParseError> {
    fn params(input : &mut Input) -> Result<Vec<FunParam>, ParseError> {
        fn parse_lambda_param(input : &mut Input) -> Result<FunParam, ParseError> {
            fn parse_colon_type(input : &mut Input) -> Result<Type, ParseError> {
                punct(input, ":")?;
                fatal(parse_type(input), "lambda parameter must have type after ':'")
            }
            let name = parse_symbol(input)?;
            let t = maybe(parse_colon_type(input))?;
            Ok(FunParam { name, t })
        }

        parse_series( parse_lambda_param, "|", "|", input) 
    }

    fn return_type(input : &mut Input) -> Result<Type, ParseError> {
        punct(input, "->")?;
        fatal(parse_type(input), "return_type must have a type after '->'")
    }

    let params = params(input)?;
    let return_type = maybe(return_type(input))?;
    let expr = Box::new(fatal(parse_expr(input), "lambda must have expr")?);
    
    Ok(Expr::Lambda { params, return_type, expr })
}

pub fn parse_expr(input : &mut Input) -> Result<Expr, ParseError> {

    fn parse_array_expr(input : &mut Input) -> Result<Expr, ParseError> {
        let es = parse_array(parse_expr, input)?;
        Ok(Expr::Array(es))
    }

    let ps = [ parse_bool_expr
             , parse_number_expr
             , parse_let
             , parse_constructor_expr
             , parse_lambda
             , parse_array_expr

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
            [|p, p, p|]
            {|p, p, p|}
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

    #[test]
    fn lambda_should_parse_no_param_lambda() -> Result<(), ParseError> {
        let mut input = Input::new("|| 5");
        let result = parse_expr(&mut input)?;
        assert!( matches!( result, Expr::Lambda { .. } ) );
        // TODO add more details 
        Ok(())
    }

    #[test]
    fn lambda_should_parse_no_type_param_lambda() -> Result<(), ParseError> {
        let mut input = Input::new("|x| x");
        let result = parse_expr(&mut input)?;
        assert!( matches!( result, Expr::Lambda { .. } ) );
        // TODO add more details 
        Ok(())
    }

    #[test]
    fn lambda_should_parse_no_type_params_lambda() -> Result<(), ParseError> {
        let mut input = Input::new("|x, y, z| Cons(x, y, z)");
        let result = parse_expr(&mut input)?;
        assert!( matches!( result, Expr::Lambda { .. } ) );
        // TODO add more details 
        Ok(())
    }

    #[test]
    fn lambda_should_parse_type_param_lambda() -> Result<(), ParseError> {
        let mut input = Input::new("|x : Type| x");
        let result = parse_expr(&mut input)?;
        assert!( matches!( result, Expr::Lambda { .. } ) );
        // TODO add more details 
        Ok(())
    }
    
    #[test]
    fn lambda_should_parse_type_params_lambda() -> Result<(), ParseError> {
        let mut input = Input::new("|x : Type, y : Type, z : Number| x");
        let result = parse_expr(&mut input)?;
        assert!( matches!( result, Expr::Lambda { .. } ) );
        // TODO add more details 
        Ok(())
    }

    #[test]
    fn lambda_should_parse_type_params_lambda_with_return_type() -> Result<(), ParseError> {
        let mut input = Input::new("|x : Type, y : Type, z : Number| -> Type x");
        let result = parse_expr(&mut input)?;
        assert!( matches!( result, Expr::Lambda { .. } ) );
        // TODO add more details 
        Ok(())
    }

    #[test]
    fn lambda_should_parse_lambda_with_return_type() -> Result<(), ParseError> {
        let mut input = Input::new("|| -> Number 5");
        let result = parse_expr(&mut input)?;
        assert!( matches!( result, Expr::Lambda { .. } ) );
        // TODO add more details 
        Ok(())
    }

    #[test]
    fn lambda_should_parse_lambda_with_fun_return_type() -> Result<(), ParseError> {
        let mut input = Input::new("|| -> fun(Number) -> Number |x| x");
        let result = parse_expr(&mut input)?;
        assert!( matches!( result, Expr::Lambda { .. } ) );
        // TODO add more details 
        Ok(())
    }

    #[test]
    fn array_should_parse_empty_array() -> Result<(), ParseError> {
        let mut input = Input::new("[]");
        let result = parse_expr(&mut input)?;
        assert!( matches!( result, Expr::Array(_) ) );
        // TODO add more details 
        Ok(())
    }

    #[test]
    fn array_should_parse_array_with_one_item() -> Result<(), ParseError> {
        let mut input = Input::new("[4]");
        let result = parse_expr(&mut input)?;
        assert!( matches!( result, Expr::Array(_) ) );
        // TODO add more details 
        Ok(())
    }

    #[test]
    fn array_should_parse_array() -> Result<(), ParseError> {
        let mut input = Input::new("[4, 6, 7]");
        let result = parse_expr(&mut input)?;
        assert!( matches!( result, Expr::Array(_) ) );
        // TODO add more details 
        Ok(())
    }
}