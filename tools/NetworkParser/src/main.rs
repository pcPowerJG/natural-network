
extern crate rand; // 0.6.5
use rand::Rng;

pub struct Neyron{
    weight: Vec<f32>,
    //inputs: Vec<f32>,
    learn_speed: f32,
 
    //result: f32,
 
}
impl Neyron {
	pub fn weight_count(&self)->usize{
		self.weight.len()
	}
	pub fn new(weight: usize, learn_speed: f32)->Neyron{
		println!("weight_count: {}", weight);
		let mut weight_: Vec<f32> = Vec::new();
		
		for i in 0..weight{
		    let mut rng = rand::thread_rng();
		    //if rng.gen() { // random bool
		    /*let mut f: f32 = rng.gen::<f32>();
			let h: f32 = rng.gen::<f32>();
			if h > 0.6 {
				f *= -1.0;
			}
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
		    }*/ let f: f32 = 0.5;
		    weight_.push(f);
		}
		/*let mut inputs_: Vec<f32> = Vec::new();
		for i in 0..inputs {
			inputs_.push(0.0);
		}*/
		Neyron { 
			weight: weight_, 
			//inputs: inputs_, 
			learn_speed: learn_speed, 
			//result: 0.0 
		}
	}
	pub fn new_input_neyron(inputs: usize)->Neyron {
		let mut weight_: Vec<f32> = Vec::new();
		let learn_speed: f32 = 1.0;
		for i in 0..inputs{		    
		    //let f: f32 = 1.0;		    
		    weight_.push(1.0);
		}
		let mut inputs_: Vec<f32> = Vec::new();
		for i in 0..inputs {
			inputs_.push(0.0);
		}
		Neyron { 
			weight: weight_,
			//inputs: inputs_,
			learn_speed: learn_speed, 
			//result: 0.0 
		}
	}
	pub fn clone(&self)->Neyron{ 
		Neyron { 
			weight: self.weight.clone(), 
			//inputs: self.inputs.clone(), 
			learn_speed: self.learn_speed.clone(), 
			//result: self.result.clone() 
		} 
	}
	pub fn on_error(&mut self, tr_value: Vec<f32>) {
		// A = L(E/x)
		// где 
		// A - значение на которое меняем веса
		// L - скорость обучение (для более точного ответа сети)
		// E - ошибка; E = верный ответ - ответ сети
		// х - вес связи
		
	}
	// fn() and get_set result
	pub fn answer(&self, inputs: Vec<f32>)->f32 {
		if inputs.len() > self.weight.clone().len() {
			panic!("input information > weight.len()");			
		}
		let mut result_: f32 = 0.0;
		//let one: f32 = 1.0;
		

		for i in 0..inputs.len() {
			// f32
			// pub fn powi(self, n: i32) -> f32
			// pub fn powf(self, n: f32) -> f32
			// pub fn sqrt(self) -> f32
			// pub fn exp(self) -> f32			
			/*	
				//Returns e^(self), (the exponential function).
				let one = 1.0f32;
				// e^1
				let e = one.exp();
				println!("e: {}", e);
				// > "e: 2.7182817"
			*/
			result_ += self.weight[i].clone() * inputs[i];
		} 
		let exp: f32 = (result_.clone() * -1.0).exp();
		if exp.clone().is_infinite(){
			panic!("результат возведения в степень e^-x не может быть бесконечностью");
		} 
		if exp.clone().is_nan(){
			panic!("результат возведения в степень e^-x не может быть NaN");
		}
		result_ = 1.0/(1.0+exp);
		println!("пришло на нейрон: {:?}\nresult внутри сети: {}", inputs, result_);
		// 	  1
		//-----------
		//       (-x)
		//	1 + e
		//self.result = result_.clone();
		result_
	}
	//pub fn get_result(&self)->f32 { self.result.clone() }	
}
pub struct LayerNet {
	layer: Vec<Neyron>, // нейроны в слою // 						   [y]
}
impl LayerNet {
	pub fn first_weight_len(&self)->usize{
		self.layer[0].weight_count()
	}
	pub fn new_output_layer(inputs_count: usize)->LayerNet{
		let mut layer_: Vec<Neyron> = Vec::new();
		layer_.push(Neyron::new(inputs_count.clone(), 0.001));
		//pub fn new(weight: usize, inputs: usize, learn_speed: f32)->Neyron		
		LayerNet { layer: layer_ }
	}
	pub fn new(create_count: usize, input_count: usize)->LayerNet{
		let mut layer_: Vec<Neyron> = Vec::new();
		//println!("count: {}", count);
		for i in 0..create_count {
			println!("i__: {}", i);
			for _ in 0..input_count.clone() {
				layer_.push(Neyron::new(input_count.clone(), 0.001));
				//pub fn new(weight: usize, inputs: usize, learn_speed: f32)->Neyron
			}
		}
		LayerNet { layer: layer_ }
	}
	pub fn new_inputs_layer(count: usize)->LayerNet{
		let mut layer_: Vec<Neyron> = Vec::new();
		for i in 0..count.clone() {
			layer_.push(Neyron::new_input_neyron(count.clone()));
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
	pub fn answer(&self, input: Vec<f32>)->Vec<f32>{
		println!("пришло: {:?}", input);
		let len_ = self.layer.len();
		let mut output: Vec<f32> = Vec::new();
		for y in 0..len_ {
			//let result: f32 = self.layer[y].answer(input.clone());
			output.push(self.layer[y].answer(input.clone()));
		}
		println!("ушло: {:?}", output);
		output
	}
	pub fn debug(&self) {
		for i in 0..self.layer.len(){
			for k in 0..self.layer[i].weight.len(){
				println!("weight: {}", self.layer[i].weight[k]);
			}			
		}
	}
}
pub struct BufferNet {
	layers: Vec<Vec<LayerNet>>, // 			 						   [z][x]
	//name: String,
	layers_in_layers: Vec<BufferNet>, // на каждый [z] свой 
	//on_layers_in_layers_len: Vec<usize> // глубина
}
impl BufferNet {
	pub fn get_z_len(&self)->usize {
		self.layers.len()
	}
	pub fn answer(&self, z: usize, input: Vec<f32>)->Vec<f32>{
		let len_: usize = self.layers[z].len();
		let mut input_: Vec<f32> = input;
		//let mut output: Vec<f32> = Vec::new();
		for x in 0..len_ {
			input_ = self.layers[z][x].answer(input_.clone());
		}
		// если это конечная точка - то вернётся вектор с одним элементом #
		input_
	}	
	/*pub fn add_to_depth(&mut self, mut depth_map: Vec<usize>){
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
	}*/
	pub fn new(x: usize, y: usize, y_count: usize)->BufferNet{
		//LayerNet::new(y: usize)->LayerNet
		let mut lyrs: Vec<LayerNet> = Vec::new();
		let mut lrss: Vec<Vec<LayerNet>> = Vec::new();
		for i in 0..x.clone(){
			lyrs.push(LayerNet::new(y, y_count));
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
	pub fn new_input(y: usize)->BufferNet{
		let mut lyrs: Vec<LayerNet> = Vec::new();
		let mut lrss: Vec<Vec<LayerNet>> = Vec::new();
		lyrs.push(LayerNet::new_inputs_layer(y));

		lrss.push(lyrs);
		BufferNet {
			layers: lrss, 
			layers_in_layers: Vec::new(),
		}
	}
	pub fn add<'a>(&mut self, arg: &'a str, value_x: Vec<usize>) {
		match arg {
			/*"lx" => {
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
			},*/ // to X
			   // просто добавляем иксы в последний z			
			"nz" => {
				self.layers.push(Vec::new());
			}, // new z
			   // пустой z
			"naz" => {
				let len_: usize = value_x.len();
				if len_ == 0 {
					panic!("попытка создать output layer с неинициализированным [y]");
				}				
				let y_count: usize = value_x[0].clone();
				// LayerNet::new_output_layer(inputs_count: usize)
				let mut tmp: Vec<LayerNet> = Vec::new();
				tmp.push(LayerNet::new_output_layer(y_count.clone()));
				self.layers.push(tmp);
				println!("naz -> {:?}", self.layers.len());
			}, // new answer z
			   // для ответа, output слой
			   // первый элемент - количество Y
			"nzx" => {
				let len_: usize = value_x.len();
				if len_ == 0 {
					panic!("попытка создать [z] с неинициализированными [y]");
				}
				let y_count: usize = value_x[0].clone();
				//let y_count: usize = value_x[1].clone();
				for i in 1..len_ {
					let mut tmp: Vec<LayerNet> = Vec::new();
					//for _i in 0..value_x[i].clone() {
						tmp.push(LayerNet::new(value_x[i].clone(), y_count.clone()));
					//	println!("_: {}", _i);
					//}
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
				
				//for i in 0..count_x {
					self.layers[indx_z].push(LayerNet::new(count_x ,count_y.clone()));
				//}
			}, // to Z add x 
			   // при этом раскладе в первом элементе [0]
			   // будет указан index Z
			"aziz"=>{
				let len_: usize = value_x.len();
				if len_ == 0 {
					panic!("попытка изменить [z] in [z] не указывая количество добавляемых нейронов [y]");
				}
				if len_ == 1 {
					panic!("попытка изменить [z] in [z] не указывая количество [x]");
				}
				if len_ == 2 {
					panic!("попытка изменить [z] in [z] не указывая количество связей");
				}
				if len_ > 3 {
					panic!("попытка изменить [z] in [z]. слишком много аргументов [in z][x][y][>3]");
				}
				let y: usize  	   = value_x[0].clone();
				let count_x: usize = value_x[1].clone();
				let count_y: usize = value_x[2].clone();
				
				/*layers_in_layers: Vec<BufferNet>,
	on_layers_index_to_go_layers_in_layers: Vec<usize>*/
				/*if indx_z >= self.layers.len() {
					panic!("попытка добавить [z] in [z] в несуществующий [z].");
				}*/
				//self.on_layers_in_layers_len[indx_z] += 1;
				//pub fn new(x: usize, y: usize)->BufferNet{
				//self.layers_in_layers.push(BufferNet::new(/*count_x.clone(), indx_z.clone(), count_y*/));
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
				//self.add_to_depth(value_x.clone());
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

pub struct LogicalSheme {
	variables_: Vec<Vec<BufferNet>>, //[variables][step] // обращение: [step].[z][x].[y] 
	// 											                        [шаг].[группа][x].[y]
	to_nav_map: Vec<ToNavMap>,
	variables_name: Vec<String>,
	//variables_val : Vec<usize>,
	words: Vec<String>,
}
pub struct ToNavMap {
	navigation_on_separate_lines: Vec<Vec<Vec<usize>>>, // навигация по отдельным линиям
	//[step][порядковый номер z предыдущий][тут текущие Z] его значение -> на какой порядковый отсылается
	output_layer_in_step_and_in_z: Vec<(usize, usize)>, // output слой в шаге и в z
	// индексы ответов в формате Vec<(usize, usize)>, 
	// где первый usize -> шаг
	// второй -> index Z в шаге
	union_layer_in_step_and_z: Vec<(usize, usize, Vec<usize>)>,
	// индексы объединений в формате Vec<(usize, usize, Vec<usize>)>,
	// где первый - это шаг
	// второй - index z для объединения
	// третий - количество входящих z
}
impl ToNavMap {	
	pub fn add_output_layer_stepNumber_zNumber(&mut self, step: usize, z_output: usize) {
		println!("в создание ответа пришло:\nstep: [{}]\nz_output: [{}]", step.clone(), z_output.clone());
		self.output_layer_in_step_and_in_z.push((step.clone(), z_output.clone()));
	}
	// return add; if != 16 -> can't add element
	pub fn add_to_step_lastZ_thisZ(&mut self, step: usize , last_z: usize, this_z: usize)->u8{
		println!("в создание шага пришло:\nstep: [{}]\nlast_z: [{}]\nthis_z: [{}]",
		step.clone(), last_z.clone(), this_z.clone());
		if self.navigation_on_separate_lines.len() > step {
			if self.navigation_on_separate_lines[step].len() > last_z {
				self.navigation_on_separate_lines[step][last_z].push(this_z);
			} else {
				if self.navigation_on_separate_lines[step].len() == last_z {
					self.navigation_on_separate_lines[step].push(Vec::new());
					self.navigation_on_separate_lines[step][last_z].push(this_z);
				} else {
					return 0;
				}
			}
		} else {
			return 1;
		} 16
	}
	// return step_id new step
	pub fn new_step(&mut self)->usize{
		self.navigation_on_separate_lines.push(Vec::new());
		self.navigation_on_separate_lines.len() - 1
	}
	pub fn new()->ToNavMap{
		ToNavMap { 
			navigation_on_separate_lines: Vec::new(),
			output_layer_in_step_and_in_z: Vec::new(),
			union_layer_in_step_and_z: Vec::new()
		}
	}
	pub fn grouping_according_to_latest_data(&mut self, x: usize, this_z: usize) -> usize{
		let mut ret_val: usize = 0;
		//let step_l: usize = self.navigation_on_separate_lines.len();
		let this_step: usize = self.navigation_on_separate_lines.len().clone() - 1;
		//let 
		let len_lastStep_lastZ: usize = self.navigation_on_separate_lines[this_step - 1].len();
		//let 
		let mut union_layer_in_step_and_z_: Vec<usize> = Vec::new();
		for i in 0..len_lastStep_lastZ {			
			let len_s: usize = self.navigation_on_separate_lines[this_step - 1][i].len(); 
			for k in 0..len_s {
				let z_: usize = self.navigation_on_separate_lines[this_step - 1][i][k];
				union_layer_in_step_and_z_.push(z_);
				while self.navigation_on_separate_lines[this_step].len() < (z_ + 1) {
					self.navigation_on_separate_lines[this_step].push(Vec::new());
				}				
				//self.navigation_on_separate_lines[this_step][z_].push(this_z);				
				ret_val += 1;
			}
		} 
		/*
			union_layer_in_step_and_z: Vec<(usize, usize, Vec<usize>)>,
			// индексы объединений в формате Vec<(usize, usize, Vec<usize>)>,
			// где первый - это шаг
			// второй - index z для объединения
			// третий - входящие z
		*/
		self.union_layer_in_step_and_z.push((this_step, this_z, union_layer_in_step_and_z_));
		ret_val
	}
	pub fn grouping_according_data_fromHere_toHere(&mut self, from_here: usize, to_here: usize, this_z: usize) -> usize{
		let mut ret_val: usize = 0;
		let this_step: usize = self.navigation_on_separate_lines.len().clone() - 1;
		while self.navigation_on_separate_lines[this_step].len() < (to_here + 1) {
			self.navigation_on_separate_lines[this_step].push(Vec::new());
		}
		let mut union_layer_in_step_and_z_: Vec<usize> = Vec::new();
		let len_lastStep_lastZ: usize = self.navigation_on_separate_lines[this_step - 1].len();
		for i in 0..len_lastStep_lastZ {
			let len_s: usize = self.navigation_on_separate_lines[this_step - 1][i].len(); 
			for k in 0..len_s {
				let z_: usize = self.navigation_on_separate_lines[this_step - 1][i][k];				
				if z_ >= self.navigation_on_separate_lines[this_step].len() {
					while self.navigation_on_separate_lines[this_step].len() < (z_ + 1) {
						self.navigation_on_separate_lines[this_step].push(Vec::new());
					}
				}				
				if z_ >= from_here && z_ <= to_here {
					//println!("to_here: {}", to_here);					
					//self.navigation_on_separate_lines[this_step][z_].push(this_z);
					union_layer_in_step_and_z_.push(z_.clone());
					ret_val += 1;
					//println!("self.navigation_on_separate_lines[this_step]: {:?}", self.navigation_on_separate_lines[this_step].clone());
				} else if z_ > to_here {
					/*
						union_layer_in_step_and_z: Vec<(usize, usize, Vec<usize>)>,
						// индексы объединений в формате Vec<(usize, usize, Vec<usize>)>,
						// где первый - это шаг
						// второй - index z для объединения
						// третий - количество входящих z
					*/
					self.union_layer_in_step_and_z.push((this_step, this_z, union_layer_in_step_and_z_));
					return ret_val;
				}
			}
		}
		/*
			union_layer_in_step_and_z: Vec<(usize, usize, Vec<usize>)>,
			// индексы объединений в формате Vec<(usize, usize, Vec<usize>)>,
			// где первый - это шаг
			// второй - index z для объединения
			// третий - количество входящих z
		*/
		self.union_layer_in_step_and_z.push((this_step, this_z, union_layer_in_step_and_z_));
		ret_val
	}
	pub fn grouping_according_data_fromHere_toEnd(&mut self, from_here: usize, this_z: usize) -> usize{
		let mut ret_val: usize = 0;
		let this_step: usize = self.navigation_on_separate_lines.len().clone() - 1;
		//let 
		let len_lastStep_lastZ: usize = self.navigation_on_separate_lines[this_step - 1].len();
		//let 
		let mut union_layer_in_step_and_z_: Vec<usize> = Vec::new();
		for i in from_here..len_lastStep_lastZ {
			let len_s: usize = self.navigation_on_separate_lines[this_step - 1][i].len(); 			
			for k in 0..len_s {
				let z_: usize = self.navigation_on_separate_lines[this_step - 1][i][k];
				union_layer_in_step_and_z_.push(z_);
				if z_ >= self.navigation_on_separate_lines[this_step].len() {
					while self.navigation_on_separate_lines[this_step].len() < (z_ + 1) {
						self.navigation_on_separate_lines[this_step].push(Vec::new());
					}
				}				
				//self.navigation_on_separate_lines[this_step][z_].push(this_z);
				ret_val += 1;
			}
		} 
		/*
			union_layer_in_step_and_z: Vec<(usize, usize, Vec<usize>)>,
			// индексы объединений в формате Vec<(usize, usize, Vec<usize>)>,
			// где первый - это шаг
			// второй - index z для объединения
			// третий - количество входящих z
		*/
		self.union_layer_in_step_and_z.push((this_step, this_z, union_layer_in_step_and_z_));
		ret_val
	}
}
impl LogicalSheme {
	pub fn answer<F>(&self, variable: usize, inputs: Vec<f32>, answer_function: F) -> Vec<f32> 
		where F: Fn(f32) -> f32 {	// input -> output
		/*
			impl BufferNet {
				pub fn answer(&self, z: usize, input: Vec<f32>)->Vec<f32>{
				pub fn get_z_len(&self)->usize {
		*/
		/*
			pub struct LogicalSheme { // self
			variables_: Vec<Vec<BufferNet>>, //[variables][step] // обращение: [step].[z][x].[y] 
			// 											                        [шаг].[группа][x].[y]
			to_nav_map: Vec<ToNavMap>,//[variables]
			variables_name: Vec<String>,
			//variables_val : Vec<usize>,
			words: Vec<String>,
		}
		pub struct ToNavMap {
			navigation_on_separate_lines: Vec<Vec<Vec<usize>>>, // навигация по отдельным линиям
			//[step][порядковый номер z предыдущий][тут текущие Z] его значение -> на какой порядковый отсылается
			output_layer_in_step_and_in_z: Vec<(usize, usize)>, // output слой в шаге и в z
			// индексы ответов в формате Vec<(usize, usize)>, 
			// где первый usize -> шаг
			// второй -> index Z в шаге
		}
		*/
		let steps: usize = self.variables_[variable].len();
		let y_count: usize = inputs.clone().len();
		let mut for_return: Vec<f32> = Vec::new();
		let mut input_: Vec<f32> = inputs; // массив входом
		let mut output_: Vec<Vec<f32>> = Vec::new(); // массив ответов для каждого Z, где [z][answer]
		let answers: Vec<(usize, usize)> = self.to_nav_map[variable].output_layer_in_step_and_in_z.clone();		
		let group_on_step_z_count: Vec<(usize, usize, Vec<usize>)> = self.to_nav_map[variable].union_layer_in_step_and_z.clone();
		let mut ans_indx: usize = 0;
		let mut group_index: usize = 0;
		for step in 0..steps {	
			// буду переделывать, почти всё готово
			println!("step: [{}]", step);
			let mut temp_input: Vec<Vec<f32>> = output_.clone();
			output_ = Vec::new();	
			
			let map_zets_last_step: usize = self.to_nav_map[variable]
				.navigation_on_separate_lines[step].len();
			//println!("косяк?");
			if ans_indx < answers.len() && step == answers[ans_indx].0.clone() {
				// ответы сети
				for_return.push(answer_function(
					self.variables_[variable][step - 1].answer(0, 
						temp_input[answers[ans_indx].clone().1].clone())[0]
				).clone());
				ans_indx += 1;
			}
			//println!("да не");
			if group_index < group_on_step_z_count.len() && 
						step == group_on_step_z_count[group_index].0.clone() {
				//
				let mut input_s: Vec<f32> = Vec::new();
				println!("group_on_step_z_count[group_index].2.clone():  {:?}", group_on_step_z_count[group_index].2.clone());
				for z_to_group in group_on_step_z_count[group_index].2.clone() {
					for i in 0..temp_input[z_to_group].len() {
						input_s.push(temp_input[z_to_group][i]);
					}
				}
				println!("input_for_group: {:?}", input_s);
				output_.push(self.variables_[variable][step].answer(group_on_step_z_count[group_index].1.clone(), input_s));
			}
			
			if step == 0 {		
				// первый раз, заносим в инпуты		
				for i in 0..map_zets_last_step {
					let this_zs: usize = self.to_nav_map[variable]
						.navigation_on_separate_lines[step][i].len();
					for k in 0..this_zs {
						output_.push(self.variables_[variable][step].answer(k, input_.clone()));						
					}
				} 
				println!("output: {:?}", output_);
				continue;
			}
			for index_last_z in 0..map_zets_last_step {
				for index_this_z in 0..self.to_nav_map[variable].navigation_on_separate_lines[step][index_last_z].len() {
					output_.push(self.variables_[variable][step].answer(index_this_z, temp_input[index_last_z].clone()));
				}
			} 
			println!("output: {:?}", output_);
		} 
		// даём ответ
		//input_ = Vec::new();
		//for index_answer in 0..self.variables_[variable][steps - 1].get_z_len() {

		//}
		for_return
	}
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
									self.variables_[i][st].layers[k][m].debug();
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
						println!("output_layer_in_step_and_in_z [(step, z)]: {:?}", self.to_nav_map[i].output_layer_in_step_and_in_z);
						for k in 0..len__s { // по шагам
							let to____z_: usize = self.to_nav_map[i].navigation_on_separate_lines[k].len();
							for l in 0..to____z_.clone(){ // по предыдущим z
								//for i_ in 0..self.to_nav_map[i].navigation_on_separate_lines[l].len() {
									for k_ in 0..self.to_nav_map[i].navigation_on_separate_lines[k][l].len() {
										println!("[step]: [last z] [this z]\n[{}]: [ {} ] [ {} ]", k.clone(), 
										l.clone(),
										self.to_nav_map[i].navigation_on_separate_lines[k][l][k_].clone());		
									}
								//}								
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
		let mut y_count: usize = 0;	         	  			  	  // тут число входов
		let mut temp_name_variable: String = "".to_string(); 	  // тут временное имя переменной
		let mut variable_index: usize = 0;			  			  // индекс переменной
		let mut union_layer: bool = false;
		let mut output_layer:bool = false;
		//let mut last_op: [u8; 3] = [0; 3];
		
		let mut value_in_z: Vec<usize> = Vec::new(); // а тут все значения для Z (иксы)
		
		let mut step_value: usize = 0;					  // навигация по шагам
		//let mut to_z_navigation: Vec<usize> = Vec::new(); // навигация по z
		let mut index_last_z: usize = 0;
		let mut this_z_array: Vec<usize> = Vec::new();
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
			println!("index_last_z: {}\nthis_z_array: {:?}\ny_count: {}\nnext_tire: {}", 
			index_last_z.clone(), this_z_array.clone(), y_count.clone(), next_tire.clone());
			println!("buffer_text: {}\nvalue_in_z: {:?}", buffer_text.clone(), value_in_z.clone());
			println!("---------------------------------------");
			match ch {	
				'D' => { 
					self.debug();
				},
				'<' => {
					// это за ответ отвечает
					// не забудь в картах шагов добавить индексы ответов в формате Vec<(usize, usize)>, 
					// где первый usize -> шаг
					// второй -> index Z в шаге
					output_layer = true;
				},
				'e' => { return; },
				'\''=> { comment = true; },	
				';' => { 
					y_count = match buffer_text.trim().parse::<usize>() {
						Ok(A) => { A },
						Err(e)=> { panic!("значение количества входов должно быть числовым."); 0 },
					};
					buffer_text = "".to_string();
				},
				'^' => {					
					union_layer = true;
				},
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
							//self.variables_[variable_index].push(BufferNet::new_input(y_count.clone()));
							self.variables_[variable_index].push(BufferNet::new_empty());
							//self.to_nav_map[variable_index] = ToNavMap::new();
							//println!("input value_in_z: {:?}", value_in_z.clone());							
							if step_value != self.to_nav_map[variable_index].new_step() {
								panic!("проверьте ваш пк, похоже кто-то пытается ломануть!");
							}
							value_in_z = Vec::new();
							next_tire = false;
							buffer_text = "".to_string();
							continue;
						}
						if union_layer {
							let fromHere_toHere: Vec<&str> = buffer_text.split('-').collect();
							let mut value__: usize = 0;
							if fromHere_toHere.len() == 1 {
								let value_: usize = match fromHere_toHere[0].to_string().trim().parse::<usize>() {
									Ok(A)=>{ A },
									Err(e)=>{ panic!("не получилось прочитать число групп запятой."); 0 },
								};
								value__ = self.to_nav_map[variable_index]
									.grouping_according_to_latest_data(value_.clone(), index_this_z.clone());

								//value_in_z.push(value_.clone());
							} else {								
								let from_here: usize = match fromHere_toHere[0].to_string().trim().parse::<usize>() {
									Ok(A)=>{ A },
									Err(e)=>{ panic!("не получилось прочитать число групп ОТ запятой."); 0 },
								};
								if fromHere_toHere[1] != "_" {
									let to_here: usize = match fromHere_toHere[1].to_string().trim().parse::<usize>() {
										Ok(A)=>{ A },
										Err(e)=>{ panic!("не получилось прочитать число групп ДО запятой."); 0 },
									};
									value__ = self.to_nav_map[variable_index]
										.grouping_according_data_fromHere_toHere(from_here, to_here, index_this_z);
								} else {
									value__ = self.to_nav_map[variable_index]
										.grouping_according_data_fromHere_toEnd(from_here, index_this_z);
								}
								//value_in_z.push(1);
							}
							union_layer = false;
							let len_: usize = self.variables_[variable_index].len();
							value_in_z.insert(0, y_count.clone());
							println!("step value_in_z([y][x][x]): {:?}", value_in_z.clone());
							self.variables_[variable_index][len_ - 1].add("nzx", value_in_z.clone());
							
							self.variables_[variable_index].push(BufferNet::new_empty());
							//BufferNet::new_empty
							// тут не просто обнуление!
							
							//to_z_navigation = Vec::new();
							for this_i in this_z_array {
								self.to_nav_map[variable_index]
									.add_to_step_lastZ_thisZ(
										step_value.clone(), 
										index_last_z.clone(),
										this_i.clone()
									);
							}				
							step_value += 1;
							if step_value != self.to_nav_map[variable_index].new_step() {
								panic!("ошибка в карте и шагах!");
							}
						} else if output_layer {
							let output: usize = match buffer_text.trim().parse::<usize>() {
								Ok(A)=>{ A },
								Err(e)=>{ panic!("не получилось прочитать индекс [z] выходного слоя."); 0 },
							};

							let len_: usize = self.variables_[variable_index].len();
							let temp_: Vec<usize> = vec![y_count.clone()];
							self.variables_[variable_index][len_ - 1].add("naz", temp_);
							self.to_nav_map[variable_index].add_output_layer_stepNumber_zNumber(step_value.clone(), output);
							// add_output_layer_stepNumber_zNumber

							self.variables_[variable_index].push(BufferNet::new_empty());
							output_layer = false;
							
							step_value += 1;
							if step_value != self.to_nav_map[variable_index].new_step() {
								panic!("ошибка в карте и шагах!");
							}
						} else {
							// return add; if != 16 -> can't add element
							//pub add_to_step_lastZ_thisZ(&mut self, step: usize , last_z: usize, this_z: usize)->u8{

							// return step_id new step
							//pub fn new_step(&mut self)->usize{
							//for last_i in 0..index_last_z {						
							if index_this_z != 0 {
								let value_: usize = match buffer_text.trim().parse::<usize>() {
									Ok(A)=>{ A },
									Err(e)=>{ panic!("не получилось прочитать число около запятой."); 0 },
								};
								buffer_text = "".to_string();	
								this_z_array.push(index_this_z.clone());
								value_in_z.push(value_.clone());
							}
							
							for this_i in this_z_array {
								self.to_nav_map[variable_index]
									.add_to_step_lastZ_thisZ(
										step_value.clone(), 
										index_last_z.clone(),
										this_i.clone()
									);
							}
							//}
							//let mut index_last_z: usize = 0;
							//let mut this_z_array: usize = 0;

							
							let len_: usize = self.variables_[variable_index].len();
							value_in_z.insert(0, y_count.clone());
							println!("step value_in_z([y][x][x]): {:?}", value_in_z.clone());
							self.variables_[variable_index][len_ - 1].add("nzx", value_in_z.clone());
							
							self.variables_[variable_index].push(BufferNet::new_empty());
							//BufferNet::new_empty
							// тут не просто обнуление!
							
							//to_z_navigation = Vec::new();
													
							step_value += 1;
							if step_value != self.to_nav_map[variable_index].new_step() {
								panic!("ошибка в карте и шагах!");
							}	
						}						
						next_tire = false;
						index_last_z = 0;
						index_this_z = 0;
						this_z_array = Vec::new();
						value_in_z = Vec::new();
						buffer_text = "".to_string();
					}
				},				
				',' => {
					//let len_: usize = to_z_navigation.len();
					//group = true;
					// •••
					if output_layer {
						buffer_text.push(',');
						continue;
					}
					let value_: usize = match buffer_text.trim().parse::<usize>() {
						Ok(A)=>{ A },
						Err(e)=>{ panic!("не получилось прочитать число около запятой."); 0 },
					};
					buffer_text = "".to_string();
					this_z_array.push(index_this_z.clone());				
					//this_z_array += 1;			
					index_this_z += 1;				
					value_in_z.push(value_.clone());					
				},
				'|' => { 
					let len_: usize = self.variables_[variable_index].len();
					if union_layer {
						let fromHere_toHere: Vec<&str> = buffer_text.split('-').collect();
						if fromHere_toHere.len() == 1 {
							let value_: usize = match fromHere_toHere[0].to_string().trim().parse::<usize>() {
								Ok(A)=>{ A },
								Err(e)=>{ panic!("не получилось прочитать число групп запятой."); 0 },
							};
							println!("value: {}",value_);
							self.to_nav_map[variable_index]
								.grouping_according_to_latest_data(value_.clone(), index_this_z.clone());

							//value_in_z.push(value_.clone());
						} else {
							let from_here: usize = match fromHere_toHere[0].to_string().trim().parse::<usize>() {
								Ok(A)=>{ A },
								Err(e)=>{ panic!("не получилось прочитать число групп ОТ запятой."); 0 },
							};
							if fromHere_toHere[1] != "_" {
								let to_here: usize = match fromHere_toHere[1].to_string().trim().parse::<usize>() {
									Ok(A)=>{ A },
									Err(e)=>{ panic!("не получилось прочитать число групп ДО запятой."); 0 },
								};
								self.to_nav_map[variable_index]
									.grouping_according_data_fromHere_toHere(from_here, to_here, index_this_z);
							} else {
								self.to_nav_map[variable_index]
									.grouping_according_data_fromHere_toEnd(from_here, index_this_z);
							}
							// value_in_z.push(1); написать им новый 'nay' (New Answer Layer) в
							// self.variables_[variable_index][len_ - 1].add("nay", вектор);
							// в векторе первым - колво Y, второе - колво объединяемых Z
							// по формуле Y * Z (то есть колво нейронов на количество групп, будет всего колво связей)
							// сделать только один слой дефакто, потом может добавлю для других
						}
						union_layer = false;
					} else {
						let value_: usize = match buffer_text.trim().parse::<usize>() {
							Ok(A)=>{ A },
							Err(e)=>{ panic!("не получилось прочитать число около запятой."); 0 },
						};
						this_z_array.push(index_this_z.clone());
						value_in_z.push(value_.clone());
						//value_in_z.insert(0, y_count.clone());
						//self.variables_[variable_index][len_ - 1].add("nzx", value_in_z.clone());
						// step_value: usize 		// навигация по шагам
						// to_z_navigation:       ; // навигация по z
						//this_z_array += 1;
						//self.to_nav_map[variable_index].grouping_by_this_data(from_here, to_here);
						for this_i in this_z_array {
							self.to_nav_map[variable_index]
								.add_to_step_lastZ_thisZ(
									step_value.clone(), 
									index_last_z.clone(),
									this_i.clone()
								);
						}
						/*
							pub fn add_and_new_step(&mut self, value: usize){
							pub fn add_to_step(&mut self, step: usize, value: usize){
						*/
						//to_nav_map: Vec<ToNavMap>,					
					}
					// запись в Map
					buffer_text = "".to_string();	
					this_z_array = Vec::new();
					index_last_z += 1;					
					index_this_z += 1;
					//this_z_array = 0;
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
					if next_tire {
						next_tire = false;
						buffer_text.push('-');
					}				
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

fn answer_function(answer: f32)->f32{
	println!("answer: {}", answer);
	answer
}

fn main() {
    println!("Hello, world!");
	let mut t = LogicalSheme::new();
	t.parser("
	' comment
	main:  2 -> 1, 1 -> ^1 -> <0> -> De out");
	let v: Vec<f32> = vec![0.0, 1.0];
	t.answer(0, v, answer_function);
	/*
		pub fn answer<F>(&self, variable: usize, inputs: Vec<f32>, answer_function: F) -> Vec<f32> 
		where F: Fn(f32) -> f32 {	// input -> output
	*/
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

