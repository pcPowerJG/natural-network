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
	layers_in_layers: Vec<BufferNet>, // на каждый [z] свой 
	//on_layers_in_layers_len: Vec<usize> // глубина
}
impl BufferNet {
	pub fn add_to_depth(&mut self, mut depth_map: Vec<usize>){
		let len: usize = depth_map.clone().len();
		// первый - колво иксов
		// второй - колво игриков
		// третий - последний depth, [z],  точнее
		if len > 3 {
			let index: usize = depth_map[len - 1].clone();
			depth_map.remove(len - 1);
			if self.layers_in_layers.len() > index {
				self.layers_in_layers[index].add_to_depth(depth_map.clone());
			} else {
				let len_: usize = self.layers_in_layers.len();
				let index = index + 1;
				for i in len_..index {
					self.layers_in_layers.push(BufferNet::new_empty());
				} 
				self.layers_in_layers[index].add_to_depth(depth_map.clone());
			}
			//self.add_to_depth(depth_map.clone());
		} else {
			let x_count: usize = depth_map[0];
			let y_count: usize = depth_map[1];
			let z: usize = depth_map[2];
			if self.layers.len() > z {
				for i in 0..x_count {
					self.layers[z].push(LayerNet::new(y_count.clone()));
				}
			} else if self.layers.len() == z {
				self.layers.push(Vec::new());
				for i in 0..x_count {
					self.layers[z].push(LayerNet::new(y_count.clone()));
				}
			} else {
				panic!("неопределённая ошибка в add_to_depth()");
			}
		}
	}
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
			//on_layers_in_layers_len: Vec::new()
		}
	}
	pub fn new_empty()->BufferNet{
		BufferNet {
			layers: Vec::new(), 
			layers_in_layers: Vec::new(),
			//on_layers_in_layers_len: Vec::new()
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
					panic!("попытка создать [z] с неинициализированными [y]");
				}
				let y_count: usize = value_x[0].clone();
				//let y_count: usize = value_x[1].clone();
				for i in 1..len_ {
					let mut tmp: Vec<LayerNet> = Vec::new();
					for _ in 0..value_x[i].clone() {
						tmp.push(LayerNet::new(y_count.clone()));
					}
					self.layers.push(tmp);
				}
			}, // new z and x
			   // длина массива - число z, внутри него иксы. первый элемент - количество Y
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
				//self.on_layers_in_layers_len[indx_z] += 1;
				//pub fn new(x: usize, y: usize)->BufferNet{
				self.layers_in_layers.push(BufferNet::new(count_x.clone(), count_y.clone()));
			}, // add z in z
			   // добавить в layers_in_layers зет, 
			   // порядковый номер зета указан в первом элементе вектора
			"azizd"=>{
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
				if len_ == 3 {
					panic!("попытка изменить [z] in [z], не указывая глубину [z] in [z]");
				}
				self.add_to_depth(value_x.clone());
				/*if len_ > 4 {
					panic!("попытка изменить [z] in [z]. слишком много аргументов [in z][x][y][depth][>4]");
				}
				let indx_z: usize  = value_x[0].clone();
				let count_x: usize = value_x[1].clone();
				let count_y: usize = value_x[2].clone();
				let depth_: usize  = value_x[3].clone();*/
				/*layers_in_layers: Vec<BufferNet>,
	on_layers_index_to_go_layers_in_layers: Vec<usize>*/
				/*if indx_z >= self.layers.len() {
					panic!("попытка добавить [z] in [z] в несуществующий [z].");
				}*/

			}, // add z in z to depth
			   //
			   //
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
	to_nav_map: Vec<ToNavMap>,
	variables_name: Vec<String>,
	//variables_val : Vec<usize>,
	words: Vec<String>,
}
pub struct ToNavMap {
	navigation_on_separate_lines: Vec<Vec<usize>>, // навигация по отдельным линиям
	// это индекс родителя (пуповина сына) 
	// внутри шага [step] 
	// для каждого z [step][z]	

