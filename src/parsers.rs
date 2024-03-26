use crate::{ Header, Request, Response, Method, Version, StatusCode };

#[derive(Debug)]
pub enum ParseError {
    TooManyHeaders,
    InvalidRequest,
    InvalidResponse,
    InvalidHeader,
}

pub fn parse_request<'r, 'h>(input: &'r str, header_buf: &'h mut [Header<'r>]) -> Result<Request<'r, 'h, &'r str>, ParseError> {
    let (header, body) = input.split_once("\r\n\r\n").unwrap_or(("", ""));
    let mut header_lines = header.split("\r\n");
    let mut status_line = header_lines.next().unwrap_or("").split_whitespace();

    let method: Method = status_line.next().unwrap().into();
    let url = status_line.next().unwrap();
    let version: Version = status_line.next().unwrap().into();

    parse_headers(header_lines, header_buf).unwrap();

    let mut request = Request::new(body, header_buf);
    request.method(method);
    request.url_target(url);
    request.version(version);
    Ok(request)
}
pub fn parse_response<'r, 'h>(input: &'r str, header_buf: &'h mut [Header<'r>]) -> Result<Response<'r, 'h, &'r str>, ParseError> {
    let (header, body) = input.split_once("\r\n\r\n").unwrap_or(("", ""));
    let mut header_lines = header.split("\r\n");
    let mut status_line = header_lines.next().unwrap_or("").split_once(" ").unwrap_or(("", ""));

    let version: Version = status_line.0.into();
    let status_code: StatusCode = status_line.1.into();

    parse_headers(header_lines, header_buf).unwrap();

    let mut response = Response::new(body, header_buf);
    response.version(version);
    response.status_code(status_code);
    Ok(response)
}

fn parse_header(input: &str) -> Result<Header, ParseError> {
    match input.split_once(": ") {
        Some((name, value)) => Ok(Header::new(name.into(), value)),
        None => Err(ParseError::InvalidHeader)
    }
}

fn parse_headers<'r>(header_lines: core::str::Split<'r, &str> , header_buf: &'_ mut [Header<'r>]) -> Result<(), ParseError> {
    for (header_idx, header) in header_lines.enumerate() {
        if header_idx >= header_buf.len() { return Err(ParseError::TooManyHeaders); }
        header_buf[header_idx] = parse_header(header).unwrap();
    }
    Ok(())
}
