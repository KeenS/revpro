extern crate hyper;

use std::vec::Vec;
use self::hyper::server::Request;
use self::hyper::server::Response;
use self::hyper::net::Streaming;


pub trait ReverseProxyHandler: Send+Sync{
    fn perform(&self, Request, &Response<Streaming>) -> Vec<u8>;
}

// impl Handler for HTTPHandler {
//      fn handle(&self, req: Request, res: Response<Fresh>){
//          let mut res = res.start().unwrap();
//          let content = self.perform(req, &res);
//         res.write_all(&content[..]).unwrap();
//         res.end().unwrap();
//     }
// }

