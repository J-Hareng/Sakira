use std::collections::HashMap;

use dotenv::Error;
use thiserror::Error;

//"GET /index.html HTTP/1.1\r\n
// Accept: application/json, text/plain, */*\r\n
// Content-Type: application/json\r\n
// User-Agent: bruno-runtime/4.3.0\r\n
// Content-Length: 19\r\n
// Accept-Encoding: gzip, compress, deflate, br\r\n
// Host: 127.0.0.1:8000\r\n
// Connection: keep-alive\r\n
// \r\n
// {\n  \"Test\":\"test\"\n}"

#[derive(Error, Debug)]
pub enum BuilderError {}

pub struct Request {
    header: RequestHeader,
    body: String,
}

impl Request {
    pub fn new(header: RequestHeader, body: String) -> Self {
        Request { header, body }
    }
    pub fn has_body(&self) -> bool {
        !self.body.is_empty()
    }
}

pub struct RequestHeader {
    method: String,
    path: String,
    entrys: Vec<(String, String)>,
}

impl RequestHeader {
    fn new(method: String, path: String, entrys: Vec<(String, String)>) -> RequestHeader {
        RequestHeader {
            method,
            path,
            entrys,
        }
    }

    pub fn new_from_bytes(instr: &str) -> Result<RequestHeader, BuilderError> {
        let mut sliced_req = instr.split("\r\n").to_owned();
        let first_arr = sliced_req.next().unwrap().split(" ").collect::<Vec<&str>>();

        let method = first_arr[0];
        let path = first_arr.get(1);
        let http_version = first_arr.get(2);

        if method.is_empty() || path.is_none() || http_version.is_none() {
            return Err("Invalid request header format");
        }

        let entrys = sliced_req
            .map(RequestHeader::split_entry)
            .collect::<Vec<(String, String)>>();

        Ok(RequestHeader::new(
            method.to_string(),
            path.unwrap().to_string(),
            entrys,
        ))
    }

    fn split_entry(s: &str) -> (String, String) {
        let parts: Vec<&str> = s.split(":").collect();
        (
            parts[0].to_string(),
            parts.get(1).unwrap_or(&"").to_string(),
        )
    }
}
