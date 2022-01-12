
use super::input::{Input, ParseError};

pub fn into<T, A>(input : &mut Input, p : fn(&mut Input) -> Result<T, ParseError>, map : fn(T) -> A) -> Result<A, ParseError> {
    match p(input) {
        Ok(v) => Ok(map(v)),
        Err(x) => Err(x),
    }
}