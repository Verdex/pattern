
use super::input::{Input, ParseError};
use super::util::{ into
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
                 };
use super::type_parser::parse_type;
use super::pattern_parser::{parse_path_pattern, parse_array_pattern, parse_standard_pattern};
use crate::ast::{ Expr
                , Case
                , Type
                , FunParam
                };

fn parse_let(input : &mut Input) -> Result<Expr, ParseError> {
    fn colon_and_type(input : &mut Input) -> Result<Type, ParseError> {
        punct(input, ":")?;
        parse_type(input)
    }

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

fn parse_match(input : &mut Input) -> Result<Expr, ParseError> {
    fn parse_case(input : &mut Input) -> Result<Case, ParseError> {
        let pattern = parse_standard_pattern(parse_expr, input)?;
        fatal(punct(input, "=>"), "pattern case must have an => after a pattern")?;
        let expr = fatal(parse_expr(input), "pattern case must have an expr")?;
        Ok(Case { pattern, expr })
    }

    keyword(input, "match")?;
    let expr = Box::new(fatal(parse_expr(input), "match statements must have an expression")?);
    let cases = fatal(parse_series(parse_case, "{", "}", input), "match statements must have case body")?;
    Ok(Expr::Match{ expr, cases })
}

pub fn parse_expr(input : &mut Input) -> Result<Expr, ParseError> {

    fn parse_array_expr(input : &mut Input) -> Result<Expr, ParseError> {
        into(input, |i| parse_array(parse_expr, i), |es| Expr::Array(es))
    }

    fn parse_path_pattern_expr(input : &mut Input) -> Result<Expr, ParseError> {
        into( input
            , |j| parse_series( |i| parse_path_pattern(parse_expr, i)
                              , "{|"
                              , "|}"
                              , j
                              )
            , |patterns| Expr::PathPattern(patterns))
    }

    fn parse_array_pattern_expr(input : &mut Input) -> Result<Expr, ParseError> {
        into( input
            , |j| parse_series( |i| parse_array_pattern(parse_expr, i)
                              , "[|"
                              , "|]"
                              , j
                              )
            , |patterns| Expr::ArrayPattern(patterns))
    }

    let ps = [ parse_bool_expr
             , parse_number_expr
             , parse_let
             , parse_constructor_expr
             , parse_lambda
             , parse_array_expr
             , parse_path_pattern_expr 
             , parse_array_pattern_expr
             , parse_match

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

    let mut ret = match expr {
        Some(expr) => expr, 
        None => return Err(ParseError::Error),
    };

    loop {
        match parse_params(parse_expr, input) {
            Ok(params) => 
            { 
                let temp = Expr::FunCall { fun_expr : Box::new(ret), params };
                ret = temp;
                continue;
            },
            Err(ParseError::Error) => { },
            Err(e @ ParseError::Fatal(_)) => return Err(e),
        }

        match punct(input, ".") {
            Ok(_) => {
                let name = Box::new(Expr::Variable(fatal(parse_symbol(input), "there must exist a symbol after .")?));

                let mut params = fatal(parse_params(parse_expr, input), "dot function must have parameter list")?;

                params.insert(0, ret);

                let temp = Expr::FunCall { fun_expr : name, params };
                ret = temp;
            },
            Err(ParseError::Error) => return Ok(ret),
            Err(e @ ParseError::Fatal(_)) => return Err(e),
        }
    }
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

    #[test]
    fn should_parse_match() -> Result<(), ParseError> {
        let mut input = Input::new("match 7 { x => x }");
        let result = parse_expr(&mut input)?;
        assert!( matches!( result, Expr::Match{ .. } ) );
        // TODO add more details 
        Ok(())
    }

    #[test]
    fn should_parse_match_with_multiple_cases() -> Result<(), ParseError> {
        let mut input = Input::new("match 7 { 
            x => x,
            _ => 0
        }");
        let result = parse_expr(&mut input)?;
        assert!( matches!( result, Expr::Match{ .. } ) );
        // TODO add more details 
        Ok(())
    }
}