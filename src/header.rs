pub const EMPTY_HEADER: Header<'_> = Header { name: HeaderName::Custom(""), value: "" };

#[derive(Debug)]
pub struct Header<'h> {
    name: HeaderName<'h>,
    value: &'h str
}

impl Default for Header<'_> {
    fn default() -> Self { EMPTY_HEADER }
}

impl<'h> Header<'h> {
    pub fn new(name: HeaderName<'h>, value: &'h str) -> Self {
        Self { name, value }
    }
}

#[derive(Debug)]
pub enum HeaderName<'h> {
    Standard(StandardHeaderName),
    Custom(&'h str)
}

impl<'h> From<&'h str> for HeaderName<'h> {
    fn from(value: &'h str) -> Self {
        let header: Result<StandardHeaderName, InvalidStandardHeaderName> = value.try_into();
        match header {
            Ok(header) => Self::Standard(header),
            Err(header) => Self::Custom(header.0)
        }
    }
}

#[derive(Debug)]
pub enum StandardHeaderName {
    Accept,
    AcceptCharset,
    AcceptEncoding,
    AcceptLanguage,
    AcceptPatch,
    AcceptPost,
    AcceptRanges,
    AccessControlAllowCredentials,
    AccessControlAllowHeaders,
    AccessControlAllowMethods,
    AccessControlAllowOrigin,
    AccessControlExposeHeaders,
    AccessControlMaxAge,
    AccessControlRequestHeaders,
    AccessControlRequestMethod,
    Age,
    Allow,
    AltSvc,
    AltUsed,
    Authoritzation,
    CacheControl,
    ClearSiteData,
    Connection,
    ContentDisposition,
    ContentEncoding,
    ContentLanguage,
    ContentLength,
    ContentLocation,
    ContentRange,
    ContentSecurityPolicy,
    ContentSecurityPolicyReportOnly,
    ContentType,
    Cookie,
    CrossOriginEmbedderPolicy,
    CrossOriginOpenerPolicy,
    CrossOriginResourcePolicy,
    Date,
    DeviceMemory,
    Etag,
    Expect,
    Expires,
    Forwarded,
    From,
    Host,
    IfMatch,
    IfModifiedSince,
    IfNoneMatch,
    IfRange,
    IfUnmodifiedSince,
    KeepAlive,
    LastModified,
    Link,
    Location,
    MaxForwards,
    Origin,
    PermissionsPolicy,
    ProxyAuthenticate,
    ProxyAuthorization,
    Range,
    Referer,
    RefererPolicy,
    ReportingEndpoints,
    RetryAfter,
    SecFetchDest,
    SecFetchMode,
    SecFetchSite,
    SecFetchUser,
    SecPurpose,
    SecWebsocketAccept,
    Server,
    ServerTiming,
    ServiceWorkerNavigationPreload,
    SetCookie,
    SourceMap,
    StrictTransportSecurity,
    TE,
    TimingAllowOrigin,
    Trailer,
    TransferEncoding,
    Upgrade,
    UpgradeInsecureRequests,
    UserAgent,
    Vary,
    Via,
    WWWAuthenticate,
    XContentTypeOptions,
    XFrameOptions,
    XXssProtection,
}

pub struct InvalidStandardHeaderName<'i>(&'i str); // ??: should i include the header name in the error type

