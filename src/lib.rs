#![no_std]

use squid::{ Url, parse_url };
use hashbrown::HashMap;

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

pub enum Version {
    V10,
    V11,
    V2,
}

pub enum StatusCode { // TODO: add Into + From &str with actual codes (200, 404, etc)
    Ok,
    NotFound,
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
            method: Method::Get,
            target: parse_url("/").unwrap(),
            version: Version::V11,
            headers: HashMap::new(),
            body,
        }
    }
}

impl<B> Response<'_, B> {
    pub fn new(body: B) -> Self {
        Self {
            version: Version::V11,
            status_code: StatusCode::Ok,
            headers: HashMap::new(),
            body
        }
    }
}
