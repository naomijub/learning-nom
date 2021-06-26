#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub struct URI<'a> {
    pub(crate) scheme: Scheme,
    pub(crate) authority: Option<Authority<'a>>,
    pub(crate) host: Host,
    pub(crate) port: Option<u16>,
    pub(crate) path: Option<Vec<&'a str>>,
    pub(crate) query: Option<QueryParams<'a>>,
    pub(crate) fragment: Option<&'a str>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub enum Scheme {
    #[allow(non_camel_case_types)]
    HTTP,
    #[allow(non_camel_case_types)]
    HTTPS,
}

pub type Authority<'a> = (&'a str, Option<&'a str>);

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub enum Host {
    #[allow(non_camel_case_types)]
    HOST(String),
    #[allow(non_camel_case_types)]
    IP([u8; 4]),
}

pub type QueryParam<'a> = (&'a str, &'a str);
pub type QueryParams<'a> = Vec<QueryParam<'a>>;

impl From<&str> for Scheme {
    fn from(i: &str) -> Self {
        match i.to_lowercase().as_str() {
            "http://" => Scheme::HTTP,
            "https://" => Scheme::HTTPS,
            _ => unimplemented!("no other schemes supported"),
        }
    }
}
