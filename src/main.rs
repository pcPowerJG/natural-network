
extern crate ThRustGorge;
use ThRustGorge::Language;

use std::net::*;
use std::thread;
use std::io::Write;
use std::io::Read;

fn main() {
    //println!("Hello, world!");
    let mut program_text: Vec<String> = Vec::new();
	let mut o = Language::on_create();
    /*let listener = TcpListener::bind("127.0.0.1:9273").unwrap();
    println!("listening started, ready to accept");
    //fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize>
    for stream in listener.incoming() {
        thread::spawn(move| | {
            let mut stream = match stream {
                Ok(A) => { A },
                Err(e) => { 
                    println!("ошибка получения пользователя."); 
                    println!("{:?}", e); 
                    panic!("ошибка получения потока пользователя"); 
                    stream.unwrap()
                },
            };
            let mut bytes: Vec<u8> = Vec::new();
            match stream.read_to_end(&mut bytes) {
                Ok(A) => {  },
                Err(e)=> { println!("не удалось прочитать сообщение"); },
            };
            let row_: String = match String::from_utf8(bytes) {
                Ok(A) => { A },
                Err(e)=> { println!("ошибка преобразования строки"); "".to_string() },
            };
            if row_ != "".to_string() {

            }
        });
    }*/
    //self.get_all_func(text.clone(), temp_to_func.clone());
	o.get_("


object lib=hello_ivi.out
object func_=say_hello
object other =12
if other > 10.5
    print func_
end
import(lib, lib1, lib2)
    extern_func(lib, func_, other, return_variable)
    extern_func(lib, func_, other)
    extern_func(lib, func_)
close_import(lib, lib1, lib2)

".to_string(), "".to_string(), false, 0); /*
object toto_other=21
object gb =Good Bue looper_
func hello(arg1)

    if other < to_other
        hello_(other)        
    end
    print arg1
end_func

func looper_(arg1)
    loop
        if other < toto_other
            break
        end
        other = other + 2 : math
    end_loop
end_func

func hello_(arg1)
    other = other + 1 : math
    if other < to_other
        hello_(other)        
    end
end_func

hello(other)

println!("\n\nall: \n\n{}", o.get_all_func("
object other=0
object to_other=10
object toto_other=21
object gb =Good Bue looper_
func hello(arg1)
    print arg1
end_func
func looper_(arg1)
    loop
        if other > to_other
            break
        end
        other = other + 1 : math
        end_loop
    hello(other)
    print gb
end_func

looper_(other)
".to_string(), "hello,looper_".to_string()));*/
	/*o.get_("
object other=1
object to_other=10
object toto_other=5
object gb =Good Bue looper_

func plus_one(arg2)
    loop
        if other > toto_other 
            break
        end
        other = other + 2 : math
    end_loop
end_func

func hello(arg1)
    print arg1
end_func
func looper_(arg1)
    loop#plus_one
        if other > to_other
            break
        end
        plus_one(other)        
        other = other + 1 : math
    end_loop

    hello(other)
    print gb

end_func

looper_(other)

".to_string(), "".to_string(), false, 0);*/
	//o.unsafe_print_bufs_and_vals();	
    //dbg!(o.get_value("obj2".to_string()));

    /*
        \n server serv0
        \n server serv1 =127.0.0.1:8080
        \n obj1 = serv1 
        \n obj1 = obj2
        \n object obj3 
        \n obj3 = ney1 
        \n create ney0 [5] 
        \n ney1 = ney0
        \n print ney1
        \n ney02 = obj3
        \n ney1 = { 1.2 1.4 1.9 2.0 -5.6 }
        \n remove obj3
        \n print hello
        \n hello = ney1
        \n print hello
        \n hello =Hello, World!== '\" lasoqw U!@` 2?
        \n print hello 
        \n print obj2
        \n print ney1
        \n print ney02
        \n print ney0
        \n print serv1
        \n print obj4    
    */
}
