//! json_request exports a single function, [`request`](fn.request.html) for making JSON calls to an HTTP server.
//!
//! This crate relies on [hyper][] for proving an HTTP client, and [rustc_serialize][] for automatic
//! encoding/decoding of JSON requests/responses.
//!
//! [hyper]: http://hyper.rs/hyper/hyper/index.html
//! [rustc_serialize]: https://doc.rust-lang.org/rustc-serialize/rustc_serialize/index.html
extern crate hyper;
extern crate rustc_serialize;

use rustc_serialize::{json, Encodable, Decodable};

pub use hyper::method::Method;
use hyper::Client;
use hyper::header;
use hyper::status::StatusClass;

use std::io::{Read, self};
use std::error::Error as StdError;

// ---------------------------------- ERROR HANDLING STUFF -----------------------------------------
/// Error wrapper
#[derive(Debug)]
pub enum Error {
    /// Error in HTTP library
    HttpClient(hyper::Error),
    /// Error encoding JSON object for request
    JsonEncoder(json::EncoderError),
    /// Error decoding JSON object from response
    JsonDecoder(json::DecoderError),
    /// Error reading body from hyper response
    IoError(io::Error),
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::HttpClient(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<json::EncoderError> for Error {
    fn from(err: json::EncoderError) -> Error {
        Error::JsonEncoder(err)
    }
}

impl From<json::DecoderError> for Error {
    fn from(err: json::DecoderError) -> Error {
        Error::JsonDecoder(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl ::std::error::Error for Error {
    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::HttpClient(ref err) => Some(err),
            Error::JsonDecoder(ref err) => Some(err),
            Error::JsonEncoder(ref err) => Some(err),
            Error::IoError(ref err) => Some(err),
        }
    }

    fn description(&self) -> &str {
        match *self {
            Error::HttpClient(ref err) => err.description(),
            Error::JsonDecoder(ref err) => err.description(),
            Error::JsonEncoder(ref err) => err.description(),
            Error::IoError(ref err) => err.description(),
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str(self.description())
    }
}
// -------------------------------- END ERROR HANDLING STUFF ---------------------------------------

/// Make an HTTP request
///
/// The third parameter of request, data, will be automatically serialized as JSON and set as the
/// request body before making the request. The wrapped result will be decoded from any 2xx JSON
/// response. For the automatic encoding to work, the `data` type must implement
/// `rustc_serialize::Encodable`, and the result type must implement `rustc_serialize::Decodable`.
/// This can usually be achieved with `#[derive(RustcEncodable)]` and `#[derive(RustcDecodable)]`.
///
/// # Example
///
/// ```no_run
/// extern crate rustc_serialize;
/// extern crate json_request;
///
/// use json_request::{request, Method};
///
/// #[derive(Debug, RustcEncodable)]
/// struct RequestData {
///     ping: bool
/// }
///
/// #[derive(Debug, RustcDecodable)]
/// struct ResponseData {
///     pong: bool
/// }
///
/// # fn main() {
/// let data = RequestData { ping: true };
/// let res = request(Method::Post, "http://example.com/ping", Some(data));
/// // Request returns a Result<Option<D>>; hence, two unwrap calls
/// let pong: ResponseData = res.unwrap().unwrap();
/// # }
/// ```
///
/// Alternatively, if you don't want to specify the binding type, pass the type parameter to
/// `request`.
///
/// ```no_run
/// # extern crate rustc_serialize;
/// # extern crate json_request;
/// # use json_request::{request, Method};
/// # #[derive(Debug, RustcEncodable)]
/// # struct RequestData {
/// #     ping: bool
/// # }
/// # #[derive(Debug, RustcDecodable)]
/// # struct ResponseData {
/// #     pong: bool
/// # }
/// # fn main() {
/// # let data = RequestData { ping: true };
/// let res = request::<_, ResponseData>(Method::Post, "http://example.com/ping", Some(data));
/// let pong = res.unwrap().unwrap();
/// # }
/// ```
pub fn request<S, D>(method: Method, url: &str, data: Option<S>) -> Result<Option<D>>
where S: Encodable, D: Decodable {
    let mut body = String::new();

    let client = Client::new();
    println!("url: {}", url);

    let mut res = match data {
        Some(inner) => {
            let payload = try!(json::encode(&inner));
            let builder = client.request(method, url)
                                .header(header::Connection::close())
                                .body(&payload[..]);

            try!(builder.send())
        },
        None => {
            let builder = client.request(method, url)
                                .header(header::Connection::close());

            try!(builder.send())
        }
    };

    Ok(match res.status.class() {
        StatusClass::Success => {
            try!(res.read_to_string(&mut body));
            Some(try!(json::decode::<D>(&body[..])))
        },
        _ => None
    })
}
