
use super::input::{Input, ParseError};

pub fn into<T, A>(input : &mut Input, p : fn(&mut Input) -> Result<T, ParseError>, map : fn(T) -> A) -> Result<A, ParseError> {
    match p(input) {
        Ok(v) => Ok(map(v)),
        Err(x) => Err(x),
    }
}

pub fn parse_junk(input : &mut Input) -> Result<(), ParseError> {

    let mut comment = false;

    loop {

        match (comment, input.peek()) {
            (true, Ok('\n')) => { comment = false; input.next().unwrap(); }
            (true, Ok('\r')) => { comment = false; input.next().unwrap(); }
            (true, Ok(_)) => { input.next().unwrap(); }
            (false, Ok(c)) if c.is_whitespace() => { input.next().unwrap(); },
            (false, Ok('#')) => { comment = true; input.next().unwrap(); },
            (false, Ok(_)) => return Ok(()), 
            (_, Err(ParseError::Error)) => return Ok(()),
            (_, Err(e @ ParseError::Fatal(_))) => return Err(e),
        }
    }
}

pub fn parse_symbol(input : &mut Input) -> Result<String, ParseError> {
    parse_junk(input)?;

    let mut cs = vec![];

    match input.peek() {
        Ok(c) if c.is_alphabetic() || c == '_' => { cs.push(c); input.next().unwrap(); },
        Err(e @ ParseError::Fatal(_)) => return Err(e),
        _ => return Err(ParseError::Error),
    }

    loop {
        match input.peek() {
            Ok(c) if c.is_alphanumeric() || c == '_' => { cs.push(c); input.next().unwrap(); },
            Err(e @ ParseError::Fatal(_)) => return Err(e),
            _ => return Ok(cs.into_iter().collect::<String>()),
        }
    }
}

pub fn parse_number(input : &mut Input) -> Result<i64, ParseError> {
    parse_junk(input)?;

    let mut cs = vec![];
    let mut negative = false;

    match input.peek() {
        Ok(c) if c.is_ascii_digit() => { cs.push(c); input.next().unwrap(); },
        Ok(c) if c == '-' => { negative = true; input.next().unwrap(); },
        Err(e @ ParseError::Fatal(_)) => return Err(e),
        _ => return Err(ParseError::Error),
    }

    loop {
        match input.peek() {
            Ok(c) if c.is_ascii_digit() => { cs.push(c); input.next().unwrap(); },
            Err(e @ ParseError::Fatal(_)) => return Err(e),
            _ if cs.len() < 1 => return fail("encountered single '-'"),
            _ if negative => return Ok(cs.into_iter().collect::<String>().parse::<i64>().expect("Internal Rust Parse Error") * -1),
            _ => return Ok(cs.into_iter().collect::<String>().parse::<i64>().expect("Internal Rust Parse Error")),
        }
    }
}

pub fn parse_bool(input : &mut Input) -> Result<bool, ParseError> {
    parse_junk(input)?;

    let rp = input.clone();

    match parse_symbol(input) {
        Ok(sym) if sym == "true" => Ok(true),
        Ok(sym) if sym == "false" => Ok(false),
        Err(e @ ParseError::Fatal(_)) => Err(e),
        _ => { input.restore(rp); Err(ParseError::Error)},
    }
}

pub fn keyword(input : &mut Input, value : &str) -> Result<(), ParseError> {
    parse_junk(input)?;

    let rp = input.clone();

    for c in value.chars() {
        match input.next() {
            Ok(v) if c == v => { },
            Ok(_) => { input.restore(rp); return Err(ParseError::Error); },
            Err(ParseError::Error) => { input.restore(rp); return Err(ParseError::Error); }, 
            Err(e @ ParseError::Fatal(_)) => return Err(e),
        }
    }

    match input.peek() {
        Ok(v) if !(v.is_alphanumeric() || v == '_') => Ok(()),
        Ok(_) => { input.restore(rp); Err(ParseError::Error) },
        Err(ParseError::Error) => Ok(()),
        Err(e @ ParseError::Fatal(_)) => Err(e),
    }
}

pub fn punct(input : &mut Input, value : &str) -> Result<(), ParseError> {
    parse_junk(input)?;

    let rp = input.clone();

    for c in value.chars() {
        match input.next() {
            Ok(v) if c == v => { },
            Ok(_) => { input.restore(rp); return Err(ParseError::Error); },
            Err(ParseError::Error) => { input.restore(rp); return Err(ParseError::Error); }, 
            Err(e @ ParseError::Fatal(_)) => return Err(e),
        }
    }

    Ok(())
}

