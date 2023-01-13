use super::method::Method;
use std::convert::TryFrom;

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = String;
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}
