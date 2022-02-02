
use crate::ast::{ Ast
                , FunParam
                , ConsDef
                };
use super::input::{Input, ParseError};
use super::util::{ parse_symbol
                 , parse_params
                 , keyword
                 , maybe
                 , punct
                 , fatal
                 , fail
                 };
use super::type_parser::parse_type;
use super::expr_parser::parse_expr;

pub fn parse(input : &str) -> Result<Vec<Ast>, ParseError> {
    let mut input = Input::new(input);

    let mut tls = vec![];
    loop {
        match parse_top_level(&mut input) {
            Ok(tl) => tls.push(tl),
            Err(ParseError::Error) => return Ok(tls),
            Err(e @ ParseError::Fatal(_)) => return Err(e),
        }
    }
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
    fn parse_cons_def(input : &mut Input) -> Result<ConsDef, ParseError> {
        let name = parse_type_name(input)?;
        match maybe(parse_params(parse_type, input))? {
            Some(params) => Ok(ConsDef { name, params }),
            None => Ok(ConsDef { name, params: vec![] }),
        }
    }
    
    fn parse_cons_defs(input : &mut Input) -> Result<Vec<ConsDef>, ParseError> {
        match punct(input, ";") {
            Ok(_) => return Ok(vec![]),
            Err(ParseError::Error) => { },
            Err(e @ ParseError::Fatal(_)) => return Err(e),
        }
        let mut ps = vec![];
        loop {
            ps.push(fatal(parse_cons_def(input), "data must have valid constructor definitions")?);
            match punct(input, "|") {
                Ok(_) => continue,
                Err(ParseError::Error) => { },
                Err(e @ ParseError::Fatal(_)) => return Err(e),
            }
            match punct(input, ";") {
                Ok(_) => break,
                Err(ParseError::Error) => return fail("data definition must end with ;"),
                Err(e @ ParseError::Fatal(_)) => return Err(e),
            }
        }

        Ok(ps)
    }

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

    let cons_defs = fatal(parse_cons_defs(input), "data definition must have data defs")?;

    Ok(Ast::DataDef{ name, cons_defs })
}

fn parse_top_level(input : &mut Input) -> Result<Ast, ParseError> {

    let ps = [ parse_fun_def 
             , parse_data_def
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_function() -> Result<(), ParseError> {
        let result = parse("
            fun name ( a : T, b : T ) -> Number = 8;
        ")?;

        assert_eq!( result.len(), 1 );
        assert!( matches!( result[0], Ast::FunDef { .. } ) );
        Ok(())
    }

    #[test]
    fn should_parse_data() -> Result<(), ParseError> {
        let result = parse("
            data X = H(X, Number) | Nil;
        ")?;

        assert_eq!( result.len(), 1 );
        assert!( matches!( result[0], Ast::DataDef { .. } ) );
        Ok(())
    }

}