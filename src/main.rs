
extern crate ThRustGorge;
use ThRustGorge::Language;

fn main() {
    //println!("Hello, world!");
	let mut o = Language::on_create();
	println!("{}", o.get_("
func hello(arg1)
    print one
end_func

object other=0
object to_other=3
object one=To One Cirkle
object two=To Two Cirkle

loop
    other = other + 3 : math    
    if other > to_other
        hello(other)
        break
    end
end_loop#

".to_string(), "".to_string(), false, 0));/*
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
