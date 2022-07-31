use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::BufRead;
use std::str;
use std::str::Utf8Error;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;

        match get_next_word(request) {
            Some((method, request)) => {}
            None => return Err(ParseError::Request),
        }

        let (method, request) = get_next_word(request).ok_or(ParseError::Request)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::Request)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::Request)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::Protocol);
        }

        unimplemented!()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i..]));
        }
    }

    None
}

pub enum ParseError {
    Request,
    Encoding,
    Protocol,
    Method,
}

impl ParseError {
    fn message(&self) -> &str {
        match *self {
            Self::Request => "Invalid Request",
            Self::Encoding => "Invalid Encoding",
            Self::Protocol => "Invalid Protocol",
            Self::Method => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::Encoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
