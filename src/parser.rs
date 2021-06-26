use nom::{branch::alt, bytes::streaming::{tag, tag_no_case}, character::streaming::alphanumeric1, combinator::opt, error::{ErrorKind, context}, sequence::{separated_pair, terminated}};

use crate::model::{Authority, Scheme};


pub fn scheme(input: &str) -> Result<(&str, Scheme), nom::Err<(&str, ErrorKind)>> {
    context(
        "scheme",
        alt((tag_no_case("HTTP://"), tag_no_case("HTTPS://"))),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

pub fn authority(input: &str) -> Result<(&str, Authority), nom::Err<(&str, ErrorKind)>> {
    context(
        "authority",
        terminated(
            separated_pair(alphanumeric1, opt(tag(":")), opt(alphanumeric1)),
            tag("@"),
        ),
    )(input)
}


#[cfg(test)]
mod tests {
    use super::*;
    use nom::{
        error::{ErrorKind,},
        Err as NomErr,
    };

    #[test]
    fn test_scheme() {
        assert_eq!(scheme("https://yay"), Ok(("yay", Scheme::HTTPS)));
        assert_eq!(scheme("http://yay"), Ok(("yay", Scheme::HTTP)));
        assert_eq!(scheme("bla://yay"), Err(NomErr::Error(("bla://yay", ErrorKind::Tag))))
    }

    #[test]
    fn test_authority() {
        assert_eq!(
            authority("username:password@zupzup.org"),
            Ok(("zupzup.org", ("username", Some("password"))))
        );
        assert_eq!(
            authority("username@zupzup.org"),
            Ok(("zupzup.org", ("username", None)))
        );
        assert_eq!(
            authority("zupzup.org"),
            Err(NomErr::Error((".org", ErrorKind::Tag)))
        );
    //     assert_eq!(
    //         authority(":zupzup.org"),
    //         Err(NomErr::Error(VerboseError {
    //             errors: vec![
    //                 (
    //                     ":zupzup.org",
    //                     VerboseErrorKind::Nom(ErrorKind::AlphaNumeric)
    //                 ),
    //                 (":zupzup.org", VerboseErrorKind::Context("authority")),
    //             ]
    //         }))
    //     );
    //     assert_eq!(
    //         authority("username:passwordzupzup.org"),
    //         Err(NomErr::Error(VerboseError {
    //             errors: vec![
    //                 (".org", VerboseErrorKind::Nom(ErrorKind::Tag)),
    //                 (
    //                     "username:passwordzupzup.org",
    //                     VerboseErrorKind::Context("authority")
    //                 ),
    //             ]
    //         }))
    //     );
    //     assert_eq!(
    //         authority("@zupzup.org"),
    //         Err(NomErr::Error(VerboseError {
    //             errors: vec![
    //                 (
    //                     "@zupzup.org",
    //                     VerboseErrorKind::Nom(ErrorKind::AlphaNumeric)
    //                 ),
    //                 ("@zupzup.org", VerboseErrorKind::Context("authority")),
    //             ]
    //         }))
    }
}