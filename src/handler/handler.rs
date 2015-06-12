extern crate hyper;
extern crate core;

use std::io::Write;
use std::vec::Vec;
use self::hyper::server::Handler;
use self::hyper::server::Request;
use self::hyper::server::Response;
use self::hyper::net::Streaming;
use self::hyper::net::Fresh;
use self::core::marker::Send;
use self::core::marker::Sync;


pub trait ReverseProxyHandler: Send+Sync{
    fn perform(&self, Request, &Response<Streaming>) -> Vec<u8>;
}

// impl Handler for ReverseProxyHandler{
//     fn handle(&self, req: Request, res: Response<Streaming>){
//         let mut res = res.start().unwrap();
//         res.write_all(&self.perform(req, res)).unwrap();
//         req.end().unwrap();
//     }
// }
