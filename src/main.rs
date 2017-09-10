#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;
extern crate quick_xml;
//extern crate serde_derive;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::str;
use serde_xml_rs::deserialize;


mod server;
mod iso8583_parser;

struct Configuration {
    address: String,
    port: String,
}


fn read_configuration() -> Configuration 
{
    use quick_xml::reader::Reader;
    use quick_xml::events::Event;
    let mut cfg: Configuration = Configuration { address: String::new (), port: String::new () };
	let mut file = File::open("config.xml").expect("config file not found");
	let mut contents = String::new();
		file.read_to_string(&mut contents)
			.expect("something went wrong reading the file");
    let mut reader = Reader::from_str(&mut contents);
    reader.trim_text(true);

    let mut bind_address = Vec::new();
    let mut port = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) if e.name() == b"bind_address" => {
//                    let ss = reader.read_text(b"bind_address").expect("Cannot decode text value");
                bind_address.push(
                    reader
                        .read_text(b"bind_address", &mut Vec::new())
                        .expect("Cannot decode text value"),
                );
				cfg.address =bind_address.pop().unwrap();
              println!("{:?}", cfg.address);
            }
            Ok(Event::Start(ref e)) if e.name() == b"port" => {
                port.push(
                    reader
                        .read_text(b"port", &mut Vec::new())
                        .expect("Cannot decode text value"),
                );
				cfg.port =port.pop().unwrap();
                println!("{:?}", cfg.port);
            }
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
        buf.clear();
    }
	cfg
}

 
fn main() {
	let config :Configuration;
    config = read_configuration();
	let listening_address = format!("{}:{}", config.address, config.port);
//	iso8583_parser::read_iso_xml();
	server::start_listening(listening_address);
}
