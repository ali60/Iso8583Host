use std::thread;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::time::Duration;
 
use iso8583_parser;

pub fn start_listening(	address: String) {

    println!("start listening on {}", address);
   let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
		thread::spawn(|| {
            handle_connection(stream);
        });
//        handle_connection(stream);
    }

}

//parse the request and send the response 
fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 1024*10];

    stream.read(&mut buffer).unwrap();

    let mut str_buffer =  String::from_utf8_lossy(&buffer[..]);
	println!("request transaction: {}",str_buffer.trim());
	let mut req: iso8583_parser::transaction=   iso8583_parser::parse_request(str_buffer.to_string());
   let mut response:String = iso8583_parser::generate_response(&req);
	println!("response transaction: {}",response);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();   
}


