
extern crate ThRustGorge;
use ThRustGorge::Language;

fn main() {
    println!("Hello, world!");
	let mut o = Language::on_create();
	o.get_("create ney0 [5] \n create ney1 { 1.6, 1.8, 1.20 } \n object obj1 \n object obj2 = fl,as;lmdas;ldma;sld \n obj1 = obj2\n server serv0\nserver serv1 =127.0.0.1:8080\n obj1 = serv1 \n object obj3 \n obj3 = ney1 \n ney0 = ney1\n".to_string());
}
