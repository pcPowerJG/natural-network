
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
    let mut o = Language::on_create();
    for stream in listener.incoming() {
        let stream = stream.unwrap(); 
        o = handle_client(stream, o);
    }


	//o.get_("load ".to_string());
    //println!("{}", o.get_("search _ in table \n".to_string()));
    //println!("{}", o.get_("GIVE_ALL_TABLE_NAME \n".to_string()));
}

fn handle_client(mut stream: TcpStream, mut o: Language::Words)->Language::Words {
    // ...
    println!("new connection {:?}", stream);
    
    let b: [u8; 105] = [0; 105];
    let mut buffer: [u8; 1024] = [0; 1024];
    let mut u: usize = 0;
    loop {
        std::thread::sleep(time::Duration::from_millis(100));
        stream.read(&mut buffer);
        if buffer.starts_with(&b) { if u > 300 { println!("client say: bue"); break; } else { u += 1; } } else {
            let mut b: Vec<u8> = Vec::new();            
            for i in 0..1024 {
                if buffer[i] != 0 {
                    b.push(buffer[i]);                    
                }
            }
            let input = match String::from_utf8(b) {
                Ok(A) => { A }, // если не принимает сообщения - ставь больше задержку передачи между ними.
                Err(e) => { println!("ERROR: {:?}", e); "\n\n".to_string() },
            };
            let result = o.get_(input);
            stream.write(result.as_bytes());
            
            buffer = [0; 1024];
        }
    } o
}