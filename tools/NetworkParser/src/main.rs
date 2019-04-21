extern crate rand; // 0.6.5
use rand::Rng;


pub struct Neyron{
    weight: Vec<f32>,
    inputs: Vec<f32>,
    learn_speed: f32,
 
    result: f32,
 
}
impl Neyron {
	pub fn new(weight: usize, inputs: usize, learn_speed: f32)->Neyron{
		let mut weight_: Vec<f32> = Vec::new();
		
		for i in 0..weight{
		    let mut rng = rand::thread_rng();
		    //if rng.gen() { // random bool
		    let mut f: f32 = rng.gen::<f32>();
		    if f < 0.0 {
			while f < -1.0 {
			    f /= 10.0;
			}
		    } else if f > 0.0 {
			while f > 1.0 {
			    f /= 10.0;
			}
		    } else {
			f = -0.5;
		    }
		    weight_.push(f);
		}
		let mut inputs_: Vec<f32> = Vec::new();
		for i in 0..inputs {
			inputs_.push(0.0);
		}
		Neyron { weight: weight_, inputs: inputs_, learn_speed: learn_speed, result: 0.0 }
	}
	pub fn clone(&self)->Neyron{ 
		Neyron { weight: self.weight.clone(), inputs: self.inputs.clone(), learn_speed: self.learn_speed.clone(), result: self.result.clone() } 
	}
}
pub struct BufferNet {
	layers: Vec<Vec<LayerNet>>, // 			 						   [z][x]
	//name: String,
	layers_in_layers: Vec<BufferNet>,
	on_layers_index_to_go_layers_in_layers: Vec<usize>
}
impl BufferNet {
	pub fn new(x: usize, y: usize)->BufferNet{
		//LayerNet::new(y: usize)->LayerNet
		let mut lyrs: Vec<LayerNet> = Vec::new();
		let mut lrss: Vec<Vec<LayerNet>> = Vec::new();
		for i in 0..x.clone(){
			lyrs.push(LayerNet::new(y));
		}
		lrss.push(lyrs);
		BufferNet {
			layers: lrss, 
			layers_in_layers: Vec::new(),
			on_layers_index_to_go_layers_in_layers: Vec::new()
		}
	}
	pub fn add<'a>(&mut self, arg: &'a str, value_x: Vec<usize>) {
		match arg {
			"lx" => {
				let len_: usize = self.layers.len();
				if len_ == 0 {
					panic!("попытка добавить [x] в неинициализированный [z]");
				}
				if value_x.len() == 0 {
					panic!("попытка добавить неинициализированный [x] в [z]");
				}
				let len_in: usize = self.layers[len_ - 1].len();
				if len_in == 0 {
					if value_x.len() < 2 { 
						panic!("попытка создания нового [x] в последнем [z] без указания количества [y]");
					}
					for i in 0..value_x[0] {
						self.layers[len_ - 1].push(LayerNet::new(value_x[1].clone()));
					}
				} else {
					for i in 0..value_x[0] {
						let elem = self.layers[len_ - 1][len_in - 1].clone();
						self.layers[len_ - 1].push(elem.clone());
					}
				}
			}, // to X
			   // просто добавляем иксы в последний z			
			"nz" => {
				self.layers.push(Vec::new());
			}, // new z
			   // пустой z
			"nzx" => {
				let len_: usize = value_x.len();
				if len_ == 0 {
					panic!("попытка создать [z] с неинициализированными [x], возможно пропущен знак?");
				}
				if len_ < 2 {
					panic!("попытка созать [z] не указывая [x] и [y]");
				}
				let x_count: usize = value_x[0].clone();
				let y_count: usize = value_x[1].clone();
				for i in 2..len_ {
					let mut tmp: Vec<LayerNet> = Vec::new();
					for _ in 0..x_count.clone() {
						tmp.push(LayerNet::new(y_count.clone()));
					}
					self.layers.push(tmp);
				}
			}, // new z and x
			   // длина массива - число z, внутри него иксы. первый элемент - количество Х, второй - количество Y
			"tzx" => {
				let len_: usize = value_x.len();
				if len_ == 0 {
					panic!("попытка изменить [z] не указывая индекса");
				}				
				if len_ == 1 {
					panic!("попытка изменить [z] по индексу, не указывая количество добавляемых [x]");
				}
				if len_ == 2 {
					panic!("попытка изменить [z] по индексу, не указывая количество добавляемых [y]");
				}
				if len_ > 3 {
					panic!("не корректное поведение [z]. err: txz[>2]");
				}
				let indx_z: usize  = value_x[0].clone();
				let count_x: usize = value_x[1].clone();
				let count_y: usize = value_x[2].clone();
				
				for i in 0..count_x {
					self.layers[indx_z].push(LayerNet::new(count_y.clone()));
				}
			}, // to Z add x 
			   // при этом раскладе в первом элементе [0]
			   // будет указан index Z
			"aziz"=>{
				let len_: usize = value_x.len();
				if len_ == 0 {
					panic!("попытка изменить [z] in [z] не указывая индекса");
				}
				if len_ == 1 {
					panic!("попытка изменить [z] in [z] не указывая количество [x]");
				}
				if len_ == 2 {
					panic!("попытка изменить [z] in [z] не указывая количество [y]");
				}
				if len_ > 3 {
					panic!("попытка изменить [z] in [z]. слишком много аргументов [in z][x][y][>3]");
				}
				let indx_z: usize  = value_x[0].clone();
				let count_x: usize = value_x[1].clone();
				let count_y: usize = value_x[2].clone();
				
				/*layers_in_layers: Vec<BufferNet>,
	on_layers_index_to_go_layers_in_layers: Vec<usize>*/
				if indx_z >= self.layers.len() {
					panic!("попытка добавить [z] in [z] в несуществующий [z].");
				}
				self.on_layers_index_to_go_layers_in_layers.push(indx_z.clone());
				//pub fn new(x: usize, y: usize)->BufferNet{
				self.layers_in_layers.push(BufferNet::new(count_x.clone(), count_y.clone()));
			}, // add z in z
			   // добавить в layers_in_layers зет, 
			   // порядковый номер зета указан в первом элементе вектора
			_ => {
				panic!("ошибка в передаче строкового параметра");
			},
		}
	}
}
pub struct LayerNet {
	layer: Vec<Neyron>, // нейроны в слою // 						   [y]
}
impl LayerNet {
	pub fn new(count: usize)->LayerNet{
		let mut layer_: Vec<Neyron> = Vec::new();
		for i in 0..count.clone() {
			layer_.push(Neyron::new(count.clone(), count.clone(), 0.001));
			//pub fn new(weight: usize, inputs: usize, learn_speed: f32)->Neyron
		}
		LayerNet { layer: layer_ }
	}
	
