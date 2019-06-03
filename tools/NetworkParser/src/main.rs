
extern crate rand; // 0.6.5
use rand::Rng;

pub struct Neyron{
    weight: Vec<f32>,
    //inputs: Vec<f32>,
    learn_speed: f32,
 
    result: f32,
 
}
impl Neyron {
	pub fn weight_count(&self)->usize{
		self.weight.len()
	}
	pub fn get_weight(&self)->Vec<f32>{
		self.weight.clone()
	}
	pub fn set_weight(&mut self, weights: Vec<f32>){
		self.weight = weights.clone();
	}
	pub fn result(&self)->f32{
		self.result.clone()
	}
	pub fn new(weight: usize, learn_speed_: f32, create_new_weight: &'static Fn(usize, f32) -> (Vec<f32>, f32)) -> Neyron
	/*where F: Fn(usize, f32) -> (Vec<f32>, f32) */{
		//println!("weight_count: {}", weight);
		let (weight_, learn_speed) = create_new_weight(weight.clone(), learn_speed_.clone());
		//let mut weight_: Vec<f32> = Vec::new();
		
		/*for i in 0..weight{
		    let mut rng = rand::thread_rng();
		    //if rng.gen() { // random bool
		    let mut f: f32 = *//*rng.gen::<f32>();*//*0.1;
			let h: f32 = rng.gen::<f32>();*//*
			if h > 0.6 {
				f *= -1.0;
			}
		    if f < 0.0 {
				f *= 1.0;
				while f < -0.3 {
					f /= 10.0;
				}
			} else if f > 0.0 {
				while f > 0.3 {
					f /= 10.0;
				}
		    } else {
				f = -0.5;
		    }*/ //let f: f32 = 0.1;*/
		    /*weight_.push(f);
		}*/
		
		/*let mut inputs_: Vec<f32> = Vec::new();
		for i in 0..inputs {
			inputs_.push(0.0);
		}*/
		//let learn_speed: f32 = 0.001;
		Neyron { 
			weight: weight_, 
			//inputs: inputs_, 
			learn_speed: learn_speed, 
			result: 0.0 
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
			result: 0.0 
		}
	}
	pub fn clone(&self)->Neyron{ 
		Neyron { 
			weight: self.weight.clone(), 
			//inputs: self.inputs.clone(), 
			learn_speed: self.learn_speed.clone(), 
			result: self.result.clone() 
		} 
	}
	pub fn perc_answer(&mut self, input: f32) -> f32 {
		let mut result_: f32 = 0.0;
		for i in 0..1 {
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
			//println!("{} += {} * input [{}]", result_, self.weight[i].clone(), input);
			result_ += self.weight[i].clone() * input;
		} 
		result_ = self.sigmoid(result_.clone());
		self.result = result_.clone();
		result_
	}
	// fn() and get_set result
	pub fn answer(&mut self, inputs: Vec<f32>)->f32 {
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
		let mut exp: f32 = (result_.clone() * -1.0).exp();
		while exp.is_infinite(){
			result_ = 0.0;
			panic!("y = f(x) не может быть равен бесконечности");
			return 1.0;
		  	println!("----------------------------------------------------------------");
		  	//println!("веса: {:?}", self.weight);
			//let mut max: usize = 0;
			let mut rng = rand::thread_rng();
			for i in 0..self.weight.len() {				
				//if rng.gen() { // random bool
				
				let mut f: f32 = /*rng.gen::<f32>();*/ 0.1;
				/*let h: f32 = rng.gen::<f32>();
				if h > 0.6 {
					f *= -1.0;
				}
				if f < 0.0 {
					f = -0.5;
				} else if f > 0.0 {
					f = 0.5;
				} else {
					f = -0.5;
				}*/
				/*if self.weight[i].is_infinite() || self.weight[i].is_nan(){
					if f > 0.0 {
						self.weight[i] = -0.5;
					} else {
						self.weight[i] = 0.5;
					}
				}*/
				self.weight[i] = f.clone();
			}
			for i in 0..inputs.len() {			
				result_ += self.weight[i].clone() * inputs[i];
			} 
			exp = (result_.clone() * -1.0).exp();
			//panic!("результат возведения в степень e^-x не может быть бесконечностью");
		} 
		while exp.is_nan(){
			result_ = 0.0;
			panic!("y = f(x) не может быть равен 1/бесконечность");
			return 0.0;
		  	println!("----------------------------------------------------------------");
		  	//println!("веса: {:?}", self.weight);
			//panic!("результат возведения в степень e^-x не может быть NaN");
			//let mut min: usize = 0;
			let mut rng = rand::thread_rng();
			for i in 0..self.weight.len() {				
				//if rng.gen() { // random bool
				let mut f: f32 = /*rng.gen::<f32>();*/ 0.1;
				/*let h: f32 = rng.gen::<f32>();
				if h > 0.6 {
					f *= -1.0;
				}
				if f < 0.0 {
					f = -0.5;
				} else if f > 0.0 {
					f = 0.5;
				} else {
					f = -0.5;
				}*/
				/*if f < 0.0 {
					while f < -1.0 {
						f /= 10.0;
					}
				} else if f > 0.0 {
					while f > 1.0 {
						f /= 10.0;
					}
				} else {
					f = -0.5;
				}*/
				self.weight[i] = f.clone();
			}			
			for i in 0..inputs.len() {			
				result_ += self.weight[i].clone() * inputs[i];
			} 
			exp = (result_.clone() * -1.0).exp();
		}
		result_ = self.sigmoid(result_.clone());
		//println!("веса: {:?}", self.weight);
		//println!("пришло на нейрон: {:?}\nresult внутри сети: {}", inputs, result_);
		
		// 	  1
		//-----------
		//       (-x)
		//	1 + e
		self.result = result_.clone();
		result_
	}
	//pub fn get_result(&self)->f32 { self.result.clone() }	
	pub fn sigmoid(&self, x: f32)->f32{
		//println!(" результат сигмоиды: {:?}", (1.0/(1.0 + (x.clone() * -1.0).exp())));
		(				1.0/
			(1.0 + (x.clone() * -1.0).exp())
		)
	}
	pub fn umnoz(&self, indx: usize, out: Vec<f32>) -> f32 {
		let mut summ: f32 = 0.0;
		for out_ in out {
			summ += self.weight[indx].clone() * out_;
		} summ
	}
	pub fn delta(&self, result: f32, true_result: f32) -> f32 {
		(
			result * (1.0 - result.clone()) * (true_result.clone() - result.clone())
		)
	}
	pub fn hidden_delta(&self, indx: usize, true_result: f32, out_: f32) -> f32{
		0.0
	}
	pub fn error_perc(&mut self, fact_result: f32, true_result: f32, input: f32, hidden_errors: f32) {
		for i in 0..1 {
			print!("\tвес[{}] был: [{}]", i.clone(), self.weight[i].clone());
			self.weight[i] += self.learn_speed * ((hidden_errors.clone() * self.result * (1.0 - self.result)) * input);
			println!(" | стал: [{}]", self.weight[i].clone());
		}
	}
	pub fn error(&mut self, fact_result: f32, true_result: f32, inputs: Vec<f32>, hidden_errors: Vec<f32>) {
		
		for i in 0..self.weight.clone().len() {
			//print!("\tвес[{}] был: [{}]", i.clone(), self.weight[i].clone());
			self.weight[i] += self.learn_speed * (((hidden_errors[i].clone() * self.result * (1.0 - self.result)) * inputs[i])/self.weight[i]);
			//println!(" | стал: [{}]", self.weight[i].clone());
		}		
		return;
		// засунуть в for_start
		//let d: f32 = self.result.clone() * (1.0 - self.result.clone())*(true_result.clone() - self.result.clone());
		//let new_out_layer_weigth: f32 = old_out_layer_weigth + (self.learn_speed * d.clone() * self.result.clone());
		// для скрытых слоёв
		//let d: f32 = self.result.clone() * (1.0 - self.result.clone());

		//let new_hidden_layer: f32 = old_hidden_layer + 
		//	self.learn_speed * (self.hidden_delta(индекс_веса_который_меняем, true_result.clone()) * вес_связи_с_этим_слоем) * self.result.clone();
		//
		//println!("-------------------\nerror in neyron: {}", error);
		// where F: Fn(f32) -> f32 
		// A = L(E/x)
		// где 
		// A - значение на которое меняем веса
		// L - скорость обучение (для более точного ответа сети)
		// E - ошибка; E = верный ответ - ответ сети
		// х - вес связи
		
		/*let mut summ: f32 = 0.0;
		for k in 0..self.weight.len().clone() {
			summ += self.weight[k].clone();
		}		
		println!("сумма весов: [{}]", summ);

		for i in 0..self.weight.len().clone() {
			//pub fn sigmoid(&self, x: f32)->f32{
			//pub fn umnoz(&self, indx: usize, out: Vec<f32>) -> f32 {
			
			let weight_: f32 = self.weight[i].clone();

			println!("вес: [{}]", weight_);	*/
			
			//let error = (weight_ / summ) * error.clone();			
			/*if true_result.clone() > 0.0 && self.sigmoid(self.umnoz(i.clone(), output_prev_layer.clone())) > 0.0 {
				j_ *= -1.0;
			} else {
				//
			}*/
			//self.weight[i] = self.learn_speed.clone() * (error.clone() / weight_.clone());
			/*println!(" изменяем вес на: {:?}", (((-1.0 * error.clone()) * self.sigmoid(self.umnoz(i.clone(), output_prev_layer.clone())) * 
			(1.0 - self.sigmoid(self.umnoz(i.clone(), output_prev_layer.clone()))) * output_prev_layer[i]) * self.learn_speed));

			self.weight[i] += ((-1.0 * error.clone()) * self.sigmoid(self.umnoz(i.clone(), output_prev_layer.clone())) * 
			(1.0 - self.sigmoid(self.umnoz(i.clone(), output_prev_layer.clone()))) * output_prev_layer[i]) * self.learn_speed;

			println!("вес: [{}]", self.weight[i]);
			*//*if self.weight[i] < 0.00000000000001 {
				self.weight[i] = -0.5;
			} else if self.weight[i] > -0.00000000000001 {
				self.weight[i] = 0.5;
			}*/
			/*let weight_: f32 = self.weight[i].clone();
			for_return.push((weight_ / summ) * error.clone());
			println!("ошибка на вес[{}]: [{}]", i, ((weight_ / summ) * error.clone()));
		}*/
		//Ok(for_return)
	}
}
pub struct LayerNet {
	layer: Vec<Neyron>, // нейроны в слою // 						   [y]
}
impl LayerNet {
	pub fn get_output(&self) -> Vec<f32> {
		let mut output_: Vec<f32> = Vec::new();
		for i in 0..self.layer.len() {
			output_.push(self.layer[i].result.clone());
		} output_
	}	
	pub fn error_perc(&mut self, fact_result: f32, last_out: Vec<f32>, 
						true_result: f32, inputs: Vec<f32>, hidden_errors: Vec<f32>) {
		println!("error_perc in LayerNet");
		for i in 0..self.layer.len(){			
			self.layer[i].error_perc(fact_result.clone(), true_result.clone(), inputs[i], hidden_errors[i].clone());			
		}
	}
	pub fn error(&mut self, fact_result: f32, true_result: f32, inputs: Vec<f32>, hidden_errors: Vec<f32>)/*->Result<Vec<f32>, ()>*/{		
		//println!("error in LayerNet");
		//let mut for_return: Vec<f32> = Vec::new();
		//let mut output_: Vec<f32> = prev_layer.get_output();
		//println!("error.len(): {}", hidden_errors.len());	
		//println!("self.layer.len(): {}", self.layer.len());

		for i in 0..self.layer.len(){			
			/*let for_temp_value: Vec<f32> = match */self.layer[i].error(fact_result.clone(), true_result.clone(), inputs.clone(), hidden_errors.clone());/* {
				Ok(A) => { A },
				Err(e) =>{ return Err(()); Vec::new() },
			};
			println!("for_temp_value.len(): {}", for_temp_value.len());
			for k in 0..for_temp_value.len() {
				if k < for_return.len() {
					for_return[k] += for_temp_value[k].clone();
				} else {
					for_return.push(for_temp_value[k].clone());
				}
			}*/
		} //Ok(for_return)
	}
	pub fn first_weight_len(&self)->usize{
		self.layer[0].weight_count()
	}
	pub fn to_output(weight_count: usize, learn_speed: f32)->(Vec<f32>, f32) {
		(Vec::new(), 0.0)
	}
	pub fn new_output_layer(inputs_count: usize, new_fn: &'static Fn(usize, f32) -> (Vec<f32>, f32))->LayerNet {
		let mut layer_: Vec<Neyron> = Vec::new();
		layer_.push(Neyron::new(inputs_count.clone(), 0.1, new_fn));
		//pub fn new(weight: usize, inputs: usize, learn_speed: f32)->Neyron		
		LayerNet { layer: layer_ }
	}
	pub fn new(input_count: usize, fn_to_create: &'static Fn(usize, f32) -> (Vec<f32>, f32))->LayerNet {
		let mut layer_: Vec<Neyron> = Vec::new();
		//println!("count: {}", count);
		//for i in 0..create_count {
		//println!("i__: {}", i);
		for _ in 0..input_count.clone() {
			layer_.push(Neyron::new(input_count.clone(), 0.1, fn_to_create));
			//pub fn new(weight: usize, inputs: usize, learn_speed: f32)->Neyron
		}
		//}
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
	pub fn perc_answer(&mut self, input: f32, y: usize) -> f32 {
		println!("пришло: {:?}", input);
		println!("self.layer.len(): {}", self.layer.len());
		let len_ = self.layer.len();
		let mut output: Vec<f32> = Vec::new();
		//for y in 0..len_ {
			//let result: f32 = self.layer[y].answer(input.clone());
		//	if y == 0 {
		if y >= self.layer.len() { return std::f32::NAN; }
		self.layer[y].perc_answer(input.clone())
		//	}
		//}
		//println!("ушло: {:?}", output);
		//output
	}
	pub fn answer(&mut self, input: Vec<f32>, y: usize)->f32{
		//println!("пришло: {:?}", input);
		//println!("self.layer.len(): {}", self.layer.len());
		let len_ = self.layer.len();
		let mut output: Vec<f32> = Vec::new();
		//for y in 0..len_ {
			//let result: f32 = self.layer[y].answer(input.clone());
		//	if y == 0 {
		if y >= self.layer.len() { return std::f32::NAN; }
		self.layer[y].answer(input.clone())
		//	}
		//}
		//println!("ушло: {:?}", output);
		//output
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
	pub fn clone(&self) -> BufferNet {
		let mut lyrs: Vec<Vec<LayerNet>> = Vec::new();
		//pub fn new(x: usize, y: usize, y_count: usize)->BufferNet{
		let layers_in_layers: Vec<BufferNet> = Vec::new();
		for i in 0..self.layers.len() {
			let mut lyr: Vec<LayerNet> = Vec::new();
			for k in 0..self.layers[i].len(){
				lyr.push(self.layers[i][k].clone());
			}
			lyrs.push(lyr);
		}
		BufferNet {
			layers: lyrs, // 			 						   [z][x]
			//name: String,
			layers_in_layers: layers_in_layers, // на каждый [z] свой 
			//on_layers_in_layers_len: Vec<usize> // глубина
		}
	}
	pub fn get_z_len(&self)->usize {
		self.layers.len()
	}
	pub fn insert_(&mut self, z: usize, x: usize, y: LayerNet){
		if z < self.layers.len() && x < self.layers[z].len() && y.layer.len() > 0 {
			//println!("self.layers.len(): {}, self.layers[z].len(): {}, y.layer.len(): {}"
			//, self.layers.len(), self.layers[z].len(), y.layer.len());
			self.layers[z][x] = y;
		}
	}
	pub fn for_start(&mut self, z: usize, last_z_s: Vec<usize>, last_step: BufferNet, true_answer: f32, net_ans: f32) -> Vec<f32> {
		//println!("пришло на for_start: [z]: [{}]\nlast_z_s: {:?}\ntrue_aswer: [{:?}]", 
			//z.clone(), last_z_s.clone(), true_answer.clone());
		let mut neyron = self.layers[z][self.layers[z].len() - 1].clone(); // neyron
		
		let mut out: Vec<f32> = Vec::new();
		let mut errors: Vec<f32> = Vec::new();
		for z_ in last_z_s {
			//println!("last_step.layers.len() = {:?}", last_step.layers.len());
			let neyron_ = last_step.layers[z_][last_step.layers[z_].len() - 1].clone(); // neyron_
			// тут?
			for i in 0..neyron_.layer.len(){			
				out.push(neyron_.layer[i].result());
				//println!("косяк?");
				//for_return.push((weight_ / summ) * error.clone());				
			} 
		}		
		for i in 0..neyron.layer.len() { // тут по факту 1 [y] но на всякий // set_weight(weights: Vec<f32>)
			// pub fn result(&self)->f32{
			//pub fn sigmoid(&self, x: f32)->f32{
			//pub fn umnoz(&self, indx: usize, out: Vec<f32>) -> f32 {	
			let mut summ: f32 = 0.0;
			let result: f32 = neyron.layer[i].result();	
			let mut j_: f32 = 1.0;
			/*if (true_answer.clone() - net_ans.clone()) > 0.0 {
				j_ = -1.0;
			} else {
				j_ = 1.0;
			}*/
			for l in 0..neyron.layer[i].weight.len() {
				/*for k in 0..neyron.layer[i].weight.len() {
					summ += neyron.layer[i].weight[k].clone();
				} */
				//if (out[l] - net_ans.clone()) > 0.0 {
				//j_ = -1.0 * (true_answer.clone() - net_ans.clone()) * (true_answer.clone() - net_ans.clone());
				//} else {
				//	j_ = 1.0;
				//}
				//println!("отправили на вес[{}] ошибку[ {:?} ]", l.clone(), ((true_answer - net_ans) * neyron.layer[i].weight[l]));
				errors.push((true_answer - net_ans) * neyron.layer[i].weight[l]);
			}
			let weights: Vec<f32> = neyron.layer[i].clone().get_weight();
			//let result: f32 = neyron.layer[i].result();		
			for k in 0..weights.len(){
				//println!("neyron.layer[i].weight[k]: {}", neyron.layer[i].weight[k].clone());
				/*if neyron.layer[i].weight[k].clone().is_infinite() || (neyron.layer[i].weight[k].clone()*-1.0).is_infinite(){					 
					if neyron.layer[i].weight[k].clone().is_infinite() {
						neyron.layer[i].weight[k] = -0.5;
					} else if (neyron.layer[i].weight[k].clone()*-1.0).is_infinite() {
						neyron.layer[i].weight[k] = 0.5;
					} else {
						neyron.layer[i].weight[k] = 0.1;
					}
					println!("стал: {}", neyron.layer[i].weight[k].clone());
				}*/
				neyron.layer[i].weight[k] += neyron.layer[i].learn_speed * (((true_answer * net_ans) * (1.0 - net_ans)) * out[k].clone()); // запили отдельный метод, если ласт был группой
				//println!("neyron.layer[i].weight[k]: {}", neyron.layer[i].weight[k].clone());
			}
		} 
		let x = self.layers[z].len() - 1;
		self.layers[z][x] = neyron;
		errors
	}
	pub fn get_output_to_las_x(&self, z: usize) -> Vec<f32> {
		if z < self.layers.len() {
			if self.layers[z].len() == 0 {
				panic!("неинициализированный слой");
			}
			let x: usize = self.layers[z].len() - 1;
			self.layers[z][x].get_output()
		} else {
			panic!("неинициализированный слой");
			Vec::new()
		}
	}
	pub fn error_perc(&mut self, 
		z: usize, fact_result: f32, true_result: f32, inputs: Vec<f32>, hidden_errors: Vec<f32>) {
			let mut x: usize = self.layers[z].len() - 1;		
		loop {			
			if x == 0 {
				self.layers[z][x].error(fact_result.clone(), true_result.clone(), inputs.clone(), hidden_errors.clone());
				//println!("вышли из цикла");
				break;
			}
			//println!("работаем в цикле");
			self.layers[z][x].error(fact_result.clone(), true_result.clone(), inputs.clone(), hidden_errors.clone());			
			x -= 1;
		}
	}
	pub fn error(&mut self, 
		z: usize, fact_result: f32, true_result: f32, inputs: Vec<f32>, hidden_errors: Vec<f32>) {
		// лучше тащить не предыдущий layerNet, а ответы с предыдущего шага, по Z
		// предыдущий (нулевой) с предыдущего step [z][0], возвращаем ошибки, старый предыдущий, новый предыдущий
		//error(&mut self, error: Vec<f32>, true_result: f32, prev_layer: LayerNet)
		// fact_result: f32, true_result: f32, inputs: Vec<f32>, hidden_errors: Vec<f32>
		let mut x: usize = self.layers[z].len() - 1;		
		loop {			
			if x == 0 {
				//prev_layer = self.layers[z][x].clone();
				/*error = match*/ self.layers[z][x].error(fact_result.clone(), true_result.clone(), inputs.clone(), hidden_errors.clone()); /*{
					Ok(A) => { A },
					Err(e)=> { return Err(()); Vec::new() },
				};*/
				//println!("вышли из цикла");
				break;
			}
			//println!("работаем в цикле");
			/*let last_x_output: Vec<f32> = self.layers[z][x - 1].clone().get_output();
			error = match*/ self.layers[z][x].error(fact_result.clone(), true_result.clone(), inputs.clone(), hidden_errors.clone()); /*{
				Ok(A) => { A },
				Err(e)=> { return Err(()); Vec::new() },
			};*/
			//prev_layer = self.layers[z][x].clone();
			x -= 1;
		}
		//Ok(error)
	}
	pub fn z_count(&self)->usize{
		self.layers.len()
	}
	pub fn perc_answer(&mut self, z: usize, input: Vec<f32>) -> Vec<f32> {
		//println!("z: {}", z);
		if z >= self.layers.len() {
			panic!("указанное Z больше фактического последнего.");
		}
		let len_: usize = self.layers[z].len();
		let y_count: usize = input.clone().len();
		let mut input_: Vec<f32> = input;
		//let mut output: Vec<f32> = Vec::new();		
		for x in 0..len_ {
			//println!("отправили: {:?}", input_);
			let mut input_tmp: Vec<f32> = Vec::new();
			for i in 0..y_count {
				let r: f32 = self.layers[z][x].perc_answer(input_[i].clone(), i);
				if r.is_nan() {
					input_tmp.push(0.0);
					continue;
				}
				input_tmp.push(r);
			}
			input_ = input_tmp;
		}
		// если это конечная точка - то вернётся вектор с одним элементом #
		input_
	}
	pub fn answer(&mut self, z: usize, input: Vec<f32>)->Vec<f32>{
		//println!("z: {}", z);
		if z >= self.layers.len() {
			panic!("указанное Z больше фактического последнего.");
		}
		let len_: usize = self.layers[z].len();
		let y_count: usize = input.clone().len();
		let mut input_: Vec<f32> = input;
		//let mut output: Vec<f32> = Vec::new();		
		for x in 0..len_ {
			//println!("отправили: {:?}", input_);
			let mut input_tmp: Vec<f32> = Vec::new();
			for i in 0..y_count {
				let r: f32 = self.layers[z][x].answer(input_.clone(), i);
				if r.is_nan() {
					break;
				}
				input_tmp.push(r);
			}
			input_ = input_tmp;
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
	pub fn new<F>(x: usize, y: usize, y_count: usize, fn_to_create: &'static Fn(usize, f32) -> (Vec<f32>, f32))->BufferNet
			{
		//LayerNet::new(y: usize)->LayerNet
		let mut lyrs: Vec<LayerNet> = Vec::new();
		let mut lrss: Vec<Vec<LayerNet>> = Vec::new();
		for i in 0..x.clone(){
			lyrs.push(LayerNet::new(y_count, fn_to_create));
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
	pub fn add<'a>(&mut self, arg: &'a str, value_x: Vec<usize>, fn_to_create: &'static Fn(usize, f32) -> (Vec<f32>, f32)) 
			{
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
				//let mut tmp: Vec<LayerNet> = Vec::new();
				//tmp.push(LayerNet::new_output_layer(y_count.clone()));
				//self.layers.push(tmp);
				//println!("naz -> {:?}", self.layers.len());
			}, // new answer z
			   // для ответа, output слой
			   // первый элемент - количество Y
			"nzx" => {
				let len_: usize = value_x.len();
				if len_ == 0 {
					panic!("попытка создать [z] с неинициализированными [y]");
				}
				let y_count: usize = value_x[0].clone();
				//println!("y_count in add 'nzx': {}", y_count);
				//let y_count: usize = value_x[1].clone();
				for i in 1..len_ {
					let mut tmp: Vec<LayerNet> = Vec::new();
					for k in 0..value_x[i] {
					//for _i in 0..value_x[i].clone() {
						tmp.push(LayerNet::new(y_count.clone(), fn_to_create));
					}
					//	//println!("_: {}", _i);
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
				
				for i in 0..count_x {
					//for k in 0..count_y {
						self.layers[indx_z].push(LayerNet::new(count_y.clone(), fn_to_create));
					//}
				}
			}, // to Z add x 
			   // при этом раскладе в первом элементе [0]
			   // будет указан index Z
			"nay" => {
				let len_: usize = value_x.len();
				if len_ == 0 {
					panic!("попытка добавить слой ответов, не указывая количество [y]");
				}
				if len_ == 1 {
					panic!("попытка добавить слой ответов, не указывая количество [z]");
				}
				let count_y: usize  = value_x[0].clone();
				let count_z: usize = value_x[1].clone();
				let mut tmp: Vec<LayerNet> = Vec::new();
				//for _i in 0..value_x[i].clone() {
				tmp.push(LayerNet::new_output_layer(count_y * count_z, fn_to_create));
				//	//println!("_: {}", _i);
				//}
				self.layers.push(tmp);
			}, // "nay", вектор);
			   // в векторе первым - колво Y, второе - колво объединяемых Z
			   // по формуле Y * Z (то есть колво нейронов на количество групп, будет всего колво связей)
			   // сделать только один слой дефакто, потом может добавлю для других
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
	pub fn grouping_according_to_latest_data(&mut self, x: usize, this_z: usize, last_z: usize) -> usize {
		println!("grouping_according_to_latest_data: [{}] [{}]", x.clone(), this_z.clone());
		let mut ret_val: usize = 0;
		//let step_l: usize = self.navigation_on_separate_lines.len();
		let this_step: usize = self.navigation_on_separate_lines.len().clone() - 1;
		//add_to_step_lastZ_thisZ(&mut self, step: usize , last_z: usize, this_z: usize)
		self.add_to_step_lastZ_thisZ(this_step.clone(), last_z, this_z.clone());
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
		println!("union_layer_in_step_and_z_: {:?}", union_layer_in_step_and_z_.clone());
		self.union_layer_in_step_and_z.push((this_step, this_z, union_layer_in_step_and_z_));
		ret_val
	}
	pub fn grouping_according_data_fromHere_toHere(&mut self, 
		from_here: usize, 
		to_here: usize, 
		this_z: usize, 
		last_z: usize
	) -> usize{
		let mut ret_val: usize = 0;
		let this_step: usize = self.navigation_on_separate_lines.len().clone() - 1;
		//add_to_step_lastZ_thisZ(&mut self, step: usize , last_z: usize, this_z: usize)
		self.add_to_step_lastZ_thisZ(this_step.clone(), last_z, this_z.clone());

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
	pub fn grouping_according_data_fromHere_toEnd(&mut self, from_here: usize, this_z: usize, last_z: usize) -> usize{
		let mut ret_val: usize = 0;
		let this_step: usize = self.navigation_on_separate_lines.len().clone() - 1;
		//add_to_step_lastZ_thisZ(&mut self, step: usize , last_z: usize, this_z: usize)
		self.add_to_step_lastZ_thisZ(this_step.clone(), last_z, this_z.clone());
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
	pub fn last_z_for_z(&self, step: usize, this_z: usize)->Result<usize, ()>{
		if step >= self.navigation_on_separate_lines.len().clone() {
			panic!("указан слишком большой шаг");
		}
		if step == 0 {
			panic!("не может быть предыдущего Z у нулевого шага");
		}
		let len_lastStep_lastZ: usize = self.navigation_on_separate_lines[step - 1].len();
		for i in 0..len_lastStep_lastZ { 
			let len_s: usize = self.navigation_on_separate_lines[step - 1][i].len(); 			
			for k in 0..len_s {
				if self.navigation_on_separate_lines[step - 1][i][k] == this_z {
					return Ok(i);
				}
			}
		}
		let unions_len: usize = self.union_layer_in_step_and_z.len();
		for i in 0..unions_len {
			if self.union_layer_in_step_and_z[i].0 == (step - 1) {
				for k in 0..self.union_layer_in_step_and_z[i].2.len(){
					if this_z == self.union_layer_in_step_and_z[i].2[k] {
						return Ok(k);
					}
				}
			}				
		} Err(())
	}
}
impl LogicalSheme {
	pub fn search_eq_value_and_return_indexs(index: usize, vector_: Vec<usize>)->Vec<usize>{
		let mut v: Vec<usize> = Vec::new();
		for i in 0..vector_.len(){
			if vector_[i] == index {
				v.push(i.clone());
			}
		} v
	}
	pub fn vectorf32_plus_vectorf32(mut vector_: Vec<f32>, arg_plus: Vec<f32>)->Vec<f32>{
		for i in 0..arg_plus.len() {
			vector_[i] += arg_plus[i];
		} vector_
	}
	pub fn on_error(&mut self, variable: usize, answer_index: usize, y_count: usize, net_ans: f32, true_result: f32, inputs_: Vec<f32>) -> Result<f32, ()> {
		/*
			impl BufferNet {
		*/
		//println!("-------------------------------------\nВошли в ошибки");
		//let y_count: usize = //errors.clone().len();
		let answers: Vec<(usize, usize)> = self.to_nav_map[variable].output_layer_in_step_and_in_z.clone();	
		let mut this_step: usize = answers[answer_index].0.clone() - 1;
		let mut prev_z: Vec<usize> = Vec::new();
		
		//let mut union_z_to_error: Vec<usize> = Vec::new();
		let mut errors: Vec<Vec<f32>> = Vec::new(); // [z][error value]
		let mut start: bool = true;
		//let mut prev_layers: Vec<LayerNet> = Vec::new();
		//let mut z_for_prev_layers: Vec<usize> = Vec::new();
		//let mut errors_prev_layers: Vec<Vec<f32>> = Vec::new();
		//let mut last_step_prev_layers: Vec<LayerNet> = Vec::new();
		//let mut last_step_z_for_prev_layer: Vec<LayerNet> = Vec::new();
		// z: usize, mut error: Vec<f32>, true_result: f32, mut prev_layer: LayerNet) -> Result<(Vec<f32>, LayerNet), ()> {
		// индексы ответов в формате Vec<(usize, usize)>, 
		// где первый usize -> шаг
		// второй -> index Z в шаге
		/*
			output_layer_in_step_and_in_z: Vec<(usize, usize)>, // output слой в шаге и в z
			// индексы ответов в формате Vec<(usize, usize)>, 
			// где первый usize -> шаг
			// второй -> index Z в шаге
			union_layer_in_step_and_z: Vec<(usize, usize, Vec<usize>)>,
			// индексы объединений в формате Vec<(usize, usize, Vec<usize>)>,
			// где первый - это шаг
			// второй - index z для объединения
			// третий - количество входящих z
			navigation_on_separate_lines: Vec<Vec<Vec<usize>>>, // навигация по отдельным линиям
			//[step][порядковый номер z предыдущий][тут текущие Z] его значение -> на какой порядковый отсылается
		*/
		let mut index_group: usize = 0;
		//println!("self.to_nav_map[variable].union_layer_in_step_and_z: {:?}", self.to_nav_map[variable].union_layer_in_step_and_z);
		for i in 0..self.to_nav_map[variable].union_layer_in_step_and_z.len() {
			if self.to_nav_map[variable].union_layer_in_step_and_z[i].0 == this_step /*&& 
				self.to_nav_map[variable].union_layer_in_step_and_z[i].1 == answer_index*/ {					
				for index_ in self.to_nav_map[variable].union_layer_in_step_and_z[i].2.clone(){
					//let v: Vec<usize> = vec![index_];
					prev_z.push(index_);	
					//prev_layers.push(LayerNet::new(0));
					//z_for_prev_layers.push(index_);		
				}				
				index_group = i.clone();
				if index_group == 0 { index_group = 1; }
				index_group -= 1;
				break;
			}
		}				
		//this_step += 1;		
		loop {
			if answer_index != 0 && answers[answer_index - 1].0 == this_step {
				break;
			}
			//println!("-------------------\nstep: [{}]\n-------------------", this_step);
			let mut this_z: Vec<usize> = prev_z.clone();
			//println!("this_z: {:?}", this_z);
			let mut this_error: Vec<Vec<f32>> = errors.clone();
			//let mut last_step_prev_layers: Vec<LayerNet> = Vec::new();
			//for _i in 0..prev_layers.len() {
			//	last_step_prev_layers.push(prev_layers[_i].clone());
			//}
			//let mut last_step_z_for_prev_layer: Vec<usize> = z_for_prev_layers.clone();
			//let last_step_errors: Vec<Vec<f32>> = errors_prev_layers.clone();

			//errors_prev_layers = Vec::new();
			//z_for_prev_layers = Vec::new();
			//prev_layers = Vec::new();
			errors = Vec::new();
			prev_z = Vec::new();

			if start {
				//println!("вошли в старт");
				// отправляем на уровень ниже
				// ищем ошибки
				start = false;
				let v: Vec<f32> = vec![(true_result.clone() - net_ans.clone())];
				//this_step -= 1;	
				//let count: usize = this_z.len() / y_count.clone();
				//pub fn for_start(&mut self, z: usize, last_z_s: Vec<usize>, true_answer: f32, last_step: BufferNet) -> Vec<f32> {
				//println!("self.variables_[variable][this_step - 1].layers.len(): {:?}", self.variables_[variable][(this_step - 1)].layers.len());
				let last_step = self.variables_[variable][this_step - 1].clone();
				// pub fn for_start(&mut self, z: usize, last_z_s: Vec<usize>, last_step: BufferNet, true_answer: f32, net_ans: f32) -> Vec<f32> {
				let result__ = self.variables_[variable][this_step].for_start(index_group, this_z.clone(), last_step, true_result.clone(), net_ans);
				
				//println!("self.variables_[variable][this_step].clone().layers[0][0].layer.len(): {}", 
				//	self.variables_[variable][this_step].clone().layers[0][0].layer.len());
				let count: usize = result__.len() / y_count.clone();
				//println!("result__: {:?}", result__);
				let mut z: usize = 0;
				let mut k: usize = 0;
				while k < count {
					let mut res: Vec<f32> = Vec::new();
					for i in 0..y_count {
						res.push(result__[z + i].clone());
					}
					errors.push(res);
					z += y_count.clone();
					k += 1;
					// z, err
					//last_step_prev_layers.push(self.variables_[variable][this_step].clone().layers[0][0].clone());
				}		
				//let mut this_z: Vec<usize> = prev_z.clone();
				//let mut this_error: Vec<Vec<f32>> = errors.clone();
				this_step -= 1;					
				prev_z = this_z.clone();
				/*
				let mut last_step_prev_layers: Vec<LayerNet> = prev_layers;
				let mut last_step_z_for_prev_layer: Vec<usize> = z_for_prev_layers;
				let last_step_errors: Vec<Vec<f32>> = errors_prev_layers.clone();
				*/
				//prev_layers = last_step_prev_layers;
				//println!("prev_layers.len(): {}", prev_layers.len());
				//z_for_prev_layers = last_step_z_for_prev_layer;
				//errors_prev_layers = errors.clone();
				//println!("errors: {:?}\nprev_z: {:?}", errors, prev_z);	
				//errors = this_error.clone();
				//println!("вышли из старта");
				continue;	
			}			
			// находим ошибки
			let mut in_xt: usize = 0;
			// переделать
			loop {
				//for i in 0..this_z.clone().len() {
				//println!("this_error: {:?}\nthis_z: {:?}", this_error, this_z);	
				if in_xt >= this_z.len() { break; }
				let v_tmp: Vec<usize> = LogicalSheme::search_eq_value_and_return_indexs(this_z[in_xt].clone(), this_z.clone());
				if v_tmp.len() == 1 { in_xt += 1; continue; }				
				let mut delete_value: usize = 0;
				//println!("z: [{}]\nerror: {:?}\nall_z: {:?}", v_tmp[0], this_error[v_tmp[0]], v_tmp.clone());
				for indx in 1..v_tmp.clone().len() {
					////println!("vector [z][{}]: {:?}\nvector_plus [z][{}]: {:?}", v_tmp[0].clone(), 
					//	this_error[v_tmp[0]].clone(), v_tmp[indx - delete_value], this_error[indx - delete_value].clone());
					//let v_tmp_: Vec<f32> = this_error[v_tmp[0]].clone();
					////println!("LogicalSheme::vectorf32_plus_vectorf32: {:?}", LogicalSheme::vectorf32_plus_vectorf32(
					//	v_tmp_.clone(), this_error[v_tmp[indx - delete_value]].clone()));
					//this_error[v_tmp[0]] = LogicalSheme::vectorf32_plus_vectorf32(
					//	v_tmp_.clone(), this_error[v_tmp[indx - delete_value]].clone());
					//if v_tmp[0].clone() == v_tmp[indx].clone() {
					this_z.remove(v_tmp[indx - delete_value]);
					this_error.remove(v_tmp[indx - delete_value]);
					delete_value += 1;
					//}
				}
				//in_xt += 1;
			}
			//println!("this_error: {:?}\nthis_z: {:?}", this_error, this_z);	
			let mut count_element_in_group: usize = 0;
			// ищем следующие Z для обновления по шагам
			for last_z_ in 0..self.to_nav_map[variable].navigation_on_separate_lines[this_step].len() {
				//let mut to_add: Vec<usize> = Vec::new();				
				for this_z_ in 0..self.to_nav_map[variable].navigation_on_separate_lines[this_step][last_z_].len() {
					
					for i in 0..this_z.len() {
						if index_group < self.to_nav_map[variable].union_layer_in_step_and_z.len() &&
							self.to_nav_map[variable].union_layer_in_step_and_z[index_group].0 == this_step && 
							self.to_nav_map[variable].union_layer_in_step_and_z[index_group].1 == this_z[i] {
							// z: usize, mut error: Vec<f32>, true_result: f32, mut prev_layer: LayerNet) -> Result<(Vec<f32>, LayerNet), ()> {
							
							/*pub fn error(&mut self, 
								z: usize, mut error: Vec<f32>, true_result: f32, 
									mut prev_layer: LayerNet) -> Result<(Vec<f32>, LayerNet, LayerNet), ()> {
								// предыдущий (нулевой) с предыдущего step [z][0], возвращаем ошибки, старый предыдущий, новый предыдущий
								
								pub fn last_z_for_z(&self, step, this_z)->Result<usize, ()>

								insert_(&mut self, z, x, layernet)
								let mut prev_layers: Vec<LayerNet> = Vec::new();
								let mut z_for_prev_layers: Vec<usize> = Vec::new();
								let mut errors_prev_layers: Vec<Vec<f32>> = Vec::new();

								let last_step_errors: Vec<Vec<f32>> = errors_prev_layers.clone();
								let mut last_step_prev_layers: Vec<LayerNet> = prev_layers;
								let mut last_step_z_for_prev_layer: Vec<usize> = z_for_prev_layers;

							*/
							let mut result__: Vec<f32> = Vec::new();
							let mut hid_err: Vec<f32> = Vec::new();
							// self.variables_[variable][this_step - 1].clone().get_output_to_las_x(last_z_);
							for _z in self.to_nav_map[variable].union_layer_in_step_and_z[index_group].2.clone() {
								/*let out_temp: Vec<f32> = self.variables_[variable][this_step].
															clone().get_output_to_las_x(_z);*/
								for f_32 in this_error[_z].clone() {
									hid_err.push(f_32.clone());
								}
							}
							// z: usize, fact_result: f32, true_result: f32, inputs: Vec<f32>, hidden_errors: Vec<f32>
							/*result__ = match */self.variables_[variable][this_step].error(
								this_z[i].clone(), net_ans.clone(), 
								true_result.clone(), inputs_.clone(), hid_err.clone());/* {
									Ok(A) => { A },
									Err(e)=> { return Err(()); Vec::new() }
							};*/
							//result__ = result__t;

							let count: usize = self.to_nav_map[variable].
												union_layer_in_step_and_z[index_group].2.len() / y_count.clone();
							let mut z: usize = 0;
							while z <= (count + 1) {
								let mut res: Vec<f32> = Vec::new();
								for i in 0..y_count {
									res.push(hid_err[z + i].clone());
								}
								errors.push(res);
								z += y_count.clone();
								// z, err
							}
							//println!("errors: {:?}", errors.clone());

							for index_ in self.to_nav_map[variable].union_layer_in_step_and_z[index_group].2.clone(){
								prev_z.push(index_);
							}			
							index_group -= 1;
							continue;			
						}
						if self.to_nav_map[variable].navigation_on_separate_lines[this_step][last_z_][this_z_] == this_z[i] {
							//println!("this_error: {:?}\ni: {}", this_error, i);
							//let mut result__: Vec<f32> = Vec::new();
							//println!("z с которым работам, его индекс(\ni: [{}],\nзначение: [{}]\n)", i.clone(), this_z[i].clone());							
							//if this_step != 0 {
								/*let last_out: Vec<f32> = self.variables_[variable][this_step - 1].clone().get_output_to_las_x(last_z_);
								let result__t = match*/
								// z: usize, fact_result: f32, true_result: f32, inputs: Vec<f32>, hidden_errors: Vec<f32>
								//let mut this_z: Vec<usize> = prev_z.clone();
								//let mut this_error: Vec<Vec<f32>> = errors.clone();
								self.variables_[variable][this_step].error(
									this_z[i].clone(), net_ans.clone(), 
									true_result.clone(), inputs_.clone(), this_error[i].clone());/* {
										Ok(A) => { A },
										Err(e)=> { return Err(()); Vec::new() }
								};
								result__ = result__t; 
							} else {
								let last_out: Vec<f32> = inputs_.clone();
								let result__t = match self.variables_[variable][this_step].error(
									this_z[i].clone(), this_error[i].clone(), 
									true_result.clone(), last_out) {
										Ok(A) => { A },
										Err(e)=> { return Err(()); Vec::new() }
								};
								result__ = result__t;*/
							//}						
							//println!("вышли");
							
							errors.push(this_error[i].clone());
							prev_z.push(last_z_.clone());							
							//println!("errors: {:?}\nprev_z: {:?}", errors, prev_z);
						}											
					}
				}				
			}// end circle
			if this_step == 0 { break; }
			this_step -= 1;
		} Ok(0.0)
	}
	pub fn answer<F>(&mut self, variable: usize, inputs: Vec<f32>, answer_function: F) -> Vec<f32> 
		where F: Fn(f32, usize) -> f32 {	// input -> output
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
		let mut index_union_in_step: usize = 0;
		for step in 0..steps {	
			// буду переделывать, почти всё готово
			//println!("step: [{}]", step);						
			let mut temp_input: Vec<Vec<f32>> = output_.clone();
			output_ = Vec::new();

			let map_zets_last_step: usize = self.to_nav_map[variable]
				.navigation_on_separate_lines[step].len();
			//println!("map_zets_last_step: {:?}", map_zets_last_step);
			////println!("косфк?");
			if ans_indx < answers.len() && step == answers[ans_indx].0.clone() {
				// ответы сети
				//println!("вошли в ответ");
				let res: f32 = answer_function( 
					//if temp_input[answers[ans_indx].clone().1].clone().len() == 1 {
						temp_input[answers[ans_indx].clone().1].clone()[0], ans_indx.clone()
					//} else {
					//	panic!("вы не объединили слоя, невозможно выдать ответ");
					//	0.0
					//}
				).clone();
				for_return.push(res);
				ans_indx += 1;
				output_ = temp_input.clone();
				//println!("output_ from answer: {:?}", output_);
				continue;
			}
			//println!("да нет косяка");
			
			if group_index < group_on_step_z_count.len() && 
						step == group_on_step_z_count[group_index].0.clone() {
				//
				while 
					group_index < group_on_step_z_count.len() && 
						step == group_on_step_z_count[group_index].0.clone() {
				let mut input_s: Vec<f32> = Vec::new();
					//println!("group_on_step_z_count[group_index].clone(): [step][z][z-s]  {:?}", group_on_step_z_count[group_index].clone());
					for z_to_group in group_on_step_z_count[group_index].2.clone() {
						for i in 0..temp_input[z_to_group].len() {
							input_s.push(temp_input[z_to_group][i]);
						}
					}
					//println!("input_for_group: {:?}", input_s);
					let mut r: Vec<f32> = self.variables_[variable][step].answer(group_on_step_z_count[group_index].1.clone(), input_s);
					for _ in 0..(y_count - 1){
						r.push(r[0].clone());
					}
					for _ in 0..y_count{
						output_.push(r.clone());
					}
					index_union_in_step += 1;
					group_index += 1;
					//println!("output_ from group: {:?}", output_);
				}
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
				//println!("output: {:?}", output_);
				continue;
			}
			////println!("косяк?");
			////println!("да не");
			for index_last_z in 0..map_zets_last_step {
				//println!("зашли");
				for index_this_z in 0..self.to_nav_map[variable].navigation_on_separate_lines[step][index_last_z].len() {
					//println!("index_this_z: {}", index_this_z);
					////println!("передаём в: [{}]\nvalue: {:?}", index_this_z, temp_input[index_last_z].clone());
					//println!("temp_input: {:?}", temp_input);
					////println!("косяк?");
					if self.variables_[variable][step].z_count() <= (index_this_z + index_union_in_step) {
						// конец сети...
						break;
					}
					////println!("да не");
					output_.push(self.variables_[variable][step].answer(index_this_z + index_union_in_step, temp_input[index_last_z].clone()));
				
				}
			} 
			//println!("output: {:?}", output_);
			index_union_in_step = 0;
		} 
		// даём ответ
		//input_ = Vec::new();
		//for index_answer in 0..self.variables_[variable][steps - 1].get_z_len() {

		//}
		//println!("self.answer end");
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
	pub fn edit_var<F>(&mut self, var_name: String, var_value: String, fn_to_create: &'static Fn(usize, f32) -> (Vec<f32>, f32))->bool
			 {
		
		let mut indx: usize = match self.search_var(var_name.clone()) {
			Ok(A) => { A },
			Err(e)=> { return false; 0 },
		};
		self.variables_[indx] = Vec::new();
		self.parser(var_value.clone().as_str(), fn_to_create);
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
	pub fn parser<'a>(&mut self, mut line_: &'a str, fn_to_create: &'static Fn(usize, f32) -> (Vec<f32>, f32) )
				 {
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
		let mut index_union : usize = 0;
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
									.grouping_according_to_latest_data(value_.clone(), index_union.clone(), index_last_z.clone());

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
										.grouping_according_data_fromHere_toHere(from_here, to_here, index_union, index_last_z.clone());
								} else {
									value__ = self.to_nav_map[variable_index]
										.grouping_according_data_fromHere_toEnd(from_here, index_union, index_last_z.clone());
								}
								//value_in_z.push(1);
							}
							union_layer = false;	
							index_union += 1;						
							let len_: usize = self.variables_[variable_index].len();
							let temp_vec: Vec<usize> = vec![y_count.clone(), value__.clone()];
							self.variables_[variable_index][len_ - 1].add("nay", temp_vec, fn_to_create);
							value_in_z.insert(0, y_count.clone());
							println!("step value_in_z([y][x][x]): {:?}", value_in_z.clone());
							self.variables_[variable_index][len_ - 1].add("nzx", value_in_z.clone(), fn_to_create);
							
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
							self.variables_[variable_index][len_ - 1].add("naz", temp_, fn_to_create);
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
							//if index_this_z != 0 {
								let value_: usize = match buffer_text.trim().parse::<usize>() {
									Ok(A)=>{ A },
									Err(e)=>{ panic!("не получилось прочитать число около запятой."); 0 },
								};
								buffer_text = "".to_string();	
								this_z_array.push(index_this_z.clone());
								value_in_z.push(value_.clone());
							//}
							
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
							self.variables_[variable_index][len_ - 1].add("nzx", value_in_z.clone(), fn_to_create);
							
							self.variables_[variable_index].push(BufferNet::new_empty());
							//BufferNet::new_empty
							// тут не просто обнуление!
							
							//to_z_navigation = Vec::new();
													
							step_value += 1;
							if step_value != self.to_nav_map[variable_index].new_step() {
								panic!("ошибка в карте и шагах!");
							}	
						}				
						index_union = 0;		
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
						let mut z_count: usize = 0;
						if fromHere_toHere.len() == 1 {
							let value_: usize = match fromHere_toHere[0].to_string().trim().parse::<usize>() {
								Ok(A)=>{ A },
								Err(e)=>{ panic!("не получилось прочитать число групп запятой."); 0 },
							};
							println!("value: {}",value_);
							z_count = self.to_nav_map[variable_index]
								.grouping_according_to_latest_data(value_.clone(), index_union.clone(), index_last_z.clone());

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
								z_count = self.to_nav_map[variable_index]
									.grouping_according_data_fromHere_toHere(from_here, to_here, index_union, index_last_z.clone());
							} else {
								z_count = self.to_nav_map[variable_index]
									.grouping_according_data_fromHere_toEnd(from_here, index_union, index_last_z.clone());
							}
							// value_in_z.push(1); написать им новый 'nay' (New Answer Layer) в
							// self.variables_[variable_index][len_ - 1].add("nay", вектор);
							// в векторе первым - колво Y, второе - колво объединяемых Z
							// по формуле Y * Z (то есть колво нейронов на количество групп, будет всего колво связей)
							// сделать только один слой дефакто, потом может добавлю для других
						}
						let temp_vec: Vec<usize> = vec![y_count.clone(), z_count.clone()];
						self.variables_[variable_index][len_ - 1].add("nay", temp_vec, fn_to_create);
						index_union += 1;
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

fn answer_function(answer: f32, number: usize)->f32{
	//println!("answer: {}", answer);
	answer
}
fn for_create_weight(weight_count: usize, learn_speed: f32) -> (Vec<f32>, f32) {
	let mut weight_: Vec<f32> = Vec::new();
	for i in 0..weight_count {
		weight_.push(0.1);
	}
	println!(" вернули: {:?}, 0.01", weight_.clone());
	(weight_, 0.01)
} //where F: Fn(usize, f32) -> (Vec<f32>, f32)
fn main() {
    println!("Hello, world!");
	
	
	let mut v: Vec<f32> = vec![0.0, 0.0];
	/*let v1: Vec<f32> =vec![1.0, 0.0];
	let v2: Vec<f32> =vec![1.0, 1.0];
	let v3: Vec<f32> =vec![0.0, 0.0];*/
	let mut b: bool = true;
	//loop {
		let mut t = LogicalSheme::new();
		/*
			___*
			_*_*
			___*
			___*
			___*
			0001 0101 0001 0001 0001

			****
			___*
			****
			*___
			****
			1111 0001 1111 1000 1111

			****
			___*
			****
			___*
			****
			1111 0001 1111 0001 1111

			*__*
			*__*
			****
			___*
			___*
			1001 1001 1111 0001 0001

			****
			*___
			****
			___*
			****
			1111 1000 1111 0001 1111

			****
			*___
			****
			*__*
			****
			1111 1000 1001 1111

			****
			___*
			___*
			___*
			___*
			1111 0001 0001 0001 0001

			****
			*__*
			****
			*__*
			****
			1111 1001 1111 1001 1111

			****
			*__*
			****
			___*
			****
			1111 1001 1111 0001 1111

			****
			*__*
			*__*
			*__*
			****
			1111 1001 1001 1001 1111

		*/
		let one: Vec<f32> = vec![0.1,0.1,0.1,0.9, 0.1,0.9,0.1,0.9, 0.1,0.1,0.1,0.9, 0.1,0.1,0.1,0.9, 0.1,0.1,0.1,0.9];
		let five: Vec<f32> = vec![0.9,0.9,0.9,0.9, 0.9,0.1,0.1,0.1, 0.9,0.9,0.9,0.9, 0.1,0.1,0.1,0.9, 0.9,0.9,0.9,0.9];
		let two: Vec<f32> = vec![0.9,0.9,0.9,0.9, 0.1,0.1,0.1,0.9, 0.9,0.9,0.9,0.9, 0.9,0.1,0.1,0.1, 0.9,0.9,0.9,0.9];
		let seven: Vec<f32> = vec![0.9,0.9,0.9,0.9, 0.1,0.1,0.1,0.9, 0.1,0.1,0.1,0.9, 0.1,0.1,0.1,0.9, 0.1,0.1,0.1,0.9];
		//println!("count: {}", five.len());
		//return;
		t.parser("
		' comment
		main:  20 -> 2, 2, 2 -> 4, 4 | 2 | 2 -> ^0-1 | 1 | ^1-1 | ^2-2 -> <0> -> De out", &for_create_weight);
		//let five: Vec<f32> = vec![0.9, 0.9];
		//let two: Vec<f32> = vec![0.9, 0.1];
		//let seven: Vec<f32> = vec![0.1, 0.9];
		let mut ans: f32 = 0.0;
		let mut s: Vec<f32> = Vec::new();
		//let mut b: usize = 0;
		for i in 0..600/*8_000*/ {
			//if i % 10 == 0 {
				println!(" итерация: [{}]", i.clone());
			//}
			let mut rng = rand::thread_rng();
		    //if rng.gen() { // random bool
		    let mut f: f32 = rng.gen::<f32>();
			let h: u8 = rng.gen::<u8>();
			if h < 100 {
				v = five.clone();
			} else if h > 100 && h < 200 {
				v = two.clone();
			} else if h > 200 {
				v = seven.clone();
			}
			ans = t.answer(0, v.clone(), answer_function)[0];
			let l_: usize = s.len();
			//if s[l_ - 1][0] > 0.99 {
			//	println!("---------------------------------------");
			//	println!("цикл на котором всё ок: {:?}", i);
			//	break;
			//}
			if h > 100 && h < 200 {
				//if ans[l_].0[0] > 0.9 { s.push(0.0); }
				//b += 1;
				if ans < 1.0 {
					t.on_error(0, 0, 20, ans, 1.0, v.clone());	
				} 
			} else {
				//if ans[0] > 0.9 { s.push(1.0); }
				if ans > 0.5 {
					t.on_error(0, 0, 20, ans, 0.5, v.clone());		
				}	
			}
		}
		let v: Vec<f32> = vec![t.answer(0, five.clone(), answer_function)[0], t.answer(0, two.clone(), answer_function)[0],
		t.answer(0, seven.clone(), answer_function)[0]];
		println!("its five:    {}", v[0]);
		println!("its no five: {}", v[1]);
		println!("its no five: {}", v[2]);
		//let s1= t.answer(0, v.clone(), answer_function);
		println!("---------------------------------------");
		//println!("s: |{:?}|", (s.len()/b));
		/*loop {				
			let ans_ = t.answer(0, v.clone(), answer_function);
			if ans_[0] == s[0] {
				println!("повторяются? [{}]", ans);
				break;
			}
			if ans_[0] > 0.8 {
				break;
			}
			t.on_error(0, 0, 2, (1.0 - ans_[0]), 1.0);
			let ans_ = t.answer(0, v.clone(), answer_function);
			s = ans_.clone();
			t.on_error(0, 0, 2, (1.0 - ans_[0]), 1.0);
			t.on_error(0, 0, 2, (1.0 - ans_[0]), 1.0);
			//s = ans_.clone();
			ans += 1;
		}*/
		//ans.push(t.answer(0, v.clone(), answer_function)[0]);
		//let ans = t.answer(0, v.clone(), answer_function);
		
		println!("answer: {:?}", ans);
		/*loop {
			let ans = t.answer(0, v.clone(), answer_function);
			if ans[0] < 0.1 && ans[1] > 0.9 && ans[2] > 0.9 { println!("answer: {:?}", ans.clone()); b = false; break; }
			//if ans[2] > 0.9 { println!("answer: {:?}", ans.clone()); b = false; break; }
			println!("---------------------------------------");
			println!("answer: {:?}", ans.clone());
			//println!("---------------------------------------");
			if ans[0] > 0.1 {
				match t.on_error(0, 0, 2, (0.0 - ans[0]), 0.0){
					Ok(A) => {},
					Err(e)=> { break; }
				}
			}
			if ans[1] < 0.9 {
				match t.on_error(0, 1, 2, (1.0 - ans[2]), 1.0){
					Ok(A) => {},
					Err(e)=> {  break; }
				}
			}
			if ans[2] < 0.9 {
				match t.on_error(0, 2, 2, (1.0 - ans[2]), 1.0){
					Ok(A) => {},
					Err(e)=> {  break; }
				}
			}
		} if b == false { break; }*/
	//}
	
	/*
		pub fn answer<F>(&self, variable: usize, inputs: Vec<f32>, answer_function: F) -> Vec<f32> 
		where F: Fn(f32) -> f32 {	// input -> output
		on_error<F>(&mut self, variable: usize, answer_index: usize, y_count: usize, error: f32, true_result.clone())
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

