use crate::{ Header, Request, Response, Method, Version };

#[derive(Debug)]
pub struct HeaderParseError;
#[derive(Debug)]
pub struct RequestParseError;

pub fn parse_request<'r, 'h>(input: &'r str, header_buf: &'h mut [Header<'r>]) -> Result<Request<'r, 'h, &'r str>, RequestParseError> {
    let (header, body) = input.split_once("\r\n\r\n").unwrap_or(("", ""));
    let mut header_lines = header.split("\r\n");
    let mut status_line = header_lines.next().unwrap_or("").split_whitespace();
    let method: Method = status_line.next().unwrap().into();
    let url = status_line.next().unwrap();
    let version: Version = status_line.next().unwrap().into();

    for (header_idx, header) in header_lines.enumerate() {
        if header_idx >= header_buf.len() { return Err(RequestParseError); }
        header_buf[header_idx] = parse_header(header).unwrap();
    }

    let mut request = Request::new(body, header_buf);
    request.method(method);
    request.url_target(url);
    request.version(version);
    Ok(request)
}
// pub fn parse_response(input: &str) -> Request {}

fn parse_header(input: &str) -> Result<Header, HeaderParseError> {
    match input.split_once(": ") {
        Some((name, value)) => Ok(Header::new(name.into(), value)),
        None => Err(HeaderParseError)
    }
}

// fn parse_headers(input: &str) -> []
