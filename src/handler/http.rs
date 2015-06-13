extern crate hyper;

use std::io::{Write,Read};
use std::vec::Vec;
use self::hyper::Client;
use self::hyper::server::Request;
use self::hyper::server::Response;
use self::hyper::server::Handler;
use self::hyper::net::Fresh;
use self::hyper::net::Streaming;
use self::hyper::uri::RequestUri;
use super::handler::ReverseProxyHandler;

pub struct HTTPHandler;

impl ReverseProxyHandler for HTTPHandler {
    fn perform(&self, req: Request, _: &Response<Streaming>) -> Vec<u8> {
        let path = match req.uri {
            RequestUri::AbsolutePath(str) => str,
            _ => "".to_string()
        };
        let mut client = Client::new();
        let mut content = client.get(&format!("{}{}","http://localhost:8080", path))
            .send().unwrap();
        let mut body = Vec::new();
        content.read_to_end(&mut body).unwrap();
        return body;
    }
}

impl Handler for HTTPHandler {
     fn handle(&self, req: Request, res: Response<Fresh>){
         let mut res = res.start().unwrap();
         let content = self.perform(req, &res);
        res.write_all(&content[..]).unwrap();
        res.end().unwrap();
    }
}

