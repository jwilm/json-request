json-request
============

Rust library for talking to JSON HTTP servers

[Documentation][]

## Usage

One method, `request`, is exposed which makes HTTP requests and provides
automatic serialization/deserialization of Rust types to JSON.

```rust
extern crate rustc_serialize;
extern crate json_request;

use json_request::{request, Method};

#[derive(Debug, RustcEncodable)]
struct RequestData {
    ping: bool
}

#[derive(Debug, RustcDecodable)]
struct ResponseData {
    pong: bool
}

// `data` is the object to be serialized and sent to the HTTP server
let data = RequestData { ping: true };

// Actually build the request
let res = request(Method::Post, "http://example.com/", Some(data));

// Request returns a Result<Option<D>>; hence, two unwrap calls. The wrapped
// value has been deserialized from a JSON response.
let pong: ResponseData = res.unwrap().unwrap();
```

## Install

Add the following to your Cargo.toml

```toml
[dependencies.json-request]
git = "https://github.com/jwilm/json-request"
```

## Notes

- *TODO*: The `data` parameter should be url encoded and appended to the URL for
  GET requests.
- *TODO*: Would be nice to have a DSL macro that's a little more user friendly

[Documentation]: http://jwilm.github.io/json-request/json_request
