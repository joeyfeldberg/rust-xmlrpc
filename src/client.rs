// Copyright 2014-2015 Galen Clark Haynes
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Rust XML-RPC library

use hyper;
use std::string;
use std::io::Read;

pub struct Client {
    url: string::String,
}

impl Client {
    pub fn new(s: &str) -> Client {
        Client { url: s.to_string() }
    }

    pub fn remote_call(&self, request: &super::Request) -> Option<super::Response> {
        let mut http_client = hyper::Client::new();
        let result = http_client.post(self.url.as_str())
            .body(request.body.as_str()) // FIXME: use to_xml() somehow?
            .send();
        let mut body = String::new();    
        result.ok().unwrap().read_to_string(&mut body).ok().expect("could not read response");
        //println!("{}", response.unwrap());
        Some(super::Response::new(body.as_str())) // FIXME: change to a Result<> type
    }
}
