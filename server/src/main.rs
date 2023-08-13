#![allow(unused)]

use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

pub fn main() {
    println!("{}", config::url);
    let listener = TcpListener::bind(config::bind).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle(stream);
    }
}

fn handle(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let req = buf_reader.lines().next().unwrap().unwrap();
    println!("{}", req);
    http::GETParser::new().parse(&req).unwrap();
}
