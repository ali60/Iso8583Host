//extern crate Iso8583;
//use Iso8583::ThreadPool;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use std::thread;
use std::time::Duration;

use iso8583_parser;

pub fn start_listening(	address: String) {

    println!("start listening on {}", address);
   let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }

}

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 1024*10];

    stream.read(&mut buffer).unwrap();

    let mut str_buffer =  String::from_utf8_lossy(&buffer[..]);
	let mut req: iso8583_parser::transaction=   iso8583_parser::parse_request(str_buffer.to_string());
   let mut response:String = iso8583_parser::generate_response(&req);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();   
}


