
extern crate ThRustGorge;
use ThRustGorge::Language;

fn main() {
    //println!("Hello, world!");
	let mut o = Language::on_create();
	//pub fn unsafe_funtion_memory_add(&mut self, name: String, value: String, type_: usize){
	o.unsafe_funtion_memory_add("array1.2".to_string(), "".to_string(), 5);
	o.unsafe_funtion_memory_add("one".to_string(), "".to_string(), 1);
	o.unsafe_funtion_memory_add("two".to_string(), "".to_string(), 2);		
	o.get_("perc ney02 
        \n object hello = Hello, World!\0
        \n object obj1 
        \n print hello
        \n object obj2 = Other Text!\0
        \n obj1 = obj2
        \n perc ney1 { 1.6, 1.8, 1.20 }         
        \n obj1 = ney1
        \n print obj2
        \n print obj1
        \n object obj4 = ney02
        \n print obj4
        \n server serv = 192.168.0.2
        \n server serv_= serv
        \n print serv
        \n print serv_
        \n".to_string());
	//o.unsafe_print_bufs_and_vals();
    dbg!(o.get_value("obj2".to_string()));

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
