extern crate hyper;

use std::vec::Vec;
use std::io::Write;
use std::collections::HashMap;
use std::sync::Mutex;
use self::hyper::server::Request;
use self::hyper::server::Response;
use self::hyper::net::Fresh;
use self::hyper::server::Handler;
use self::hyper::uri::RequestUri;
use self::hyper::net::Streaming;
use super::handler::ReverseProxyHandler;

pub struct CachingHandler{
    inner: Box<ReverseProxyHandler>,
    cache: Mutex<HashMap<String, Vec<u8>>>
}

impl CachingHandler{
    pub fn new(inner: Box<ReverseProxyHandler>) -> CachingHandler{
        CachingHandler{
            inner: inner,
            cache: Mutex::new(HashMap::new())
        }
    }
}

impl ReverseProxyHandler for CachingHandler {
    fn perform(&self, req: Request, res: &Response<Streaming>) -> Vec<u8>{
        let path = match req.uri {
            RequestUri::AbsolutePath(ref str) => str.clone(),
            _ => "".to_string()
        };
        let mut cache = self.cache.lock().unwrap();
        match cache.get(&path) {
            Some(content) => return content.clone(),
            None => ()
        };
        let content =  self.inner.perform(req, res);
        cache.insert(path, content.clone());
        return content;
    }
}

impl Handler for CachingHandler {
     fn handle(&self, req: Request, res: Response<Fresh>){
         let mut res = res.start().unwrap();
         let content = self.perform(req, &res);
        res.write_all(&content[..]).unwrap();
        res.end().unwrap();
    }
}
