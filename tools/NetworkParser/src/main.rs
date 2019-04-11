

pub struct Neyron{
    weight: Vec<f32>,
    inputs: Vec<f32>,
    learn_speed: f32,
 
    result: f32,
 
}

pub struct BufferNet {
	layers: Vec<Vec<LayerNet>>, // 			 						   [z][x]
	name: String,
}
pub struct LayerNet {
	layer: Vec<Neyron>, // нейроны в слою // 						   [y]
}

pub struct LogicalSheme {
	variables_: Vec<Vec<BufferNet>>, //[variables][step] // обращение: [step].[z][x].[y] 
	// 											                        [шаг].[группа][x].[y]
	variables_name: Vec<String>,
	words: Vec<String>,
}

impl LogicalSheme {
	pub fn search_var(&self, text: String)-> Result<usize, ()>{
		let mut indx: usize = 0;
		for word in self.variables_name.clone() {			
			if word == text { return Ok(indx); }			
			indx += 1;
		} // изменить
		Err()
	}
	pub fn eq_lite(&self, text: String)->u8{
		let mut indx: u8 = 0;
		for word in self.words.clone() {			
			if word == text { return indx; }			
			indx += 1;
		} // изменить
		indx = 17;
		indx
	}
	pub fn new()->LogicalSheme{
		let mut words: Vec<String> = vec![
			"out".to_string(),//0  // выход сети
			"main".to_string(),//1 // точка входа
			"->".to_string(),//2   // след. шаг
		];
		LogicalSheme { variables_: Vec::new(), variables_name: Vec::new(), words: words }
	}
	pub fn parser<'a>(&mut self, mut line_: &'a str ){
		let line = trim(line_.to_string(), "\t ");

		let mut constants: Vec<&'a str> = Vec::new();

		let mut open_br: bool = false;
		let mut close_br: bool = false;
		let mut comment: bool = false;
		let mut group: bool = false; // group start? 
		//let mut depth_flag: bool = false;
		let mut program: bool = false; // start - true
		//let mut what_depth: bool = true; // if after '|' digit or symbol => true, else => false
		
		// main: [ 728 -> 300, 300 -> ^1 -> out ]
		
		
		
		
		//let mut layer_len: usize = 0;
		//let mut layer_type: u8 = 0;
		//let mut steps: 
		let mut buffer_text0: String = "".to_string();
		let mut buffer_text1: String = "".to_string();
		let mut buffer_text2: String = "".to_string();
		let mut last_op: [u8; 3] = [0; 3];
		// шаги
		//let mut networks: Vec<(usize, usize, usize, usize, usize)> = Vec::new();
		//let mut network_size: usize = 0;

		//fn eq_lite(&self, text: String)->bool {

		let mut last_char: char = '\0';
		for ch in line.chars(){
			if comment && ch == '\n' { comment = false; }
			if comment { continue; }
			match ch {
				' ' => {  },
				'\''=> { comment = true; },
				'{' => { open_br = true; },
				'}' => { close_br = true;},
				'[' => {
					program = true;
				},
				']' => {
					program = false;
				},
				'|' => { 
					// по слоям
				},
				',' => {
					group = true;
					// •••
					continue;
					//обнаружили
				},
				_ => {
					
				},
			}			
			if program { last_char = ch; }
		}
		println!("line: {}", line);
	}
}
//fn eq_lite
 
fn trim<'a>(text: String, to_: &'a str)->String{
    let to_: String = to_.to_string();
    let mut result_: String = String::new();
    for ch in text.chars() {
        let mut t: bool = false;
        for ch1 in to_.chars() {
            if ch == ch1 {
                t = true;
            }
        }
        if !t {
            result_.push(ch.clone());
        }
    } result_
}
 
fn main() {
    println!("Hello, world!");
	let mut t = LogicalSheme::new();
    t.parser("
            ' ThGorge Parser Ver: 0.01
            ' constants:
            out_1: 10
            out_2: 10
            out_3: 35
            out_4: out_1 + out_2
            
			outp:[ ^10 -> out ]
			two: [ 200, 300 -> 200 | 100 -> out | outp ]
			one: [ 300, 300 -> ^500 -> outp ]
            main: [ 728 -> 300, 300 -> one | two ]
			
			'		300
			'	300		500--|
			'		300		 |->10->out
			'728			 |
			'		200	200--|
			'	300		
			'		300	100 out
                ");
}


























