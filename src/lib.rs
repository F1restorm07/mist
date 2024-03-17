#![no_std]

use squid::{ Url, parse_url };
use hashbrown::HashMap;

mod status_code;
pub use status_code::StatusCode;

// TODO: add defaults for Url + others in Request and Response

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

pub enum Header<'h> {
    Standard(StandardHeader),
    Custom(&'h [u8])
}

pub enum StandardHeader {
    Accept,
    AcceptEncoding,
    AcceptLanguage,
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
    status_code: StatusCode, // ??: matching (or bundling) the status text to the status code
    // status_text: &'r str,
    headers: HashMap<Header<'r>, &'r [u8]>,
    body: B
}

impl<B> Request<'_, B> {
    pub fn new(body: B) -> Self {
        Self {
            method: Method::default(),
            target: parse_url("/").unwrap(),
            version: Version::default(),
            headers: HashMap::new(),
            body,
        }
    }
}

impl<B> Response<'_, B> {
    pub fn new(body: B) -> Self {
        Self {
            version: Version::default(),
            status_code: StatusCode::default(),
            headers: HashMap::new(),
            body
        }
    }
}
