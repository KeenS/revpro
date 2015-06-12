extern crate hyper;

use std::io::Write;
use std::vec::Vec;
use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;
use std::io::Read;
use hyper::Client;
use hyper::header::Connection;



fn hello(_: Request, res: Response<Fresh>){
    let mut res = res.start().unwrap();

    let mut client = Client::new();
    let mut content = client.get("http://localhost:8080")
        .send().unwrap();;
    let mut body = Vec::new();
    content.read_to_end(&mut body).unwrap();
    println!("{:?}", body);
    res.write_all(&body[..]).unwrap();
    res.end().unwrap();
}

fn main(){
    Server::http(hello).listen("127.0.0.1:3000").unwrap();

}
