
use crate::ast::{ Ast
                , FunParam
                };
use super::input::{Input, ParseError};
use super::util::{ parse_symbol
                 , parse_params
                 , keyword
                 , punct
                 , fatal
                 , fail
                 };
use super::type_parser::parse_type;
use super::expr_parser::parse_expr;

pub fn parse(input : &str) -> Result<Ast, ParseError> {
    let mut input = Input::new(input);

    // TODO get a list of top levels?
    let _output = parse_top_level(&mut input);


    fail("TODO")
}

fn parse_fun_def(input : &mut Input) -> Result<Ast, ParseError> {
    fn parse_fun_name(input : &mut Input) -> Result<String, ParseError> {
        let rp = input.clone();

        let sym = parse_symbol(input)?;

        let first = sym.chars().nth(0)
            .expect("parse_fun_def parse_symbol somehow returned zero length string");

        if first.is_lowercase() {
            Ok(sym)
        }
        else {
            input.restore(rp);
            Err(ParseError::Error)
        }
    }

    fn params(input : &mut Input) -> Result<FunParam, ParseError> {
        let name = parse_symbol(input)?;
        fatal(punct(input, ":"), "fun parameter needs :")?;
        let t = Some(fatal(parse_type(input), "fun parameter needs types")?);
        Ok(FunParam{ name, t })
    }

    keyword(input, "fun")?;

    let name = fatal(parse_fun_name(input), "fun must have a name")?;

    let params = fatal(parse_params(|i| params(i), input), "fun must have parameters")?;

    fatal(punct(input, "->"), "fun must have ->")?;

    let return_type = fatal(parse_type(input), "fun must have type")?;

    fatal(punct(input, "="), "fun must have =")?;

    let expr = fatal(parse_expr(input), "fun must have an expr")?;

    fatal(punct(input, ";"), "fun must have an ending ';'")?;

    Ok(Ast::FunDef { name, params, return_type, expr })
}

fn parse_data_def(input : &mut Input) -> Result<Ast, ParseError> {
    fn parse_type_name(input : &mut Input) -> Result<String, ParseError> {
        let rp = input.clone();

        let sym = parse_symbol(input)?;

        let first = sym.chars().nth(0)
            .expect("parse_type_name parse_symbol somehow returned zero length string");

        if first.is_uppercase() {
            Ok(sym)
        }
        else {
            input.restore(rp);
            Err(ParseError::Error)
        }
    }

    keyword(input, "data")?;

    let name = fatal(parse_type_name(input), "data definition must have a name")?;

    fatal(punct(input, "="), "data definition must have a =")?;

    // TODO:  parse_type_name optional ( type ) | or ; loop

    fail("TODO")
}

fn parse_top_level(input : &mut Input) -> Result<Ast, ParseError> {

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
    */
}

#[cfg(test)]
mod test {
    use super::*;

}