pub fn maybe<T>( x : Result<T, ParseError> ) -> Result<Option<T>, ParseError> {
    match x { 
        Ok(v) => Ok(Some(v)),
        Err(ParseError::Error) => Ok(None),
        Err(e @ ParseError::Fatal(_)) => Err(e),
    }
}

pub fn fatal<T>(x : Result<T, ParseError>, message : &str) -> Result<T, ParseError> {
    match x {
        o @ Ok(_) => o,
        Err(ParseError::Error) => Err(ParseError::Fatal(vec![message.to_string()])), 
        Err(ParseError::Fatal(mut fs)) => {
            fs.push(message.to_string());
            Err(ParseError::Fatal(fs))
        },
    }
}

pub fn fail<T>(message : &str) -> Result<T, ParseError> {
    Err(ParseError::Fatal(vec![message.to_string()]))
}

pub fn parse_params<T>(p : fn(&mut Input) -> Result<T, ParseError>, input : &mut Input) -> Result<Vec<T>, ParseError> {
    punct(input, "(")?;
    match punct(input, ")") {
        Ok(_) => return Ok(vec![]),
        Err(ParseError::Error) => { },
        Err(e @ ParseError::Fatal(_)) => return Err(e),
    }
    let mut ps = vec![];
    loop {
        ps.push(p(input)?);
        match punct(input, ",") {
            Ok(_) => continue,
            Err(ParseError::Error) => { },
            Err(e @ ParseError::Fatal(_)) => return Err(e),
        }
        match punct(input, ")") {
            Ok(_) => break,
            Err(ParseError::Error) => return fail("list parameters must have ending ')'"),
            Err(e @ ParseError::Fatal(_)) => return Err(e),
        }
    }

    Ok(ps)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_single_underscore_symbol() -> Result<(), ParseError> {
        let mut input = Input::new("_");
        let result = parse_symbol(&mut input)?;

        assert_eq!( result, "_" );

        Ok(())
    }

    #[test]
    fn should_parse_single_character_symbol() -> Result<(), ParseError> {
        let mut input = Input::new("a");
        let result = parse_symbol(&mut input)?;

        assert_eq!( result, "a" );

        Ok(())
    }

    #[test]
    fn should_parse_symbol() -> Result<(), ParseError> {
        let mut input = Input::new("blah_1234");
        let result = parse_symbol(&mut input)?;

        assert_eq!( result, "blah_1234" );

        Ok(())
    }

    #[test] 
    fn should_parse_positive_int() -> Result<(), ParseError> {
        let mut input = Input::new("1234");
        let result = parse_number(&mut input)?;

        assert_eq!( result, 1234 );

        Ok(())
    }

    #[test] 
    fn should_parse_negative_int() -> Result<(), ParseError> {
        let mut input = Input::new("-1234");
        let result = parse_number(&mut input)?;

        assert_eq!( result, -1234 );

        Ok(())
    }

    #[test]
    fn should_parse_true() -> Result<(), ParseError> {
        let mut input = Input::new("true");
        let result = parse_bool(&mut input)?;

        assert_eq!( result, true );

        Ok(())
    }

    #[test]
    fn should_parse_false() -> Result<(), ParseError> {
        let mut input = Input::new("false");
        let result = parse_bool(&mut input)?;

        assert_eq!( result, false );

        Ok(())
    }

    #[test]
    fn should_not_consume_non_bool() -> Result<(), ParseError> {
        let mut input = Input::new("false_");
        let result = parse_bool(&mut input);

        assert!( matches!( result, Err(_) ) );

        let result = parse_symbol(&mut input)?;

        assert_eq!( result, "false_" );

        Ok(())
    }

    #[test]
    fn keyword_should_parse_with_whitespace() -> Result<(), ParseError> {
        let mut input = Input::new("input extra");
        keyword(&mut input, "input")?;
        Ok(())
    }

    #[test]
    fn keyword_should_parse_with_non_symbol() -> Result<(), ParseError> {
        let mut input = Input::new("input(");
        keyword(&mut input, "input")?;
        punct(&mut input, "(")?;
        Ok(())
    }

    #[test]
    fn keyword_should_parse_with_end_of_file() -> Result<(), ParseError> {
        let mut input = Input::new("input");
        keyword(&mut input, "input")?;
        Ok(())
    }

    #[test]
    fn keyword_should_leave_input_alone_on_failure() -> Result<(), ParseError> {
        let mut input = Input::new("inputx");
        let result = keyword(&mut input, "input");
        
        assert!( matches!( result, Err(_) ) );

        let result = parse_symbol(&mut input)?;

        assert_eq!( result, "inputx" );

        Ok(())
    }

}