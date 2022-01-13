
use super::input::{Input, ParseError};
use super::util::{ into
                 , parse_junk
                 , parse_symbol
                 , keyword
                 , punct
                 , maybe
                 , fatal
                 };
use crate::ast::Type;

pub fn parse_type(_input : &mut Input) -> Result<Type, ParseError> {
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