	//[step][порядковый номер z текущий] его значение -> на какой порядковый отсылается
}
impl ToNavMap {
	pub fn add_and_new_step(&mut self, value: usize){
		let len: usize = self.navigation_on_separate_lines.len();
		self.navigation_on_separate_lines.push(Vec::new());
		if len != 0 {
			self.navigation_on_separate_lines[len - 1].push(value.clone());
		} else {
			self.navigation_on_separate_lines[0].push(value.clone());
		}
	}
	pub fn add_to_step(&mut self, step: usize, value: usize){		
		let len: usize = self.navigation_on_separate_lines.len();
		println!("len: {}", len.clone());
		if step == len {
			self.add_and_new_step(value.clone());
		} else if step > len {
			panic!("попытка добавить значение в карту. шага не существует.");
		} else {
			self.navigation_on_separate_lines[step].push(value);
		}
	}
	pub fn new()->ToNavMap{
		ToNavMap { navigation_on_separate_lines: Vec::new() }
	}
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
		self.variables_name.len() - 1
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
		LogicalSheme { 
			variables_: Vec::new(), 
			variables_name: Vec::new(), 
			to_nav_map: Vec::new(), 
			words: words 
		}
	}
	pub fn debug(&self){
		println!("variables count: {}", self.variables_.len());
					for i in 0..self.variables_.len(){
						println!("step count[ {} ] in variables {}", 
																	self.variables_[i].len(),
																	self.variables_name[i].clone());
						for st in 0..self.variables_[i].len() {
							let l: usize = self.variables_[i][st].layers.len();
							println!("[z]: {}", l.clone());
							for k in 0..l {
								let l_: usize = self.variables_[i][st].layers[k].len();
								println!("[x]: {}", l_.clone());
								for m in 0..l_.clone() {
									println!("[y]: {}", self.variables_[i][st].layers[k][m].layer.len());
								}
							}						
						}
						println!("---------------map----------------");
						if self.to_nav_map.len() == 0 {
							println!("self.to_nav_map == 0");
							continue;
						}
						let len__s: usize = self.to_nav_map[i].navigation_on_separate_lines.len();
						if len__s == 0{
							println!("self.navigation_on_separate_lines.len() = 0");
						} else { 
							println!("self.navigation_on_separate_lines.len() = {}", len__s.clone());
						}
						for k in 0..len__s {
							let to____z_: usize = self.to_nav_map[i].navigation_on_separate_lines[k].len();
							for l in 0..to____z_.clone(){
								println!("[z]: [to_z]\n[ {} ]: [ {} ]", k.clone(), 
										self.to_nav_map[i].navigation_on_separate_lines[k][l].clone());
							}
						}
					}
					println!("-----------debug end---------------")
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
		let mut buffer_text: String = "".to_string(); 			  // тут обычный буффер
		let mut y_count: usize = 0;		         	  			  // тут число входов
		let mut temp_name_variable: String = "".to_string(); 	  // тут временное имя переменной
		let mut variable_index: usize = 0;			  			  // индекс переменной
		//let mut last_op: [u8; 3] = [0; 3];
		
		let mut value_in_z: Vec<usize> = Vec::new(); // а тут все значения для Z (иксы)
		
		let mut step_value: usize = 0;					  // навигация по шагам
		//let mut to_z_navigation: Vec<usize> = Vec::new(); // навигация по z
		let mut index_last_z: usize = 0;
		let mut index_this_z: usize = 0;
		// шаги
		//let mut networks: Vec<(usize, usize, usize, usize, usize)> = Vec::new();
		//let mut network_size: usize = 0;

		//fn eq_lite(&self, text: String)->bool {

		let mut last_char: char = '\0';
		for ch in line.chars(){
// self
//variables_: Vec<Vec<BufferNet>>, //[variables][step] // обращение: [step].[z][x].[y] 
// 											                        [шаг].[группа][x].[y]
//variables_name: Vec<String>,

			if comment && ch == '\n' { comment = false; }
			if comment { continue; }
			// не забудь выделить переменную под хранение [y] и не забывай передавать [x]
			println!("char: {}", ch.clone());
			println!("step: {}", step_value.clone());
			println!("index_last_z: {}\nindex_this_z: {}\ny_count: {}\nnext_tire: {}", 
			index_last_z.clone(), index_this_z.clone(), y_count.clone(), next_tire.clone());
			println!("buffer_text: {}\nvalue_in_z: {:?}", buffer_text.clone(), value_in_z.clone());
			println!("---------------------------------------");
			match ch {	
				'D' => { 
					self.debug();
				},
				'e' => { return; },
				'\''=> { comment = true; },			
				':' => { 
					//variable = true;
					temp_name_variable = buffer_text.clone();
					variable_index = self.add_var(buffer_text.clone());
					self.to_nav_map.push(ToNavMap::new());
					buffer_text = "".to_string();
				},
				'-' => {
					next_tire = true;
					//buffer_text = "".to_string();					
				},
				'>' => {
					if next_tire {
						if y_count == 0 {
							y_count = match buffer_text.trim().parse::<usize>() {
								Ok(A) => { A },
								Err(e)=> { panic!("значение количества входов должно быть числовым."); 0 },
							};
							self.variables_[variable_index] = Vec::new();
							self.variables_[variable_index].push(BufferNet::new(1, y_count.clone()));
							self.variables_[variable_index].push(BufferNet::new_empty());
							//self.to_nav_map[variable_index] = ToNavMap::new();
							//println!("input value_in_z: {:?}", value_in_z.clone());
							value_in_z = Vec::new();
							next_tire = false;
							buffer_text = "".to_string();	
							continue;
						}
						if index_last_z != 0 {
							for _ in 0..index_this_z.clone() {
								self.to_nav_map[variable_index].add_to_step(step_value, index_last_z.clone());
							}
							//println!("")
						} else {
							self.to_nav_map[variable_index].add_to_step(step_value, 0);
						}
						if index_this_z != 0 {
							let value_: usize = match buffer_text.trim().parse::<usize>() {
								Ok(A)=>{ A },
								Err(e)=>{ panic!("не получилось прочитать число около запятой."); 0 },
							};
							buffer_text = "".to_string();				
							index_this_z += 1;							
							value_in_z.push(value_.clone());
						}
						let len_: usize = self.variables_[variable_index].len();
						value_in_z.insert(0, y_count.clone());
						println!("step value_in_z([y][x][x]): {:?}", value_in_z.clone());
						self.variables_[variable_index][len_ - 1].add("nzx", value_in_z.clone());
						
						self.variables_[variable_index].push(BufferNet::new_empty());
						//BufferNet::new_empty
						// тут не просто обнуление!
						value_in_z = Vec::new();
						//to_z_navigation = Vec::new();
						buffer_text = "".to_string();
						step_value += 1;
						next_tire = false;
					}
				},				
				',' => {
					//let len_: usize = to_z_navigation.len();
					//group = true;
					// •••
					let value_: usize = match buffer_text.trim().parse::<usize>() {
						Ok(A)=>{ A },
						Err(e)=>{ panic!("не получилось прочитать число около запятой."); 0 },
					};
					buffer_text = "".to_string();				
					index_this_z += 1;							
					value_in_z.push(value_.clone());					
				},
				'|' => { 
					let len_: usize = self.variables_[variable_index].len();
					value_in_z.insert(0, y_count.clone());
					self.variables_[variable_index][len_ - 1].add("nzx", value_in_z.clone());
					// step_value: usize 		// навигация по шагам
					// to_z_navigation:       ; // навигация по z
					for _ in 0..index_this_z.clone() {
						self.to_nav_map[variable_index].add_to_step(step_value, index_last_z.clone());
					}
					/*
						pub fn add_and_new_step(&mut self, value: usize){
						pub fn add_to_step(&mut self, step: usize, value: usize){
					*/
					//to_nav_map: Vec<ToNavMap>,

					// запись в Map
					index_last_z += 1;					
					index_this_z = 0;
				},
				' ' | '\t'=> {
					continue;
				},
				'{' => { 
					open_br = true; 
					buffer_text = "".to_string();
				},
				'}' => { close_br = true;},
				'[' => { 
					open_br = true; 
					buffer_text = "".to_string();
				},
				']' => {
					close_br = true;
				},
				_ => {					
					buffer_text.push(ch);
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
	' comment
	main: 5->5,5->De");
    /*t.parser("
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
                ");*/
}

