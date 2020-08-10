
extern crate ThGorgeForHabr;
use ThGorgeForHabr::language;
fn main() {
    let mut language: language::Words = language::create();
    language.start_("
    func say_hello(void)
        print void        
    end_func

object other = \"Hello, World from say_hello!\"
say_hello(other)
object other2 = 4
other = 2 + (2*other2)
print other
other2 = \"Hello, World from other2!\"
print other2

if (1 == !0) & 0
    print other
end
".to_string());
}