	pub fn clone(&self)->LayerNet {
		// let cl: Vec<Neyron> = self.layer;
		let mut r: Vec<Neyron> = Vec::new();
		for item in &self.layer {
			r.push(item.clone());
		}
		LayerNet { layer: r }
	}
}
pub struct LogicalSheme {
	variables_: Vec<Vec<BufferNet>>, //[variables][step] // обращение: [step].[z][x].[y] 
	// 											                        [шаг].[группа][x].[y]
	variables_name: Vec<String>,
	//variables_val : Vec<usize>,
	words: Vec<String>,
}

impl LogicalSheme {
	pub fn search_var(&self, text: String)-> Result<usize, ()>{
		let mut indx: usize = 0;
		for word in self.variables_name.clone() {			
			if word == text { return Ok(indx); }			
			indx += 1;
		} // изменить
		Err(())
	}
	pub fn add_var(&mut self, var_name: String)->usize{
		self.variables_.push(Vec::new());
		self.variables_name.push(var_name);
		self.variables_name.len()
	}
	pub fn edit_var(&mut self, var_name: String, var_value: String)->bool{
		
		let mut indx: usize = match self.search_var(var_name.clone()) {
			Ok(A) => { A },
			Err(e)=> { return false; 0 },
		};
		self.variables_[indx] = Vec::new();
		self.parser(var_value.clone().as_str());
		//parser<'a>(&mut self, mut line_: &'a str ){
		true
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

		let mut open_br: bool 	= false;
		let mut close_br: bool 	= false;
		let mut comment: bool	= false;
		let mut next_tire: bool = false;
		let mut program: bool 	= false; // start - true
		//let mut variable: bool  = false;
		//let mut what_depth: bool = true; // if after '|' digit or symbol => true, else => false
		
		// main: [ 728 -> 300, 300 -> ^1 -> out ]
		
		
		
		
		//let mut layer_len: usize = 0;
		//let mut layer_type: u8 = 0;
		//let mut steps: 
		let mut buffer_text0: String = "".to_string();    // тут обычный буффер
		let mut buffer_text1: String = "".to_string();    // тут число входов
		let mut buffer_text2: String = "".to_string(); 	  // тут временное имя переменной
		let mut variable_index: usize = 0;			      // индекс переменной
		let mut last_op: [u8; 3] = [0; 3];
		
		let mut value_in_z: Vec<usize>	    = Vec::new(); // а тут все значения для Z
		
		let mut to_z_navigation: Vec<usize> = Vec::new(); // навигация по z
														  // где последний элемент - это z в котором сейчас работает,
														  // то есть если такая строка '300,300->300,300|300,300'
														  // то вначале будет первый зет до разделителя группы, 
														  // 300 будет положено в value_in_z, на разделителе будет создан z в z
														  // а второй зет будет после
														  // это позволит создавать сложные архитектуры в одну строку
		// шаги
		//let mut networks: Vec<(usize, usize, usize, usize, usize)> = Vec::new();
		//let mut network_size: usize = 0;

		//fn eq_lite(&self, text: String)->bool {

		let mut last_char: char = '\0';
		for ch in line.chars(){
			if comment && ch == '\n' { comment = false; }
			if comment { continue; }
			// не забудь выделить переменную под хранение [y] и не забывай передавать [x]
			match ch {				
				':' => { 
					//variable = true;
					buffer_text2 = buffer_text0.clone();
					variable_index = self.add_var(buffer_text0.clone());
					buffer_text0 = "".to_string();
				},
				'-' => {
					next_tire = true;
					if buffer_text1 == "".to_string() {
						
					}
				},
				'>' => {
					
				},
				'\''=> { comment = true; },
				'{' => { open_br = true; },
				'}' => { close_br = true;},
				'[' => {
					open_br = true;
				},
				']' => {
					close_br = true;
				},
				'|' => { 
					// по слоям
				},
				',' => {
					let len_: usize = to_z_navigation.len();
					//group = true;
					// •••
					continue;
					//обнаружили
				},
				_ => {					
					buffer_text0.push(ch);
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
            ' It's a cool network
            
			outp: [ ^10 -> out ]
			two:  [ 200, 300 -> 200 | 100 -> out | outp ]
			one:  [ 300, 300 -> ^500 -> outp ]
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

