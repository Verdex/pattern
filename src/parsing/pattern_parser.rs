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


// TODO:  NOTE:  parse_series( ..., [, | ) // tada

pub fn parse_path_pattern(_input : &mut Input) -> Result<PathPattern, ParseError> {
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

pub fn parse_standard_pattern(_input : &mut Input) -> Result<StandardPattern, ParseError> {
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

pub fn parse_array_pattern(_input : &mut Input) -> Result<ArrayPattern, ParseError> { // TODO maybe pass in parse_expr ?
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

}