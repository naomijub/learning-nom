use model::URI;
use nom::{
    character::is_alphanumeric,
    combinator::{map_res, opt, recognize},
    error::{context, ErrorKind, ParseError, VerboseError},
    sequence::tuple,
    AsChar, IResult, InputTakeAtPosition,
};
use uuid::Uuid;

pub mod model;
pub mod parser;

use parser::*;

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

pub fn uri(input: &str) -> IResult<&str, URI, VerboseError<&str>> {
    context(
        "uri",
        tuple((
            scheme,
            opt(authority),
            ip_or_host,
            opt(port),
            opt(path),
            opt(query_params),
            opt(fragment),
        )),
    )(input)
    .map(|(next_input, res)| {
        let (scheme, authority, host, port, path, query, fragment) = res;
        (
            next_input,
            URI {
                scheme,
                authority,
                host,
                port,
                path,
                query,
                fragment,
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use crate::model::*;

    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            alphanumerichyphen1("c15a23cd-22d8-4351-b738-396b274599f8 WTF"),
            Ok((" WTF", "c15a23cd-22d8-4351-b738-396b274599f8"))
        );
    }

    #[test]
    fn test_uri() {
        assert_eq!(
            uri("https://www.zupzup.org/about/"),
            Ok((
                "",
                URI {
                    scheme: Scheme::HTTPS,
                    authority: None,
                    host: Host::HOST("www.zupzup.org".to_string()),
                    port: None,
                    path: Some(vec!["about"]),
                    query: None,
                    fragment: None
                }
            ))
        );

        assert_eq!(
            uri("http://localhost"),
            Ok((
                "",
                URI {
                    scheme: Scheme::HTTP,
                    authority: None,
                    host: Host::HOST("localhost".to_string()),
                    port: None,
                    path: None,
                    query: None,
                    fragment: None
                }
            ))
        );

        assert_eq!(
            uri("https://www.zupzup.org:443/about/?someVal=5#anchor"),
            Ok((
                "",
                URI {
                    scheme: Scheme::HTTPS,
                    authority: None,
                    host: Host::HOST("www.zupzup.org".to_string()),
                    port: Some(443),
                    path: Some(vec!["about"]),
                    query: Some(vec![("someVal", "5")]),
                    fragment: Some("anchor")
                }
            ))
        );

        assert_eq!(
            uri("http://user:pw@127.0.0.1:8080"),
            Ok((
                "",
                URI {
                    scheme: Scheme::HTTP,
                    authority: Some(("user", Some("pw"))),
                    host: Host::IP([127, 0, 0, 1]),
                    port: Some(8080),
                    path: None,
                    query: None,
                    fragment: None
                }
            ))
        );
    }
}
