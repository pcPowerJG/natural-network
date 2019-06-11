
extern crate ThRustGorge;
use ThRustGorge::Language;

fn main() {
    //println!("Hello, world!");
	let mut o = Language::on_create();
	//pub fn unsafe_funtion_memory_add(&mut self, name: String, value: String, type_: usize){
	//o.unsafe_funtion_memory_add("array1.2".to_string(), "".to_string(), 5);
	//o.unsafe_funtion_memory_add("one".to_string(), "".to_string(), 1);
	//o.unsafe_funtion_memory_add("two".to_string(), "".to_string(), 2);
	//o.unsafe_funtion_memory_add("1wssw".to_string(), "".to_string(), 2);
	//pub fn get_index(&self, mut name: String) -> Result<usize , ()> {
	//println!("{}", o.get_index("array1[\"one\"]".to_string()).unwrap().to_string());
	/*o.get_("perc ney02        
        \n object obj4 = ney02
        \n print obj4
	\n obj4 = { 1.9, 1.20, 20.225 }\0
	\n print obj4
	\n ney02 = obj4
	\n obj4 = ney02
        \n print obj4        
        \n".to_string());*/		
	/*o.get_("perc ney02 
        \n object hello = Hello, World!\0
        \n object obj1 
        \n print hello
        \n object obj2 = Other Text!\0
        \n obj1 = obj2
        \n perc ney1 { 1.6, 1.8, 1.20 }         
        \n obj1 = ney1
        \n print obj2
        \n print obj1
	\n ney02 = ney1
        \n object obj4 = ney02
        \n print obj4
	\n obj4 = { 1.9, 1.20, 20.225 }\0
	\n ney02 = obj4
	\n obj4 = ney02
        \n print obj4
        \n server serv = 192.168.0.2
        \n server serv_= serv
        \n print serv
        \n print serv_
        \n".to_string());*/
	o.get_("
struct str1
	object hello
	object hello1
	object hello2
	object hello3
	object hello4
end
object other =other object\0
print other

func function(arg1, arg2)\n
        print arg1\n
end_func\n


array ar = first, second, tree\n
str1[\"hello\"] = 123\n
ar[0] = ar[1]\n
ar[1] = str1[\"hello\"]
object obj_oth\n
".to_string(), "".to_string());
o.get_("
struct str1
	object hello
	object hello1
	object hello2
	object hello3
	object hello4
end
object other =other object\0
print other

func function(arg1, arg2)\n
        print arg1\n
        print arg2\n        
end_func\n
array ar = first, second, tree\n
".to_string(), "function|other|other;arg1|arg2".to_string());
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
