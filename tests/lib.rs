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

use json_request::{request, Method};

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
    pub fn new(port: u16) -> StackListener {
        let host = format!("0.0.0.0:{}", port);
        StackListener {
            server: PingServer::build().listen_with(&host[..], 1, Protocol::Http).unwrap(),
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
    let server = StackListener::new(40918);

    let req = RequestData { ping: true };

    // When this fails, the error I get it "called Option::unwrap() on a None value" which is not
    // helpful for resolving what the problem is.
    let url = server.url("/ping");
    let res: ResponseData = request(Method::Post, &url[..], Some(req)).unwrap().unwrap();
}

#[test]
#[allow(unused_variables)]
fn none_data() {
    let server = StackListener::new(40919);

    let url = server.url("/ping");
    let res: ResponseData = request(Method::Post, &url[..], None::<u8>).unwrap().unwrap();
}
