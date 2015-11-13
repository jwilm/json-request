extern crate json_request;

extern crate iron;
#[macro_use]
extern crate router;
extern crate bodyparser;

extern crate hyper;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use iron::Protocol;

use json_request::{request, request_str, Method};

struct PingServer;

impl PingServer {
    pub fn build() -> Iron<Chain> {
        Iron::new(Chain::new(router!(
            post "/ping" => PingServer::post
        )))
    }

    fn post(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "{\"pong\": true}")))
    }
}

#[derive(Debug, RustcEncodable)]
struct RequestData {
    ping: bool
}

#[derive(Debug, RustcDecodable)]
struct ResponseData {
    pong: bool
}

struct StackListener {
    server: ::hyper::server::Listening,
    host: String,
}

impl StackListener {
    pub fn new() -> StackListener {
        let server = PingServer::build().listen_with("0.0.0.0:0", 1, Protocol::Http).unwrap();
        let host = format!("{}", server.socket);

        StackListener {
            server: server,
            host: host
        }
    }

    pub fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.host, path)
    }
}

impl Drop for StackListener {
    fn drop(&mut self) {
        self.server.close().unwrap();
    }
}

#[test]
#[allow(unused_variables)]
fn with_data() {
    let server = StackListener::new();

    let req = RequestData { ping: true };

    // When this fails, the error I get it "called Option::unwrap() on a None value" which is not
    // helpful for resolving what the problem is.
    let url = server.url("/ping");
    let res: ResponseData = request(Method::Post, &url[..], Some(req)).unwrap().unwrap();
}

#[test]
#[allow(unused_variables)]
fn none_data() {
    let server = StackListener::new();

    let url = server.url("/ping");
    let res: ResponseData = request(Method::Post, &url[..], None::<u8>).unwrap().unwrap();
}

#[test]
fn str_data() {
    let server = StackListener::new();
    let url = server.url("/ping");
    let res = request_str(Method::Post, &url[..], Some("arst")).unwrap().unwrap();
    assert_eq!(&res[..], "{\"pong\": true}");
}
