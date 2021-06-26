use nom::{AsChar, IResult, InputTakeAtPosition, character::is_alphanumeric, combinator::{map_res, recognize}, error::{ErrorKind, ParseError, VerboseError}};
use uuid::Uuid;

pub mod model;
pub mod parser;

pub fn uuid_parser(s: &str) -> IResult<&str, Uuid, VerboseError<&str>> {
    map_res(recognize(uuidchar), Uuid::parse_str)(s)
}

fn uuidchar<T, E: ParseError<T>>(s: T) -> IResult<T, T, E>
where
 T: InputTakeAtPosition,
 <T as InputTakeAtPosition>::Item: AsChar,
{
 s.split_at_position1_complete(|item| {
     let ch = item.as_char();
     !(ch == '-' || is_alphanumeric(ch as u8))
 }, ErrorKind::AlphaNumeric)
}

#[test]
fn test() {
   println!("{:?}", uuid_parser("c15a23cd-22d8-4351-b738-396b274599f8 WTF"));
}