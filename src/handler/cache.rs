extern crate hyper;

use std::vec::Vec;
use std::io::Write;
use std::collections::HashMap;
use self::hyper::server::Request;
use self::hyper::server::Response;
use self::hyper::net::Fresh;
use self::hyper::server::Handler;
use self::hyper::uri::RequestUri;

pub struct CachingHandler{
    inner: Box<Handler>,
    cache: HashMap<String, Vec<u8>>
}

impl CachingHandler{
    pub fn new(inner: Box<Handler>) -> CachingHandler{
        let map = HashMap::new();
        CachingHandler{
            inner: inner,
            cache: map
        }
    }
}

impl Handler for CachingHandler {
    fn handle<'b, 'a>(&'a self, req: Request<'a, 'b>, res: Response<'a, Fresh>){
        let path = match req.uri {
            RequestUri::AbsolutePath(ref str) => str.clone(),
            _ => "".to_string()
        };
        match self.cache.get(&path){
            Some(content) => {
                let mut res = res.start().unwrap();
                res.write_all(content).unwrap();
                res.end().unwrap();
            },
            None => self.inner.handle(req, res)
        };
    }
}
