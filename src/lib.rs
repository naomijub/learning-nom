use nom::{
    character::is_alphanumeric,
    combinator::{map_res, recognize},
    error::{ErrorKind, ParseError, VerboseError},
    AsChar, IResult, InputTakeAtPosition,
};
use uuid::Uuid;

pub mod model;
pub mod parser;

pub fn uuid_parser(s: &str) -> IResult<&str, Uuid, VerboseError<&str>> {
    map_res(recognize(alphanumerichyphen), Uuid::parse_str)(s)
}
pub fn alphanumerichyphen1(s: &str) -> IResult<&str, &str, VerboseError<&str>> {
    alphanumerichyphen(s)
}

fn alphanumerichyphen<T, E: ParseError<T>>(s: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    s.split_at_position1_complete(
        |item| {
            let ch = item.as_char();
            !(ch == '-' || is_alphanumeric(ch as u8))
        },
        ErrorKind::AlphaNumeric,
    )
}

#[test]
fn test() {
    println!(
        "{:?}",
        alphanumerichyphen1("c15a23cd-22d8-4351-b738-396b274599f8 WTF")
    );
}
