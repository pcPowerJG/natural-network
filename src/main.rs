
extern crate ThRustGorge;
use ThRustGorge::Language;

fn main() {
    println!("Hello, world!");
	let mut o = Language::on_create();
	o.get_("create ney02 { }
        \n create ney0 [5] 
        \n create ney1 { 1.6, 1.8, 1.20 } 
        \n object obj1 
        \n object obj2 = fl,as;lmdas;ldma;sld 
        \n obj1 = obj2
        \n server serv0
        \n server serv1 =127.0.0.1:8080
        \n obj1 = serv1 
        \n object obj3 
        \n obj3 = ney1 
        \n ney1 = ney0
        \n ney02 = obj3
        \n ney1 = { 1.2 1.4 1.9 2.0 -5.6 }
        \n".to_string());
}
