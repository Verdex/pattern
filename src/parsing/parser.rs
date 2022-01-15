
use crate::ast::Ast;
use super::input::{Input, ParseError};
use super::util::{ parse_junk
                 , parse_symbol
                 , keyword
                 , punct
                 , fatal
                 , fail
                 };
use super::expr_parser::parse_expr;

pub fn parse(input : &str) -> Result<Ast, ParseError> {
    let mut input = Input::new(input);

    // TODO get a list of top levels?
    let _output = parse_top_level(&mut input);


    fail("TODO")
}

fn parse_fun_def(input : &mut Input) -> Result<Ast, ParseError> {
    parse_junk(input)?;

    keyword(input, "fun")?;

    let _name = fatal(parse_symbol(input), "fun must have a name")?;

    fatal(punct(input, "("), "fun must have a beginning '('")?;

    // TODO ...

    let _expr = fatal(parse_expr(input), "fun must have an expr")?;

    fatal(punct(input, ";"), "fun must have an ending ';'")?;

    fail("TODO")
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