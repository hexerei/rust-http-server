#[derive(Debug)]
pub enum Method {
    GET(String),
    DELETE(u64),
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTONS,
    TRACE,
    PATCH
}        