impl<'e> TryFrom<&'e str> for StandardHeaderName {
    type Error = InvalidStandardHeaderName<'e>;
    fn try_from(value: &'e str) -> Result<Self, Self::Error> {
        match value {
            "Accept" => Ok(Self::Accept),
            "Accept-Charset" => Ok(Self::AcceptCharset),
            "Accept-Encoding" => Ok(Self::AcceptEncoding),
            "Accept-Language" => Ok(Self::AcceptLanguage),
            "Accept-Patch" => Ok(Self::AcceptPatch),
            "Accept-Post" => Ok(Self::AcceptPost),
            "Accept-Ranges" => Ok(Self::AcceptRanges),
            "Access-Control-Allow-Credentials" => Ok(Self::AccessControlAllowCredentials),
            "Access-Control-Allow-Headers" => Ok(Self::AccessControlAllowHeaders),
            "Access-Control-Allow-Methods" => Ok(Self::AccessControlAllowMethods),
            "Access-Control-Allow-Origin" => Ok(Self::AccessControlAllowOrigin),
            "Access-Control-Max-Age" => Ok(Self::AccessControlMaxAge),
            "Access-Control-Request-Headers" => Ok(Self::AccessControlRequestHeaders),
            "Access-Control-Request-Method" => Ok(Self::AccessControlRequestMethod),
            "Age" => Ok(Self::Age),
            "Allow" => Ok(Self::Allow),
            "Alt-Svc" => Ok(Self::AltSvc),
            "Alt-Used" => Ok(Self::AltUsed),
            "Authorization" => Ok(Self::Authoritzation),
            "Cache-Control" => Ok(Self::CacheControl),
            "Clear-Site-Data" => Ok(Self::ClearSiteData),
            "Connection" => Ok(Self::Connection),
            "Content-Disposition" => Ok(Self::ContentDisposition),
            "Content-Encoding" => Ok(Self::ContentEncoding),
            "Content-Language" => Ok(Self::ContentLanguage),
            "Content-Length" => Ok(Self::ContentLength),
            "Content-Location" => Ok(Self::ContentLocation),
            "Content-Range" => Ok(Self::ContentRange),
            "Content-Security-Policy" => Ok(Self::ContentSecurityPolicy),
            "Content-Security-Policy-Report-Only" => Ok(Self::ContentSecurityPolicyReportOnly),
            "Content-Type" => Ok(Self::ContentType),
            "Cookie" => Ok(Self::Cookie),
            "Cross-Origin-Embedder-Policy" => Ok(Self::CrossOriginEmbedderPolicy),
            "Cross-Origin-Opener-Policy" => Ok(Self::CrossOriginOpenerPolicy),
            "Cross-Origin-Resource-Policy" => Ok(Self::CrossOriginResourcePolicy),
            "Date" => Ok(Self::Date),
            "Device-Memory" => Ok(Self::DeviceMemory),
            "Etag" => Ok(Self::Etag),
            "Expect" => Ok(Self::Expect),
            "Expires" => Ok(Self::Expires),
            "Forwarded" => Ok(Self::Forwarded),
            "From" => Ok(Self::From),
            "Host" => Ok(Self::Host),
            "If-Match" => Ok(Self::IfMatch),
            "If-Modified-Since" => Ok(Self::IfModifiedSince),
            "If-None-Match" => Ok(Self::IfNoneMatch),
            "If-Range" => Ok(Self::IfRange),
            "If-Unmodified-Since" => Ok(Self::IfUnmodifiedSince),
            "Keep-Alive" => Ok(Self::KeepAlive),
            "Last-Modified" => Ok(Self::LastModified),
            "Link" => Ok(Self::Link),
            "Location" => Ok(Self::Location),
            "Max-Forwards" => Ok(Self::MaxForwards),
            "Origin" => Ok(Self::Origin),
            "Permissions-Policy" => Ok(Self::PermissionsPolicy),
            "Proxy-Authenticate" => Ok(Self::ProxyAuthenticate),
            "Proxy-Authorization" => Ok(Self::ProxyAuthorization),
            "Range" => Ok(Self::Range),
            "Referer" => Ok(Self::Referer),
            "Referer-Policy" => Ok(Self::RefererPolicy),
            "Reporting-Endpoints" => Ok(Self::ReportingEndpoints),
            "Retry-After" => Ok(Self::RetryAfter),
            "Sec-Fetch-Dest" => Ok(Self::SecFetchDest),
            "Sec-Fetch-Mode" => Ok(Self::SecFetchMode),
            "Sec-Fetch-Site" => Ok(Self::SecFetchSite),
            "Sec-Fetch-User" => Ok(Self::SecFetchUser),
            "Sec-Purpose" => Ok(Self::SecPurpose),
            "Sec-Websocket-Accept" => Ok(Self::SecWebsocketAccept),
            "Server" => Ok(Self::Server),
            "Server-Timing" => Ok(Self::ServerTiming),
            "Service-Worker-Navigation-Preload" => Ok(StandardHeaderName::ServiceWorkerNavigationPreload),
            "Set-Cookie" => Ok(Self::SetCookie),
            "SourceMap" => Ok(Self::SourceMap),
            "Strict-Transport-Security" => Ok(StandardHeaderName::StrictTransportSecurity),
            "TE" => Ok(Self::TE),
            "Timing-Allow-Origin" => Ok(Self::TimingAllowOrigin),
            "Trailer" => Ok(Self::Trailer),
            "Transfer-Encoding" => Ok(Self::TransferEncoding),
            "Upgrade" => Ok(Self::Upgrade),
            "Upgrade-Insecure-Requests" => Ok(Self::UpgradeInsecureRequests),
            "User-Agent" => Ok(Self::UserAgent),
            "Vary" => Ok(Self::Vary),
            "Via" => Ok(Self::Via),
            "WWW-Authenticate" => Ok(Self::WWWAuthenticate),
            "X-Content-Type-Options" => Ok(Self::XContentTypeOptions),
            "X-Frame-Options" => Ok(Self::XFrameOptions),
            "X-XSS-Protection" => Ok(Self::XXssProtection),
            h => Err(InvalidStandardHeaderName(h)),
        }
    }
}
