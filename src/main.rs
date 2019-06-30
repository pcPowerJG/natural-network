
extern crate ThRustGorge;
use ThRustGorge::Language;

extern crate libc;
use libc::*;

use std::ffi::CString;
use std::ptr;
use std::ffi::CStr;

use std::net::*;
use std::thread;
use std::io::Write;
use std::io::Read;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io;

//#[link(name = "math")]
#[link(name="open_file")]
extern {
	//char* read_file(char* path)
	fn read_file(path: *const libc::c_char) -> *const libc::c_char;	
}

fn main() {
    //println!("Hello, world!");
	let mut o = Language::on_create();    

    // Prints each argument on a separate line
    for argument in env::args() {
        //println!("args: {}", (argument as String));
        let arg1 = (argument as String);
        let end_f: Vec<&str> = arg1.as_str().split('.').collect();
        if end_f.len() > 1 && end_f[1].clone() == "gorge" {
            // path: (argument as String)
            let mut path: String = end_f[0].to_string();
            path.push('.');
            path += end_f[1].clone();
            {                
                let mut contents = String::new();
                unsafe {
                    let result = match CStr::from_ptr(read_file(CString::new(path).expect("ошибка передачи аргумента. ошибка преобразования.").as_ptr())).to_str() {
                        Ok(A) => { A },
                        Err(e)=> { panic!("не удалось открыть файл, ошибка преобразования строк. файл должен быть в кодировке utf-8"); "" },
                    };		
                    contents = result.to_string();
                }
                o.get_(contents, "".to_string(), 0);  
            }
        }
    }   
}
