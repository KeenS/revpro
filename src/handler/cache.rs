extern crate hyper;

use std::vec::Vec;
use std::io::Write;
use std::collections::HashMap;
use self::hyper::server::Request;
use self::hyper::server::Response;
use self::hyper::net::Fresh;
use self::hyper::server::Handler;
use self::hyper::uri::RequestUri;
use super::handler::ReverseProxyHandler;
use self::hyper::net::Streaming;

pub struct CachingHandler{
    inner: Box<ReverseProxyHandler>,
    cache: HashMap<String, Vec<u8>>
}

impl CachingHandler{
    pub fn new(inner: Box<ReverseProxyHandler>) -> CachingHandler{
        let map = HashMap::new();
        CachingHandler{
            inner: inner,
            cache: map
        }
    }
}

impl ReverseProxyHandler for CachingHandler {
    fn perform(&self, req: Request, res: Response<Streaming>) -> Vec<u8>{
        let path = match req.uri {
            RequestUri::AbsolutePath(ref str) => str.clone(),
            _ => "".to_string()
        };
        match self.cache.get(&path){
            Some(&content) => return content,
            None => return self.inner.perform(req, res)
        };

    }
}

impl Handler for CachingHandler {
     fn handle(&self, req: Request, res: Response<Fresh>){
        let mut res = res.start().unwrap();
        res.write_all(&self.perform(req, res)[..]).unwrap();
        res.end().unwrap();
    }
}
