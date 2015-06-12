extern crate hyper;

use std::io::Write;
use std::vec::Vec;
use self::hyper::server::Request;
use self::hyper::server::Response;
use self::hyper::net::Fresh;
use std::io::Read;
use self::hyper::Client;
//use hyper::header::Connection;
use self::hyper::server::Handler;
use self::hyper::uri::RequestUri;

pub struct HTTPHandler;

impl Handler for HTTPHandler {
     fn handle(&self, req: Request, res: Response<Fresh>){
        let mut res = res.start().unwrap();
        let path = match req.uri {
            RequestUri::AbsolutePath(str) => str,
            _ => "".to_string()
        };
        let mut client = Client::new();
        let mut content = client.get(&format!("{}{}","http://localhost:8080", path))
            .send().unwrap();
        let mut body = Vec::new();
        content.read_to_end(&mut body).unwrap();
        res.write_all(&body[..]).unwrap();
        res.end().unwrap();
    }
}

