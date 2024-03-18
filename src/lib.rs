#![no_std]

use squid::{ Url, parse_url };
use hashbrown::HashMap;

mod status_code;
pub use status_code::StatusCode;
mod header;
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
    V10,
    V11,
    V2,
}

impl Default for Version {
    fn default() -> Self { Self::V11 }
}

pub struct Request<'r, B> {
    method: Method,
    target: Url, // TODO: add path and query type
    version: Version,
    headers: HashMap<Header<'r>, &'r [u8]>,
    body: B
}

pub struct Response<'r, B> {
    version: Version,
    status_code: StatusCode,
    headers: HashMap<Header<'r>, &'r [u8]>,
    body: B
}

impl<'r, B> Request<'r, B> {
    pub fn new(body: B) -> Self {
        Self {
            method: Method::default(),
            target: parse_url("/").unwrap(),
            version: Version::default(),
            headers: HashMap::new(),
            body,
        }
    }
    pub fn method(&mut self, method: Method) { self.method = method; }
    pub fn url_target(&mut self, target: &str) { self.target = parse_url(target).unwrap(); }
    pub fn version(&mut self, version: Version) { self.version = version; }

    pub fn add_header<I>(&mut self, header: Header<'r>, value: I)
        where
            I: Into<&'r [u8]>
        { self.headers.insert(header, value.into()); }
    pub fn add_headers<I>(&mut self, headers: impl IntoIterator<Item = (Header<'r>, I)>)
        where
            I: Into<&'r [u8]>
        { self.headers.extend(headers.into_iter().map(|(h, v)| (h, v.into()) )); }
}

impl<'r, B> Response<'r, B> {
    pub fn new(body: B) -> Self {
        Self {
            version: Version::default(),
            status_code: StatusCode::default(),
            headers: HashMap::new(),
            body
        }
    }
    pub fn version(&mut self, version: Version) { self.version = version; }
    pub fn status_code(&mut self, code: StatusCode) { self.status_code = code; }

    pub fn add_header<I>(&mut self, header: Header<'r>, value: I)
        where
            I: Into<&'r [u8]>
        { self.headers.insert(header, value.into()); }
    pub fn add_headers<I>(&mut self, headers: impl IntoIterator<Item = (Header<'r>, I)>)
        where
            I: Into<&'r [u8]>
        { self.headers.extend(headers.into_iter().map(|(h, v)| (h, v.into()) )); }
}
