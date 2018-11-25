
extern crate nGorgeDB;
use nGorgeDB::Language;

use std::io::Read;
use std::io::Write;
use std::net::*;

use std::{thread, time};

fn main() {
    //let mut o = Language::on_create();

    let listener = TcpListener::bind("127.0.0.1:8091").unwrap();

    // accept connections and process them serially
    for stream in listener.incoming() {
        let stream = stream.unwrap(); 
        handle_client(stream);
    }


	//o.get_("load ".to_string());
    //println!("{}", o.get_("search _ in table \n".to_string()));
    //println!("{}", o.get_("GIVE_ALL_TABLE_NAME \n".to_string()));
}

fn handle_client(mut stream: TcpStream) {
    // ...
    println!("new connection");
    let mut o = Language::on_create();
    let b: [u8; 105] = [0; 105];
    let mut buffer: [u8; 8024] = [0; 8024];
    loop {
        std::thread::sleep(time::Duration::from_millis(100));
        stream.read(&mut buffer);
        if buffer.starts_with(&b) { } else {
            let input = match String::from_utf8(buffer.to_vec()) {
                Ok(A) => { A },
                Err(e) => { println!("ERROR: {:?}", e); "\n\n".to_string() },
            };
            let result = o.get_(input);
            if result != String::new() {
                stream.write(result.as_bytes());
            }
        }
    }
}