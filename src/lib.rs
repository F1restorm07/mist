#![no_std]

use core::mem::MaybeUninit;

pub use squid::{ Url, parse_url };
mod status_code;
mod header;
 mod parsers;
pub use status_code::StatusCode;
pub use header::Header;
pub use parsers::*;

#[derive(Debug)]
pub enum Method {
    /// request a representaion of the sepecified resource
    Get,
    /// identical to `Get`, but does not receive a response body
    Head,
    /// submit an entity to a specified resource
    Post,
    /// apply partial modifications to a resource
    Patch,
    /// replace all current representations of the target resource with the request payload
    Put,
    /// delete the specified resource
    Delete,
    /// perform a message loopback test along the path to the target resource
    Trace,
    /// establish a tunnel to the server identified by the target resource
    Connect,
    /// describe the communication options for the target resource
    Options
}

impl Default for Method {
    fn default() -> Self { Self::Get }
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Self::Get,
            "HEAD" => Self::Head,
            "POST" => Self::Post,
            "PATCH" => Self::Patch,
            "PUT" => Self::Put,
            "DELETE" => Self::Delete,
            "TRACE" => Self::Trace,
            "CONNECT" => Self::Connect,
            "OPTIONS" => Self::Options,
            _ => Self::Get,
        }
    }
}

#[derive(Debug)]
pub enum Version {
    /// HTTP/1.0
    V10,
    /// HTTP/1.1
    V11,
    /// HTTP/2.0
    V20,
}

impl Default for Version {
    fn default() -> Self { Self::V11 }
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.0" => Self::V10,
            "HTTP/1.1" => Self::V11,
            "HTTP/2.0" => Self::V20,
            _ => Self::V11,
        }
    }
}

pub struct Request<'r, 'h, B/* , const N: usize */> {
    method: Method,
    target: Url<'r>,
    version: Version,
    headers: &'h mut [Header<'r>],
    body: B,
}

impl<B: core::fmt::Debug> core::fmt::Debug for Request<'_, '_, B> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Request")
            .field("method", &self.method)
            .field("url_target", &self.target)
            .field("version", &self.version)
            .field("headers", &self.headers)
            .field("body", &self.body)
            .finish()
    }
}

pub struct Response<'r, B, const N: usize> {
    version: Version,
    status_code: StatusCode,
    headers: [MaybeUninit<Header<'r>>; N],
    __header_init_count: usize,
    body: B
}

impl<'r, 'h, B> Request<'r, 'h, B> {
    pub fn new(body: B, headers: &'h mut [Header<'r>]) -> Self {
        Self {
            method: Method::default(),
            target: parse_url("/").unwrap(),
            version: Version::default(),
            headers,
            body,
        }
    }

    pub fn method(&mut self, method: Method) { self.method = method; }
    pub fn url_target(&mut self, target: &'r str) { self.target = parse_url(target).unwrap(); }
    pub fn version(&mut self, version: Version) { self.version = version; }
}

impl<'r, B, const N: usize> Response<'r, B, N> {
    pub fn new(body: B) -> Self {
        const UNINIT: MaybeUninit<Header<'_>> = MaybeUninit::uninit();
        Self {
            version: Version::default(),
            status_code: StatusCode::default(),
            headers: [UNINIT; N],
            __header_init_count: 0,
            body
        }
    }
    pub fn new_with_headers(body: B, headers: [MaybeUninit<Header<'r>>; N]) -> Self {
        Self {
            version: Version::default(),
            status_code: StatusCode::default(),
            __header_init_count: headers.len(),
            headers,
            body,
        }
    }

    pub fn version(&mut self, version: Version) { self.version = version; }
    pub fn status_code(&mut self, code: StatusCode) { self.status_code = code; }

    pub fn add_header(&mut self, header: Header<'r>) {
        assert!(self.__header_init_count < N);
        self.headers[self.__header_init_count].write(header);
        self.__header_init_count+=1;
    }
    pub fn add_headers(&mut self, headers: impl IntoIterator<Item = Header<'r>>) {
        assert!(self.__header_init_count < N);
        for header in headers {
            self.headers[self.__header_init_count].write(header);
            self.__header_init_count+=1;
        }
    }
}

// #[cfg(test)]
// mod tests {
//     extern crate std;
//     use super::*;
//
//     #[test]
//     fn parse_full_request() {
//         let request =
//         "POST / HTTP/1.1\r\n\
//         Host: localhost:8080\r\n\
//         User-Agent: Mozilla/5.0 (Macintosh;...) Firefox/51.0\r\n\
//         Accept: text/html,application/xhtml+xml,...,*/*;q=0.8\r\n\
//         Accept-Language: en-US,en;q=0.5\r\n\
//         Accept-Encoding: gzip,deflate\r\n\
//         Connection: keep-alive\r\n\
//         Upgrade-Insecure-Requests: 1\r\n\
//         Content-Type: multipart/form-data; boundary=-12656974\r\n\
//         Content-Length: 345\r\n\
//         \r\n\
//         -1265974\r\n\
//         ";
//         let mut header_buf = [header::EMPTY_HEADER; 9];
//         let parsed_request = parse_request(request, &mut header_buf).unwrap();
//         std::println!("{parsed_request:#?}");
//         panic!();
//     }
// }
