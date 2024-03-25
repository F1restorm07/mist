#![no_std]

use core::mem::MaybeUninit;

pub use squid::{ Url, parse_url };
mod status_code;
mod header;
pub mod parsers;
pub use status_code::StatusCode;
pub use header::Header;

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

pub enum Version {
    /// HTTP/1.0
    V10,
    /// HTTP/1.1
    V11,
    /// HTTP/2.0
    V2,
}

impl Default for Version {
    fn default() -> Self { Self::V11 }
}

pub struct Request<'r, B, const N: usize> {
    method: Method,
    target: Url<'r>, // TODO: add path and query type
    version: Version,
    headers: [MaybeUninit<Header<'r>>; N],
    __header_init_count: usize,
    body: B,
}

pub struct Response<'r, B, const N: usize> {
    version: Version,
    status_code: StatusCode,
    headers: [MaybeUninit<Header<'r>>; N],
    __header_init_count: usize,
    body: B
}

impl<'r, B, const N: usize> Request<'r, B, N> {
    pub fn new(body: B) -> Self {
        const UNINIT: MaybeUninit<Header<'_>> = MaybeUninit::uninit();
        Self {
            method: Method::default(),
            target: parse_url("/").unwrap(),
            version: Version::default(),
            headers: [UNINIT; N],
            __header_init_count: 0,
            body,
        }
    }
    pub fn new_with_headers(body: B, headers: [MaybeUninit<Header<'r>>; N]) -> Self {
        Self {
            method: Method::default(),
            target: parse_url("/").unwrap(),
            version: Version::default(),
            __header_init_count: headers.len(),
            headers,
            body,
        }
    }
    pub fn method(&mut self, method: Method) { self.method = method; }
    pub fn url_target(&mut self, target: &'r str) { self.target = parse_url(target).unwrap(); }
    pub fn version(&mut self, version: Version) { self.version = version; }

    pub fn add_header(&mut self, header: Header<'r>) {
        assert!(self.__header_init_count < N);
        self.headers[self.__header_init_count].write(header);
        self.__header_init_count+=1;
    }
    pub fn add_headers<I>(&mut self, headers: impl IntoIterator<Item = Header<'r>>) {
        assert!(self.__header_init_count < N);
        for header in headers {
            self.headers[self.__header_init_count].write(header);
            self.__header_init_count+=1;
        }
    }
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
    pub fn add_headers<I>(&mut self, headers: impl IntoIterator<Item = Header<'r>>) {
        assert!(self.__header_init_count < N);
        for header in headers {
            self.headers[self.__header_init_count].write(header);
            self.__header_init_count+=1;
        }
    }
}
