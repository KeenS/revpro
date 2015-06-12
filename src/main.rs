extern crate hyper;
extern crate reverse_proxy;

use hyper::Server;
use reverse_proxy::handler::http::HTTPHandler;
use reverse_proxy::handler::cache::CachingHandler;

fn main(){
    Server::http(HTTPHandler).listen("127.0.0.1:3000").unwrap();

}
