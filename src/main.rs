
extern crate ThGorgeForHabr;
use ThGorgeForHabr::language;
fn main() {
    let mut language: language::Words = language::create();
    language.start_("
object i = 0
loop    
loop
if i > 2
break 
end 
i = i + 1
print i
prt_stact 
end 
i = i + 10
if i > 20
break
end 
end 
"/*
    func say_hello(void)
        print other
        other = other + 1
        if other < 17
            print \"bug\"                          
            say_hello(other)
        end        
    end_func
print i
object other = \"Hello, World from say_hello!\"
other = 10
say_hello(other)
object other2 = 4
other = 2 + (2*other2)
print other
other2 = \"Hello, World from other2!\"
print other2

if 1 & 1
    print \"if ответил true!\"
    if 1 & 1
        print \"второй if ответил true\"
        if 1 & 1
            print \"третий if ответил true\"
        end 
        print \"конец третьего ифа\"    
    end
    print \"конец второго ифа\"
end
print \"конец ифа\"
prt_stact 
"*/.to_string());
}
