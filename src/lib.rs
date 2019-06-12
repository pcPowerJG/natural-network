use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::str;

use std::net::TcpStream;
pub mod ServersModule {
    pub struct Thread {
	    id: usize,
	    thread: Option<std::thread::JoinHandle<()>>,
    }
    pub fn new_thread(id: usize, new_thread: Option<std::thread::JoinHandle<()>>) -> Thread {
        Thread { id: id, thread: new_thread }
    }
}
pub mod sstring{
	pub struct Sstring {
		chars_: Vec<char>,		
	}
	impl Sstring {
		pub fn new() -> Sstring { Sstring { chars_: Vec::new() } }
		pub fn len(&self) -> usize { self.chars_.len() }
		pub fn get_chars(&self) -> Vec<char> { self.chars_.clone() }
		pub fn clone(&self) -> Sstring { Sstring { chars_: self.chars_.clone() } }
		pub fn get_char(&self, index: usize) -> char { self.chars_[index] }
		pub fn from_string(&mut self, cell: String) { 
			let mut r: Vec<char> = Vec::new();
			for ch in cell.chars() { r.push(ch); }
			self.chars_ = r;
		}
		pub fn remove(&mut self, index: usize) {
			self.chars_.remove(index);
		}
		pub fn to_string(&self) -> String {
			let mut result: String = String::new();
			for ch in self.chars_.clone() {
				result.push(ch);
			}
			result
		}
		pub fn insert(&mut self, indx: usize, ch: char) {
			self.chars_.insert(indx, ch);
		}
	}
}

pub mod NeuralNetwork{
	pub struct Net{
		data_base: Vec<Neywork>,
		map_step: Vec<(usize, usize, usize)>,// нейрон1 - нейрон2 - вход
	}
	pub struct Neywork{
		weight: Vec<f32>,
		inputs: Vec<f32>,
		learn_speed: f32,

		result: f32,

	}
	pub fn new()->Net{
		Net { data_base: Vec::new(), map_step: Vec::new() }
	}

	impl Neywork{
		pub fn proceed(&mut self){
			let mut r: f32 = 0.0;

			for i in 0..self.inputs.len(){
				r += (self.inputs[i] * self.weight[i]) + self.learn_speed;
			}
			self.result = r;
		}
		pub fn on_error(&mut self, true_result: f32){
			let delta: f32 = true_result - self.result;
			for i in 0..self.inputs.len(){
				self.weight[i] = self.weight[i] + (self.inputs[i] * delta * self.learn_speed);				
			}
		}
	}
	impl Net{
		pub fn new_neyron(&mut self, weight_count: usize, learn_speed: f32)->bool{
			let mut t1: Vec<f32> = Vec::new();
			for i in 0..weight_count{
				t1.push(0.0);
			}			
			let temp: Neywork = 
				Neywork{ weight: t1.clone(), inputs: t1.clone(), learn_speed: learn_speed, result: 0.0 };
			self.data_base.push(temp);
			true
		}
		pub fn remove_neyron(&mut self, index: usize){
			self.data_base.remove(index);
		}
		pub fn add_step(&mut self, neyron_output: usize, neyron_to: usize, neyron_to_inputID: usize){
			self.map_step.push((neyron_output, neyron_to, neyron_to_inputID));
		}
		pub fn remove_step(&mut self, index: usize){
			self.map_step.remove(index);
		}
		pub fn len(&self)->usize{ self.data_base.len() }
	}
}
#[warn(non_snake_case)]
#[warn(dead_code)]
pub mod Language{
    	use crate::ServersModule;	
	use crate::sstring::*;
	// endpointautomate
	use std::net::*;
	pub struct Words{
			words: Vec<String>,                               // буква (номер от 1 (a-z)), слово
        	neural_network: Net,        			  		  // сама сеть
			servers: Vec<ServersModule::Thread>,              // сервера
			//buffer_action: Vec<[usize; 3]>,                 // буффер для действий
			object_buffer: Vec<(String, usize)>,              // наименования объектов 	// (name, type) // 0 - нейрон, 1 -  объект, 2 - сервер, 3 - массив, 4 - структура, 10 - функция
	        value_buffer: Vec<String>,                        // значения 
	}
	pub fn on_create() -> Words {
		let mut words: Vec<String> = Vec::new(); 
		words.push("neyron".to_string());//1 // используется для создания нового перцептона
		/*
			ПРИМЕР:
				neyron one { 0.005, 0.123, 0.576 }			; создаёт нейрон с именем 'one' и значением весов: 0.005, 0.123, 0.576
				neyron two [3]								; создаёт нейрон с 3мя пустыми (нулевыми) весами
		*/
		words.push("server".to_string());//2   // используется для явного указания создания сервера и вывод нейросети в отдельный поток
		/*
			ПРИМЕР: 
				server server1 = 192.168.0.1:8085 
			создаст новый сервер, к которому мужно будет подключится введя строку подключения
		*/
				
		words.push("object".to_string());//3	 // используется для создания объекта, который хранит значения в памяти
		/*
			значение этого типа можно использовать как и где угодно. 
			ПРИМЕР1, импользование значения типа object для динамического создания сервера 

				serv server1 = 192.168.0.1:8085				; создаём сервер, чтобы принять парамерты
				object server2_text = 192.168.0.1:8085		; принимаем первый параметр с сервера и помещаем его значение в объект server2_text

				server1 = server2_text						; создаём второй сервер на ip и порту полученному с параметра 
		*/
				
		words.push("if".to_string());//4		// оператор условия, нужен для сравнения ДВУХ параметров
		/*
			ПРИМЕР:
				serv server1 = 192.168.0.1:8085				; создаём сервер, чтобы принять парамерты
				object one = server1 [0]					; принимаем первый параметр с сервера
				object two = server1 [1]					; принимаем второй параметр с сервера

			    if one == two 
					send server1	one + two: String		; отправляем ответ серверу в который помещаем сложение строк объектов one и two
					send server1	ont + two: Float		; отправляем ответ серверу в который помещаем сложение чисел объектов one и two
				end
		*/
		words.push("else".to_string());//5		// оператор условия, если первое условие не выполнилось
		/*
			ПРИМЕР: 
				serv server1 = 192.168.0.1:8085				; создаём сервер, чтобы принять парамерты
				object one = server1 [0]					; принимаем первый параметр с сервера
				object two = server1 [1]					; принимаем второй параметр с сервера

			    if one == two								; если значение one и two равны, то
					send server1	one + two: string		; отправляем ответ серверу в который помещаем сложение строк объектов one и two
					send server1	ont + two: float		; отправляем ответ серверу в который помещаем сложение чисел объектов one и two
				else
					exit_()									; иначе завершаем приложение
				end
		*/


		words.push("{".to_string());//6			// открывающий символ используется в условиях и при создании нового объекта
		words.push("}".to_string());//7			// закрывающий символ используется в условиях и при создании нового объекта
		words.push(">".to_string());//8			// знак больше
		words.push("<".to_string());//9			// знак меньше
		words.push("[".to_string());//10	    // массив
		words.push("]".to_string());//11		// массив

		words.push("-".to_string());//12		// знак минус, но в сочетании со знаком '>' ('->') позволяет выполнять итерации по сети
		words.push(":".to_string());//13		// знак явного указания типа
		words.push(".".to_string());//14		// знак вызова подпараметров объекта 

		words.push("=".to_string());//15		// знак присваивания а в сочетании с себе подомным '==' позволяет сравнивать объекты
		words.push(";".to_string());//16		// комментарий
		//255 - не читаемое (переменные)
		words.push("_".to_string());//17		// знак без смысловой нагрузки
		//255 - добавить
		words.push("send".to_string());//18		// отправка значения объекта на сервер
		/*
			ПРИМЕР:
				serv server1 = 192.168.0.1:8085				; создаём сервер на данном ip:port
				object obj_to_send = Привет Мир				; создаём объект со значением
				send server1 obj_to_send					; отправляем текст "Привет Мир" на сервер

		*/
		words.push("exit_()".to_string());//19//выход из приложения
		words.push("func".to_string());     //20//инициализация функции (param PARAM_NAME [PARAM_COUNT]) для приёма с сервера
		/*
			ПРИМЕР:
				func func(objc)								; создаём функцию
					print objc
				end_func
				
		*/
		
		words.push("print".to_string());//21 // вывод на консоль
		/*
			ПРИМЕР:
				param parametrs [2]							; создаём 2 параметра
				serv server1 = 192.168.0.1:8085				; создаём сервер
				server1 -> parametrs						; помещаем значения параметров в 'parametrs'

				object obj1 = parametrs [0]					; помещаем значение первого параметра в 'obj1'

				print obj1									; печатаем на консоль значение объекта 'obj1'
		*/
		words.push("remove".to_string());//22 //удаление
		/*
		    ПРИМЕР: 
		        object obj1                                 ; создали объект
		        remove obj1                                 ; удалили объект
		*/
		words.push("launch".to_string());//23 //запуск сервера в режиме приёма сообщений
		/*
		    ПРИМЕР:
		        server serv1 =192.168.0.1:8072              ; создаём сервер и ip
		        launch serv1                                ; вводим сервер в режим приёма сообщений
		*/
		// -------------------------------------
		//101 - добавить из вектора
		words.push("array".to_string());//24 // создание массива
		/*
			ПРИМЕР:
				object obj1
				object obj2

				array name_.size 
				; реализация в памяти
				array ar1 = { obj1, obj2, "hello, world" }

				print ar1[1]
				; print ''
				print ar1[2]
				; print 'hello, world'
		
		*/
		words.push("int".to_string()); //25 //создание переменной типа INT
		/**/
		words.push("float".to_string()); //26 // создание переменной типа f64
		/**/
		words.push("string".to_string()); //27 // создание переменной типа String
		/**/
		words.push("void".to_string());   //28 // тип void, для функций без параметров
		/**/
		words.push("struct".to_string()); //29 // создание структуры
		/*
				struct st_one
					string s1
					int s2
					object s3
				end
		*/
		words.push("\"".to_string()); //30 // служебное, для текста
		words.push("to".to_string()); //31 // для сервера
		/*
			server s1 = 192.168.0.1
			network to s1
				|728| -> ||500, 500|->|200|| -> |10|
				
				; 728 входов, 500 и 500 изолированных слоёв, которые переходят в 200 слоёв и они в свою очередь переходят в 10 выходов
			end
		*/
		words.push("network".to_string());//32// для сети
		words.push("end".to_string());//33//end operation
		words.push("end_func".to_string()); // 34 // конец функции
		/*
			func func(void)

			end_func
			
		*/ // 20 - fn, 34 - end_fn, 28 - void
		words.push("call-fn[zZaZ]~(kJmN)->".to_string()); // 35 // служебное, только для вызова функции
		// мы начинаем читать файл, с которым работаем с начала, дабы найти вызываемую функцию
		
		Words{ words: words,  neural_network: new(), servers: Vec::new(), object_buffer: Vec::new(), value_buffer: Vec::new() }
	}
	//impl
    /*pub fn clone(to: &mut Words, from: &Words){
        
    }*/
	impl Words{ 
		pub fn eq_char_in_string(ch: char, st: &String)->bool{
			for ch_in_st in st.chars(){
				if ch == ch_in_st{
					return true;
				}
			} false
		}
		pub fn eq_char_in_string_r(ch: char, mut st: String, symbol_count: usize)->bool{ 
			st = st.clone().chars().rev().collect::<String>();
			let mut i = 0;
			for ch1 in st.chars() {
				if i > symbol_count { return false; }
				if ch == ch1 { return true; }
				i += 1;
			}
			false 
		}
		pub fn is_digit(st: String) -> bool {			
			for ch in st.chars() {
				match ch {
					'0' => {},
					'1' => {},
					'2' => {},					
					'3' => {},
					'4' => {},
					'5' => {},
					'6' => {},
					'7' => {},
					'8' => {},
					'9' => {},
					_ => { return false; },
				}
			} true
		}
		pub fn get_index(&self, mut name: String) -> Result<usize , ()> {
			//pub fn eq_char_in_string_r(ch: char, mut st: String, symbol_count: usize)->bool{ 
			if !Words::eq_char_in_string_r('\0', name.clone(), 1){
				name = name.clone().as_str().trim().to_string(); // проверяем на признак конца строки..
			} else { return Err(()); }
			//println!("name: {}", name.clone());
			let mut search: usize = 0; 
			let mut flag: bool = false;
			let mut struct_count: usize = 0;

			let mut cell_name: String = String::new();
			for i in 0..self.object_buffer.len() {
				if search != 0 { 
					search -= 1;
					continue;
				}
				if flag == true {
					return Ok(i);
				}	
				if struct_count != 0 && cell_name != String::new() {
					struct_count -= 1;
					if struct_count == 0 { panic!("Var in struct not found. Code 404_3"); }
				}							
				let (name_, type_) = &self.object_buffer[i];
				let v: Vec<&str> = name_.split('.').collect();
				//println!("{:?}", v.clone());
				if v.len() == 1 {					
					if *name_ == name || (*name_ == cell_name && cell_name != String::new()) {
						return Ok(i);
					} 					
				} else {
					let index_if: Vec<&str> = name.split('[').collect();
					if v[0].to_string() != index_if[0].to_string() { continue; }
					
					
					//let mut index_count: usize = 0;
					if index_if.len() > 1 { 
						let index_if: Vec<&str> = index_if[1].split(']').collect();
						println!("{:?}", index_if.clone());
						let mut cell: Sstring = Sstring::new();
						cell.from_string(index_if[0].clone().to_string());
						if index_if.len() > 1 {
							if !Words::eq_char_in_string('\"', &index_if[0].to_string()) { 
								//"
								search = match index_if[0].clone().parse() {
									Ok(A) => { A },
									Err(e) => { panic!("Index array error. Code 404_1"); 0 }
								};
								flag = true;
							} else {
								// структура
								// через цикл вывести имя поля, проверить есть ли ']' в конце
								// и вытащить номер
								// будем считать что на вход передано обращение без пробелов (то бишь без имя [10]
								// либо name ["asasas" ] ; struct1["name"]
								// память имена: имя.колво, имя_поля, имя_поля, имя_поля, ... , прочие_имена
								// память значения: "", "значение_1", "значение2", "значение3", ... , "прочие_значения"
								let len: usize = cell.len();
								if len < 2 { panic!("Cell in Struct error. Code 404_2"); }
								cell.remove(len-1);
								cell.remove(0);
								cell_name = cell.to_string();
								struct_count = match v[1].clone().trim().parse() { 
									Ok(A) => A,
									Err(e) => { panic!("memory array error. Code 201"); 0 },
								}; struct_count += 1;
							}
						}
					}									
				}				
			} Err(())
		}
		pub fn get_index_r(&self, mut name: String) -> Result<usize , ()> {
			if !Words::eq_char_in_string_r('\0', name.clone(), 1){
				name = name.clone().as_str().trim().to_string(); // проверяем на признак конца строки..
			} else { return Err(()); }
			//println!("name: {}", name.clone());
			let mut search: usize = 0; 
			let mut flag: bool = false;
			let mut struct_count: usize = 0;

			let mut cell_name: String = String::new();
			let mut i: usize = self.object_buffer.len() - 1;
			loop {
				//for i in 0..self.object_buffer.len() {
				if search != 0 { 
					search -= 1;
					i -= 1;
					continue;
				}
				if flag == true {
					return Ok(i);
				}	
				if struct_count != 0 && cell_name != String::new() {
					struct_count -= 1;
					if struct_count == 0 { panic!("Var in struct not found. Code 404_3"); }
				}							
				let (name_, type_) = &self.object_buffer[i];
				let v: Vec<&str> = name_.split('.').collect();
				if *type_ != 1 {
					// сложный тип данных
					if i == 0 {
						return Err(());
					}
					i -= 1; 
					continue;
				}
				//println!("{:?}", v.clone());
				if v.len() == 1 {					
					if *name_ == name || (*name_ == cell_name && cell_name != String::new()) {
						return Ok(i);
					} 					
				} else {
					let index_if: Vec<&str> = name.split('[').collect();
					if v[0].to_string() != index_if[0].to_string() { 
						/*println!("\n\n\n");
						println!("name: {:?}", index_if.clone());
						println!("name_: {:?}", v.clone());*/
						i -= 1; 
						continue; 
					}
					
					
					//let mut index_count: usize = 0;
					if index_if.len() > 1 { 
						let index_if: Vec<&str> = index_if[1].split(']').collect();
						println!("{:?}", index_if.clone());
						let mut cell: Sstring = Sstring::new();
						cell.from_string(index_if[0].clone().to_string());
						if index_if.len() > 1 {
							if !Words::eq_char_in_string('\"', &index_if[0].to_string()) { 
								//"
								search = match index_if[0].clone().parse() {
									Ok(A) => { A },
									Err(e) => { panic!("Index array error. Code 404_1"); 0 }
								};
								flag = true;
							} else {
								// структура
								// через цикл вывести имя поля, проверить есть ли ']' в конце
								// и вытащить номер
								// будем считать что на вход передано обращение без пробелов (то бишь без имя [10]
								// либо name ["asasas" ] ; struct1["name"]
								// память имена: имя.колво, имя_поля, имя_поля, имя_поля, ... , прочие_имена
								// память значения: "", "значение_1", "значение2", "значение3", ... , "прочие_значения"
								let len: usize = cell.len();
								if len < 2 { panic!("Cell in Struct error. Code 404_2"); }
								cell.remove(len-1);
								cell.remove(0);
								cell_name = cell.to_string();
								struct_count = match v[1].clone().trim().parse() { 
									Ok(A) => A,
									Err(e) => { panic!("memory array error. Code 201"); 0 },
								}; struct_count += 1;
							}
						}
					}									
				}
				if i == 0 {
					return Err(());
				}
				i -= 1;				
			} 
			Err(())
		}
		pub fn get_value_to_index(&self, index_: usize) -> Result<String , ()> {
			if index_ >= self.value_buffer.len() {
				return Err(());
			} Ok (self.value_buffer[index_].clone())
		}
		pub fn get_index_by_type(&self, mut name: String, types_: usize) -> Result<usize , ()>{
			//pub fn eq_char_in_string_r(ch: char, mut st: String, symbol_count: usize)->bool{ 
			if !Words::eq_char_in_string_r('\0', name.clone(), 1){
				name = name.clone().as_str().trim().to_string(); // проверяем на признак конца строки..
			} else { return Err(()); }	
			let mut j: usize = 0;		
			let mut search: usize = 0; 
			let mut flag: bool = false;
			let mut cell_name: String = String::new();
			for i in 0..self.object_buffer.len() {
				if search != 0 { 
					search -= 1;
					continue;
				}
				if flag == true {
					return Ok(j);
				}								
				let (name_, type_) = &self.object_buffer[i];
				let v: Vec<&str> = name_.split('.').collect();
				if v.len() == 1 {
					if *name_ == name || (*name_ == cell_name && cell_name != String::new()) {
						return Ok(j);
					} 
				} else {
					if v[0].to_string() != name { continue; }
					let index_if: Vec<&str> = name.split('[').collect();
					//let mut index_count: usize = 0;
					if index_if.len() > 1 { 
						let index_if: Vec<&str> = index_if[1].split(']').collect();
						let mut cell: Sstring = Sstring::new();
						cell.from_string(index_if[0].clone().to_string());
						if index_if.len() > 1 {
							if !Words::eq_char_in_string('\"', &index_if[0].to_string()) {
								//"
								search = match index_if[0].clone().parse() {
									Ok(A) => { A },
									Err(e) => { panic!("Index array error. Code 404_1"); 0 }
								};
								flag = true;
							} else {
								// структура
								// через цикл вывести имя поля, проверить есть ли ']' в конце
								// и вытащить номер
								// будем считать что на вход передано обращение без пробелов (то бишь без имя [10]
								// либо name ["asasas" ] ; struct1["name"]
								// память имена: имя.колво, имя_поля, имя_поля, имя_поля, ... , прочие_имена
								// память значения: "", "значение_1", "значение2", "значение3", ... , "прочие_значения"
								let len: usize = cell.len();
								if len < 2 { panic!("Cell in Struct error. Code 404_2"); }
								cell.remove(len);
								cell.remove(0);
								cell_name = cell.to_string();
								
							}
						}
					}					
				}
				if self.object_buffer[i].1.clone() == types_ { j += 1; }
			} Err(())

		}
		pub fn get_index_hight_data(&self, mut temp_name: String, mut temp_values: String) -> Result<usize, ()> {
			println!("temp_name: {:?}\ntemp_values: {:?}", temp_name.clone(), temp_values.clone());
			temp_name = temp_name.as_str().trim().to_string();
			let mut miss_step: usize = 0;
			let mut for_array: bool = false;
			if temp_values != "".to_string() {
				temp_values = temp_values.clone().trim().to_string();
				if Words::eq_char_in_string('\'' , &temp_values.clone()) || Words::eq_char_in_string('\"' , &temp_values.clone()) {
					let last_index: usize = temp_values.clone().chars().count() - 1;
					temp_values.remove(last_index);
					temp_values.remove(0);
				}
				// по структурам
				for i in 0..self.object_buffer.len() {
					if miss_step != 0 {
						miss_step -= 1;
						continue;
					}
					if for_array {
						let name_: String = self.object_buffer[i].0.clone();
						if temp_values == name_ {
							//panic!("");
							return Ok(i.clone());
						}
					}
					// разделить поиск, вначале по структурам (так как они чаще всего НЕ встречаются)
					// и находятся в глобальной области видимости
					// а потом по массивам
					let this_variable: Vec<&str> = self.object_buffer[i].0.as_str().split('.').collect();
					if this_variable.clone().len() > 1 {
						if temp_name.as_str() != this_variable[0].clone() {
							miss_step = match this_variable[1].to_string().parse() {
								Ok(A) => { A },
								Err(e) => { return Err(()); 0 },
							};
						} else if temp_name.as_str() == this_variable[0].clone() {
							for_array = true;
						}
					}
				}
				let mut i: usize = self.object_buffer.len() - 1;
				loop {
					//if for_array {
						let this_value_var: Vec<&str> = self.object_buffer[i].0.as_str().split('_').collect();
						let len__: usize = this_value_var.len() - 1;
						if this_value_var[len__] != "".to_string() 
								&& Words::is_digit(this_value_var[len__].to_string()) 
								&& self.object_buffer[i].1 == 25 
								&& temp_name.as_str() == this_value_var[0].clone() 
								&& temp_values == this_value_var[len__] {
							//if temp_values.as_str() == this_value_var[len__] {				
								return Ok(i.clone());
							//}
						} else {
							/*if this_value_var[0] == temp_values.as_str() {
								return Ok(i.clone());
							}*/
						}
					//}
					if i == 0 { break; }
					i -= 1;
				} 
			} else {
				// в случае если не передан никакой доп парамерт, то возвращаем
				// индекс начала массива
				let mut i: usize = self.object_buffer.len() - 1;
				loop {
					//if for_array {
						let this_value_var: Vec<&str> = self.object_buffer[i].0.as_str().split('_').collect();
						//let len__: usize = this_value_var.len() - 1;
						if temp_name.as_str() == this_value_var[0].clone() 
							&& self.object_buffer[i].1 != 25 {
							//if temp_values.as_str() == this_value_var[len__] {				
								return Ok(i.clone());
							//}
						} 
						let this_value_var: Vec<&str> = self.object_buffer[i].0.as_str().split('.').collect();
						//let len__: usize = this_value_var.len() - 1;
						if temp_name.as_str() == this_value_var[0].clone() 
							&& self.object_buffer[i].1 != 25 {
							//if temp_values.as_str() == this_value_var[len__] {		
								//panic!("");
								return Ok(i.clone());
							//}
						} 
					//}
					if i == 0 { break; }
					i -= 1;
				} 
			}			
			Err(())
		}
		pub fn get_value_from_name(&self, name: String) -> Result<String, ()> {
			let what_is_that: Vec<&str> = name.as_str().trim().split('[').collect();
			if what_is_that.len() > 1 {
				// для массивов/структур
				let value__: Vec<&str> = what_is_that[1].split(']').collect();
				let value__: String = value__.clone()[0].to_string();
				let name__: String = what_is_that[0].to_string();
				let indx_: usize = match self.get_index_hight_data(name__.clone(), value__.clone()){
					Ok(A) => { A },
					Err(e)=> { panic!("попытка присваивания несуществующего значения"); 0 },
				};
				return self.get_value_to_index(indx_);
			} else {
				let name__: String = what_is_that[0].to_string();
				let indx_: usize = match self.get_index(name__.clone()){
					Ok(A) => { A },
					Err(e)=> { panic!("попытка присваивания несуществующего значения"); 0 },
				};
				return self.get_value_to_index(indx_);
			}
			/*
			let u: usize = match self.get_index_hight_data(temp_name.clone(), temp_values.clone()){
				Ok(A) => { A },
				Err(e)=> { panic!("попытка присваивания несуществующего значения"); 0 },
			};
			let mut to_right_value: String = String::new();
			let mut to_to_right: Vec<&str> = temp_buffer.as_str().split('[').collect();
			if to_to_right.len() > 1 {
				//panic!("зашли");
				let value__: Vec<&str> = to_to_right[1].split(']').collect();
				let value__: String = value__.clone()[0].to_string();
				let name__: String = to_to_right[0].to_string();

				let indx_: usize = match self.get_index_hight_data(name__.clone(), value__.clone()){
					Ok(A) => { A },
					Err(e)=> { panic!("попытка присваивания несуществующего значения"); 0 },
				};
				to_right_value = match self.get_value_to_index(indx_) {
					Ok(A) => { A },
					Err(e)=> { panic!("неизвестная ошибка, код: 88214; обратитесь к pcPowerJG"); String::new() }
				};
			} else {
				to_right_value = temp_buffer.clone();
			}

			self.value_buffer[u.clone()] = to_right_value.clone();
			*/
			
			Err(())
		}
		// get value from name
		pub fn get_value(&self, mut name: String) -> Result<&str, ()> {
			//pub fn eq_char_in_string_r(ch: char, mut st: String, symbol_count: usize)->bool{ 
			if !Words::eq_char_in_string_r('\0', name.clone(), 1){
				name = name.clone().as_str().trim().to_string(); // проверяем на признак конца строки..
		 	} else { return Err(()); }			
			let mut search: usize = 0; 
			let mut flag: bool = false;
			let mut cell_name: String = String::new();
			for i in 0..self.object_buffer.len() {
				if search != 0 { 
					search -= 1;
					continue;
				}
				if flag == true {
					return Ok(self.value_buffer[i].as_str());
				}								
				let (name_, type_) = &self.object_buffer[i];
				let v: Vec<&str> = name_.split('.').collect();
				if v.len() == 1 {
					if *name_ == name || (*name_ == cell_name && cell_name != String::new()) {
						return Ok(self.value_buffer[i].as_str());
					} 
				} else {
					if v[0].to_string() != name { continue; }
					let index_if: Vec<&str> = name.split('[').collect();
					//let mut index_count: usize = 0;
					if index_if.len() > 1 { 
						let index_if: Vec<&str> = index_if[1].split(']').collect();
						let mut cell: Sstring = Sstring::new();
						cell.from_string(index_if[0].clone().to_string());
						if index_if.len() > 1 {
							if !Words::eq_char_in_string('\"', &index_if[0].to_string()) {
								//"
								search = match index_if[0].clone().parse() {
									Ok(A) => { A },
									Err(e) => { panic!("Index array error. Code 404_1"); 0 }
								};
								flag = true;
							} else {
								// структура
								// через цикл вывести имя поля, проверить есть ли ']' в конце
								// и вытащить номер
								// будем считать что на вход передано обращение без пробелов (то бишь без имя [10]
								// либо name ["asasas" ] ; struct1["name"]
								// память имена: имя.колво, имя_поля, имя_поля, имя_поля, ... , прочие_имена
								// память значения: "", "значение_1", "значение2", "значение3", ... , "прочие_значения"
								let len: usize = cell.len();
								if len < 2 { panic!("Cell in Struct error. Code 404_2"); }
								cell.remove(len - 1);
								cell.remove(0);
								//cell.insert(0, '[');
								cell_name = cell.to_string();
								
							}
						}
					}					
				}				
			} Err(())
		}      
		
		pub fn search_fn(&self, temp_name: String) -> Result<String, ()> {
			for i in 0..self.object_buffer.len().clone() {
				let temp_: Vec<&str> = self.object_buffer[i].0.as_str().trim().split('|').collect();
				if temp_[0].clone().to_string() == temp_name.clone() {
					return Ok(self.object_buffer[i].clone().0);
				}
			} Err(())
		}

		pub fn remove_some_objects(&mut self, name_args_in_f: Vec<&str>) {
			for i in 0..name_args_in_f.len() {
				let indx: usize = match self.get_index_hight_data(name_args_in_f[i].to_string(), "".to_string()){
					Ok(A) => { A },
					Err(e)=> { return; 0 },
				};
				self.value_buffer.remove(indx.clone());
				self.object_buffer.remove(indx.clone());
			}
		}

		pub fn get_(&mut self, text: String, mut call_to_fn: String) -> u8 {
						
			//let mut variables_name: Vec<String> = Vec::new();
			
			//let mut object_type_string_buffer: Vec<Vec<String>> = Vec::new();		// буффер для объектов
			//let mut flag: u8 = 0;
			//-----------------------------------------------------------------------------------------------------------------
			let mut temp_values: String = String::new();			//	ВРЕМЕННЫЕ ПЕРЕМЕННЫЕ
			let mut temp_name:	 String = String::new();			//	...
			let mut temp_buffer: String = String::new();			//	...
			//let mut temp_usize_value: usize = 0;
			let mut func_inactive: bool = true;
			//let mut this_row: usize = 0;			
			let mut call_func: bool = false;
			let mut fn_active: bool = false;
			let mut vars_to_func: String = String::new();
			let mut name_args_in_f: Vec<&str> = Vec::new();
			if call_to_fn != "".to_string() {
				call_func = true;
				// имя_функкции|передаваемый_параметр|..;имя_параметра_внутри_функции|..
				let parameter_row: Vec<&str> = call_to_fn.as_str().trim().split(';').collect();
				let vars_to_func_: Vec<&str> = parameter_row[0].split('|').collect();
				name_args_in_f = parameter_row[1].split('|').collect();
				for i in 1..vars_to_func_.len() { // потому что первый - имя функции
					let value_: String = match self.get_value_from_name(vars_to_func_[i].to_string()){
						Ok(A) => { A },
						Err(e)=> { panic!("передаваемое в функцию значение не существует"); String::new() },
					};
					//name_args_in_f[i - 1]
					/*
						object_buffer: Vec<(String, usize)>,
						value_buffer: Vec<String>, // значения 
					*/
					self.object_buffer.push((name_args_in_f[i - 1].clone().to_string(), 3));
					self.value_buffer.push(value_.clone());
					vars_to_func.push(',');
					vars_to_func += name_args_in_f[i - 1].clone();
				}				
				println!("---------------------------------------------
				\nself.object_buffer: {:?}\n self.value_buffer: {:?}", self.object_buffer.clone(), 
				self.value_buffer.clone());
				// работает и создаётся, теперь в рабочем цикле над делать

			}
			//let mut buffer_this_row: usize = 0;			

			let mut temp_weight_vec: Vec<f32> = Vec::new();			//	...
			

			let mut last_op: [usize; 3] = [0; 3];					//  ...
			//-----------------------------------------------------------------------------------------------------------------
			for ch in text.chars() {			
				//Split(input: String, ch: char) ДЛЯ КОСЯКОВ
				println!("ch - {:?}\n last_op - {:?}\ntemp_buffer - {:?}\ntemp_values - {:?}\ntemp_name - {:?}\nself.value_buffer.len() - {:?}\nself.value_buffer: {:?}\nself.object_buffer - {:?}", ch.clone(), last_op.clone(), temp_buffer.clone(), temp_values.clone(), temp_name.clone(), self.value_buffer.clone().len(), self.value_buffer.clone(), self.object_buffer.clone());
				if ch == ' ' || ch == '\t' {
					/*if temp_buffer != call_to_fn && call_func && !fn_active {
						last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
						temp_buffer = String::new();							
						temp_values = String::new();
						temp_name = String::new();
						continue;
					} */

					if last_op[0] == 0 && last_op[1] == 0 && last_op[2] == 0 {
						let action: usize = Words::get_action_lite(self.words.clone(), temp_buffer.clone());
						// println!("action - {:?}", action.clone());
						match action {
							1 => { // create
								last_op[0] = 1; last_op[1] = 0; last_op[2] = 0; 
                            },
							3 => { // object
								last_op[0] = 3; last_op[1] = 0; last_op[2] = 0; 
							},
                            2 => { // serv
                                last_op[0] = 2; last_op[1] = 0; last_op[2] = 0;                                
                            },
                            17 => { continue; },
                            22 => { // remove
                                last_op[0] = 22; last_op[1] = 0; last_op[2] = 0;  
                            },
                            21 => {
                                // print
                                last_op[0] = 21;
                            },
							24 => {
								// array
								last_op[0] = 24;
								last_op[1] = 0; last_op[2] = 0;
							},
							29 => { // struct
								last_op[0] = 29; last_op[1] = 0; last_op[2] = 0;
								//last_op[1] = 17;
								// не забудь прикрутить имя..
								// last_op[1] <- тип поля
								// last_op[2] <- длинна структуры
								//let mut temp_values - имя структуры
								//let mut temp_name   - значение по умолчанию
								//let mut temp_buffer - имя поля
								
								//object_buffer: Vec<(String, usize)>, // наименования объектов // (name, type) 
								// 0 - нейрон, 1 -  объект, 2 - сервер, 3 - массив, 4 - структура
								//value_buffer: Vec<String>, // значения 
							},
							33 => { // end
								if last_op[0] == 29 { // wtf?
									let insert_to: usize = self.value_buffer.len() - last_op[2];
									// pub fn insert(&mut self, index: usize, element: T)
									temp_values.push('.');
									temp_values += last_op[2].clone().to_string().as_str();
									self.object_buffer.insert(insert_to, (temp_values, 4));
									self.value_buffer.insert(insert_to, "".to_string());
									//last_op[1] = 17; // ставим в некуда
									
									last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
									temp_buffer = String::new();
									temp_weight_vec = Vec::new();
									temp_values = String::new();
									temp_name = String::new();
								}
							},							
							// 20 - fn, 34 - end_fn, 28 - void
							// to fn
							20 => {
								last_op[0] = 20;
								temp_buffer = String::new();
								temp_values = String::new();
								temp_name = String::new();
							},
							34 => {								
								last_op[0] = 34;
								temp_buffer = String::new();
								temp_values = String::new();
								temp_name = String::new();
							}, 
							28 => {
								last_op[0] = 28;
								temp_buffer = String::new();
								temp_values = String::new();
								temp_name = String::new();
							},
							// end to fn

							//words.push("struct".to_string()); //29 // создание структуры
							//words.push("end".to_string());//33//end operation
							_ => {
								
							},
						}
						if !func_inactive && action != 34 {
							//temp_buffer = String::new();
							last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
							temp_buffer = String::new();							
							temp_values = String::new();
							temp_name = String::new();
							continue;
						} else if !func_inactive && action == 34 {
							last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
							temp_buffer = String::new();
							temp_weight_vec = Vec::new();
							temp_values = String::new();
							temp_name = String::new();
						} else if fn_active && action == 34 {
							//println!("вышли");
							return 1;
						}
						temp_buffer = String::new();								
					}
                    /*if last_op[1] == 15 {  
                        // none
                    } else*/
					if last_op[0] == 3 && last_op[1] == 13 {
						temp_buffer.push(ch.clone());					
					} else if last_op[0] == 24 && last_op[1] == 0 {
						last_op[1] = 17;
					} else if last_op[0] == 24 && last_op[1] == 15 && last_op[2] == 0 {
						temp_values = temp_buffer.clone();
						last_op[2] = 17;
						temp_buffer = String::new();
					} else if last_op[0] == 29 && last_op[1] == 0 && last_op[2] == 0 {
						temp_values = temp_buffer.clone();
						last_op[1] = 17;
					} else if last_op[0] == 29 && last_op[1] == 17 {
						if last_op[2] == 0 && (temp_values == String::new() || 
							temp_values == "") {
								temp_values = temp_buffer.clone();
							}
						let t_em_p: usize = Words::get_action_lite(self.words.clone(), temp_buffer.clone());
						//println!("t_em_p -> {}", t_em_p.clone());
						if t_em_p == 33 {
						
							let insert_to: usize = self.value_buffer.len() - last_op[2];
							// pub fn insert(&mut self, index: usize, element: T)
							temp_values.push('.');
							temp_values += last_op[2].clone().to_string().as_str();
							self.object_buffer.insert(insert_to, (temp_values, 4));
							self.value_buffer.insert(insert_to, "".to_string());
							//last_op[1] = 17; // ставим в некуда
							
							last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
							temp_buffer = String::new();
							temp_weight_vec = Vec::new();
							temp_values = String::new();
							temp_name = String::new();
							
						} /*else if t_em_p == 17 { 
							внизу на всякий стоит. зря я парюсь
						} */else {
							last_op[1] = t_em_p.clone();
							drop(t_em_p);
							temp_buffer = String::new();
						}
						
					} else if last_op[0] == 20 && last_op[1] == 0 && last_op[2] == 0 {
						// 20 - fn, 34 - end_fn, 28 - void						
						last_op[1] = 17;
					} else if last_op[0] == 20 && last_op[1] == 17 && last_op[2] == 33 {
						// name_func | arg | arg1 | ... | argN
						// порядковый номер энтера '\n'

						let args_: Vec<&str> = temp_values.as_str().split(',').collect();

						if temp_name == call_to_fn {
							fn_active = true;
							continue;
						}

						for arg_ in args_ {
							temp_name.push('|');
							temp_name += arg_;
						}

						self.object_buffer.push((temp_name.clone(), 10));
						self.value_buffer.push(String::new());
						last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
						temp_buffer = String::new();
						temp_weight_vec = Vec::new();
						temp_values = String::new();
						temp_name = String::new();
						// а управлять через другую переменную, грубо говоря если 
						func_inactive = false;
						continue;
						//panic!("работает?!");
					} else { /*continue;*/ }					 
				} else if ch == '\n' {
					//this_row += 1;
					// код осуществляющий работу
					//					 	
					if call_func {
						let action: usize = Words::get_action_lite(self.words.clone(), temp_buffer.clone());
						if action == 34 && fn_active {
							//name_args_in_f: Vec<&str> // в нём добавленные объекты
							//println!("\n\nname_args_in_f.clone(): {:?}", name_args_in_f.clone());
							//panic!("asd");
							self.remove_some_objects(name_args_in_f.clone());							
							//println!("вышли");
							return 1;
						}
						if !fn_active { 
							if last_op[0] == 20 && last_op[1] == 17 && last_op[2] == 33 {
								// name_func | arg | arg1 | ... | argN
								// порядковый номер энтера '\n'
								
								let args_: Vec<&str> = temp_values.as_str().split(',').collect();
								let call_to_fn: Vec<&str> = call_to_fn.split(';').collect();
								let call_to_fn: Vec<&str> = call_to_fn[0].split('|').collect();
								let call_to_fn: String = call_to_fn[0].to_string();
								if temp_name == call_to_fn {									
									fn_active = true;
									println!("{:?}", temp_name.clone());
									//panic!("ннашли");
									last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
									temp_buffer = String::new();
									temp_weight_vec = Vec::new();
									temp_values = String::new();
									temp_name = String::new();
									continue;
								} else {
									last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
									temp_buffer = String::new();
									temp_weight_vec = Vec::new();
									temp_values = String::new();
									temp_name = String::new();
									continue;
								}
							} else {
								last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
								temp_buffer = String::new();
								temp_weight_vec = Vec::new();
								temp_values = String::new();
								temp_name = String::new();
								continue;
							}
						}
					}
					if !func_inactive {
						let action: usize = Words::get_action_lite(self.words.clone(), temp_buffer.clone());
						if action == 34 {
							func_inactive = true;
						}
						last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
						temp_buffer = String::new();
						temp_weight_vec = Vec::new();
						temp_values = String::new();
						temp_name = String::new();
					}
					//
					if last_op[0] == 2 && last_op[1] == 15 {
                        //println!("name {}", temp_name.clone());
                        self.object_buffer.push((temp_name.clone(), 2));
                        self.value_buffer.push(String::new());
                       
                        self.i_have_u(temp_buffer.clone(), temp_name.clone(), last_op.clone());

                        last_op[0] = 0;	last_op[1] = 0;	last_op[2] = 0;

                        temp_buffer = String::new();
						temp_weight_vec = Vec::new();
						temp_values = String::new();
						temp_name = String::new();		
                        last_op[0] = 0;	last_op[1] = 0;	last_op[2] = 0;
                    } else if last_op[0] == 2 && last_op[1] == 0 {                                                
                        // servers - Vec<TcpStream>
                        // ВЕРНИСЬ
                        //println!("name {}", temp_buffer.clone());
                        self.object_buffer.push((temp_buffer.clone(), 2));
                        self.value_buffer.push(String::new());


                        temp_buffer = String::new();
						temp_weight_vec = Vec::new();
						temp_values = String::new();
						temp_name = String::new();		
                        last_op[0] = 0;	last_op[1] = 0;	last_op[2] = 0;
                    } else if last_op[0] == 1 && last_op[1] == 0  {
						
                        //temp_buffer value name
                        
                        self.value_buffer.push(String::new());
						self.object_buffer.push((temp_buffer.clone(), 0));
                        self.neural_network.new_neyron_options(temp_buffer.clone(), temp_weight_vec.clone(), 0.000001);

                        last_op[0] = 0;	last_op[1] = 0;	last_op[2] = 0;
                        
                        temp_buffer = String::new();
						temp_weight_vec = Vec::new();
						temp_values = String::new();
						temp_name = String::new();		
						continue;
					} else if last_op[0] == 1 && last_op[1] != 0 {
                        last_op[0] = 0;	last_op[1] = 0;	last_op[2] = 0;
                        
                        temp_buffer = String::new();
						temp_weight_vec = Vec::new();
						temp_values = String::new();
						temp_name = String::new();		
						continue;
                    } else if last_op[0] == 3 && last_op[1] == 13 {
						self.value_buffer.push(String::new());// было: temp_buffer.clone()
						self.object_buffer.push((temp_values.clone(), 1));	// (name, type) // 0 - нейрон, 1 -  объект, 2 - сервер
						//  i_have_u(&mut self, mut temp_buffer: String, mut temp_values: String, last_op: [usize; 3])
						//println!("self.object_buffer: ", self.object_buffer.clone());
						last_op[0] = 17; last_op[1] = 15; last_op[2] = 0;
                        
                        
                        
                        self.i_have_u(temp_buffer.clone(), temp_values.clone(), last_op.clone());
                        //println!("{:?}\n{:?}\n{:?}", self.object_buffer, self.value_buffer.clone(), self.neural_network.debug()); // ДЛЯ КОСЯКОВ
                        temp_buffer = String::new();
						temp_weight_vec = Vec::new();
						temp_values = String::new();
						temp_name = String::new();		
						
						last_op[0] = 0;	last_op[1] = 0;	last_op[2] = 0;

					} else if last_op[0] == 3 && last_op[1] != 13 {
						self.value_buffer.push(String::new());
						self.object_buffer.push((temp_buffer.clone(), 1));

						temp_buffer = String::new();
						temp_weight_vec = Vec::new();
						temp_values = String::new();
						temp_name = String::new();	

						last_op[0] = 0;	last_op[1] = 0;	last_op[2] = 0;
					} else if last_op[0] == 17 && last_op[1] == 15 && last_op[2] == 0 {
                        //i_have_u(&mut self, mut temp_buffer: String, mut temp_values: String, last_op: [usize; 3]) {
                        self.i_have_u(temp_buffer.clone(), temp_values.clone(), last_op.clone());
                        
                        temp_buffer = String::new(); // ВЕРНИСЬ
                        temp_values = String::new();
                        last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
                    } else if last_op[0] == 22 && last_op[1] == 0 && last_op[2] == 0 {
                        // remove
						// переделать под массивы и структуры. сделать проверку типов, все дела..
						// может быть: remove struct["row"]
						// либо: remove array[0]
						// либо: remove object
						// надо всё учитывать
                        for i in 0..self.value_buffer.len() {
							let name_: Vec<&str> = self.object_buffer[i].0.trim().split('.').collect();
                            if name_[0] == temp_values {
                                self.value_buffer.remove(i);
                                self.object_buffer.remove(i);
                                break;
                            }
                        }
                        last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
                        temp_buffer = String::new();
						temp_weight_vec = Vec::new();
						temp_values = String::new();
						temp_name = String::new();
                    } else if last_op[0] == 21 && last_op[1] == 0 && last_op[2] == 0 {
                        { // print
                            let t: String = temp_buffer.as_str().trim().to_string();
							
							let a = match self.get_value(t) { 
								Ok(A) => { A },
								Err(e) => { "" },
							};
							println!("{}", a);
                            /*for i in 0..self.object_buffer.len(){
                                if t == self.object_buffer[i].0.as_str(){
                                    if self.object_buffer[i].1 != 0 {
                                        println!("{}", self.value_buffer[i]);
                                    } else {
                                        //get_neyron_name //self.neural_network
                                        let mut u_for_neyron: usize = 0;
                                        for k in 0..i {
                                            if self.object_buffer[k].1 == 0 {
                                                u_for_neyron += 1;
                                            }
                                        }
                                        /*if u_for_neyron != 0 {
                                            u_for_neyron -= 1;
                                        }*/
                                        println!("{}", self.neural_network.get_neyron_name(u_for_neyron));
                                    }
                                    break;
                                }
                            }*/
                        }
                        last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
                        temp_buffer = String::new();
						temp_weight_vec = Vec::new();
						temp_values = String::new();
						temp_name = String::new();
                        //return 0;
                    } else if last_op[0] == 24 && last_op[1] == 17 && last_op[2] == 0 {
						temp_buffer += ".0";
						self.object_buffer.push((temp_buffer, 24));
						self.value_buffer.push(String::new());
						
						
						last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
						temp_buffer = String::new();
						temp_weight_vec = Vec::new();
						temp_values = String::new();
						temp_name = String::new();
					} else if last_op[0] == 24 && last_op[1] == 15 && last_op[2] == 17 {
						// создаём массив
						let objs_in_array: Vec<&str> = temp_buffer.as_str().split(',').collect();
						
						temp_name += ".";
						temp_name += objs_in_array.len().to_string().as_str();

						self.object_buffer.push((temp_name.clone(), 24));
						self.value_buffer.push(String::new());
						
						let mut tch: bool = false;
						//let chrs = temp_name.clone().chars();
						let mut i: usize = 0;
						let mut deleted_chs: usize = 0;
						for char_ in temp_name.clone().chars() {
							if char_ == '.' {
								tch = true;
							}
							if tch {
								temp_name.remove(i.clone() - deleted_chs);
								deleted_chs += 1;
							}
							i += 1;
						}
						
						temp_name += "_";
						//println!("\n\n\n--------------\n\n\n{:?}\n\n\n\n\n", objs_in_array.clone());
						for i in 0..objs_in_array.len() {
							let temp_name_: String = temp_name.clone() + i.to_string().as_str();
							self.object_buffer.push((temp_name_, 25));
							self.value_buffer.push(objs_in_array[i].clone().to_string());
						}						
						last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
						temp_buffer = String::new();
						temp_weight_vec = Vec::new();
						temp_values = String::new();
						temp_name = String::new();
					} else if last_op[0] == 17 && last_op[1] == 11 && last_op[2] == 15 {
						// temp_name - имя переменной (структура/массив)
						// temp_values - имя поля/индекс массива
						// temp_buffer - новое значение
						//temp_name += "[";
						//temp_name += temp_values.clone().as_str();
						//temp_name += "]";
						//temp_name += "\0";
						let u: usize = match self.get_index_hight_data(temp_name.clone(), temp_values.clone()){
							Ok(A) => { A },
							Err(e)=> { panic!("попытка присваивания несуществующего значения"); 0 },
						};
						let mut to_right_value: String = String::new();
						let mut to_to_right: Vec<&str> = temp_buffer.as_str().split('[').collect();
						if to_to_right.len() > 1 {
							//panic!("зашли");
							let value__: Vec<&str> = to_to_right[1].split(']').collect();
							let value__: String = value__.clone()[0].to_string();
							let name__: String = to_to_right[0].to_string();

							let indx_: usize = match self.get_index_hight_data(name__.clone(), value__.clone()){
								Ok(A) => { A },
								Err(e)=> { panic!("попытка присваивания несуществующего значения"); 0 },
							};
							to_right_value = match self.get_value_to_index(indx_) {
								Ok(A) => { A },
								Err(e)=> { panic!("неизвестная ошибка, код: 88214; обратитесь к pcPowerJG"); String::new() }
							};
						} else {
							//to_right_value = temp_buffer.clone();
							let indx_: usize = match self.get_index_hight_data(temp_buffer.clone(), "".to_string()){
								Ok(A) => { A },
								Err(e)=> { 
								to_right_value = temp_buffer.clone();
								self.value_buffer[u.clone()] = to_right_value.clone();
								last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
									temp_buffer = String::new();
									temp_weight_vec = Vec::new();
									temp_values = String::new();
									temp_name = String::new();
								continue;
								0 },
							};
							to_right_value = match self.get_value_to_index(indx_) {
								Ok(A) => { A },
								Err(e)=> { panic!("неизвестная ошибка, код: 88214; обратитесь к pcPowerJG"); String::new() }
							};
						}

						self.value_buffer[u.clone()] = to_right_value.clone();
						//pub fn get_index_by_type(&self, mut name: String, types_: usize) -> Result<usize , ()>{

						last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
						temp_buffer = String::new();
						temp_weight_vec = Vec::new();
						temp_values = String::new();
						temp_name = String::new();
					} else if last_op[0] == 29 && last_op[1] != 17 { // variable
						last_op[2] += 1;
						self.object_buffer.push((temp_buffer.clone(), last_op[1]));
						self.value_buffer.push(temp_name.clone());
						last_op[1] = 17;
						temp_buffer = String::new();
						// last_op[1] <- тип поля
						// last_op[2] <- длинна структуры
						//let mut temp_values - имя структуры
						//let mut temp_name   - значение по умолчанию
						//let mut temp_buffer - имя поля
						
						//object_buffer: Vec<(String, usize)>, // наименования объектов // (name, type) 
						// 0 - нейрон, 1 -  объект, 2 - сервер, 3 - массив, 4 - структура
						//value_buffer: Vec<String>, // значения 
					} else if last_op[0] == 29 && last_op[1] == 17 {
						let t_em_p: usize = Words::get_action_lite(self.words.clone(), temp_buffer.clone());
						//println!("t_em_p -> {}", t_em_p.clone());
						if t_em_p == 33 {
						
							let insert_to: usize = self.value_buffer.len() - last_op[2];
							// pub fn insert(&mut self, index: usize, element: T)
							temp_values.push('.');
							temp_values += last_op[2].clone().to_string().as_str();
							self.object_buffer.insert(insert_to, (temp_values, 4));
							self.value_buffer.insert(insert_to, "".to_string());
							//last_op[1] = 17; // ставим в некуда
							
							last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
							temp_buffer = String::new();
							temp_weight_vec = Vec::new();
							temp_values = String::new();
							temp_name = String::new();
							
						}
					} else if last_op[0] == 20 && last_op[1] == 17 && last_op[2] == 33 {
						// name_func | arg | arg1 | ... | argN
						// порядковый номер энтера '\n'

						let args_: Vec<&str> = temp_values.as_str().split(',').collect();

						/*if temp_name == call_to_fn {
							fn_active = true;
							continue;
						}*/


						for arg_ in args_ {
							temp_name.push('|');
							temp_name += arg_;
						}

						self.object_buffer.push((temp_name.clone(), 10));
						self.value_buffer.push(String::new());
						last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
						temp_buffer = String::new();
						temp_weight_vec = Vec::new();
						temp_values = String::new();
						temp_name = String::new();
						// а управлять через другую переменную, грубо говоря если 
						func_inactive = false;
						continue;
						//panic!("работает?!");
					} else if last_op[0] == 35 && last_op[1] == 35 && last_op[2] == 35 {						
						let args_: Vec<&str> = temp_values.trim().split(',').collect();
						let mut run_row: String = String::new();
						//function|other|other;arg1|arg2
						run_row += temp_name.as_str();
						for i in 0..args_.len() {
							run_row.push('|');
							run_row += args_[i].clone();
						}
						run_row.push(';');
						let args_in_fn: String = match self.search_fn(temp_name.clone()){
							Ok(A) => { A },
							Err(e)=> { panic!("нет такой функции"); String::new() },
						};
						//println!("{:?}", args_in_fn);
						//panic!("stop");
						let args_: Vec<&str> = args_in_fn.trim().split('|').collect();
						for i in 1..args_.len() {
							run_row += args_[i].clone();
							run_row.push('|');
						}
						let len_: usize = run_row.clone().chars().count() - 1;
						run_row.remove(len_);
						println!("{:?}", run_row);
						//pub fn get_(&mut self, text: String, mut call_to_fn: String) -> u8
						if self.get_(text.clone(), run_row.clone()) != 1 {
							panic!("внутренняя ошибка вызова функции");
						}
						last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
						temp_buffer = String::new();
						temp_weight_vec = Vec::new();
						temp_values = String::new();
						temp_name = String::new();
						//panic!("stop");
					} else {
						//continue;
					}
					
					/*else if last_op[2] == 16 {
						last_op[2] = 0; continue;
					}*/
				} /*else if ch == ';' {
					last_op[2] = 16;
				}*/
				if last_op[0] == 24 && last_op[1] == 15 && last_op[2] == 0 {
						temp_values = temp_buffer.clone();
						last_op[2] = 17;
						temp_buffer = String::new();
				} else if last_op[0] == 17 && last_op[1] == 11 && last_op[2] == 15 {
					match ch {
						' ' => { temp_buffer.push(ch.clone()) },
						_ => {},
					}
				}
				/*if (last_op[2] == 15) && (ch == ' ' || ch == '\t') { 
					temp_buffer.push(ch.clone()); 
					continue;
				}*/
				let action: usize = Words::get_action_lite(self.words.clone(), temp_buffer.clone());  
				match action {					// self.object_buffer (name, type) // 0 - нейрон, 1 -  объект, 2 - сервер
					17 => { 
						if (ch == ' ' || ch == '\t' ) && (last_op[1] != 15 || last_op[1] != 11) { 
							// else {
								continue; 
							//}
						} // обработка происходит наверху. Тут на всякий случай стоит. 
						// if ch == '\n' { panic!("PARSING ERROR CODE 692."); } // ПРОТЕСТИРОВАТЬ
						if last_op[0] == 1 && last_op[1] == 0 && last_op[2] == 0 {
							//let action_char: usize = Words::get_action_lite(self.words.clone(), ch.to_string());
							match ch {
								'{' => {
								//new_neyron(&mut self, name: String, weight_count: usize, learn_speed: f32)
								/*
									words.push("{".to_string());//6
									words.push("}".to_string());//7
									words.push(">".to_string());//8
									words.push("<".to_string());//9	
									words.push("[".to_string());//10
									words.push("]".to_string());//11
									words.push("-".to_string());//12
									words.push(":".to_string());//13
									words.push(".".to_string());//14
									words.push("=".to_string());//15
									words.push(";".to_string());//16							
									words.push("_".to_string());//17
								*/							
									last_op[1] = 6;
									temp_name = temp_buffer.clone();
									temp_buffer = String::new();
									continue;
								}, 
								'[' => {
									last_op[1] = 10;
									temp_name = temp_buffer.clone();
									temp_buffer = String::new();
									continue;
								},
								_ => {
									temp_buffer.push(ch);// включает обработку на имя (после имени сразу может быть '{' и т.п.)
								},
							}
							//}						
						} else if last_op[0] == 1 && last_op[1] == 6 {
							match ch {
								'}' => {
									if temp_values != "".to_string() { 
										temp_weight_vec.push(Words::string_to_f32(temp_values.clone())); 
									}
									
									self.neural_network.new_neyron_options(temp_name.clone(), temp_weight_vec.clone(), 0.000001);
									self.value_buffer.push(String::new());
									self.object_buffer.push((temp_name.clone(), 0));	// (name, type) // 0 - нейрон, 1 - текстовый объект, 2 - числовой объект
									//last_op[1] = 101; last_op[2] = self.value_buffer.len() - 1; // добавление из вектора[101] (1) id вектора (2)

									//self.value_buffer = Vec::new();
									temp_weight_vec = Vec::new();
									temp_name = String::new();
									temp_values = String::new();
								},
								',' => {
									//println!("{:?}", Words::string_to_f32(temp_values.clone()));
									temp_weight_vec.push(Words::string_to_f32(temp_values.clone()));
									temp_values = String::new();
								},

								  _ => {
									temp_values.push(ch);
								},
							}
						} else if last_op[0] == 1 && last_op[1] == 10 {
							match ch {
								']' => {
									let size: usize = Words::string_to_usize(temp_values.clone());

									for i in 0..size {
										temp_weight_vec.push(0.0);
									}

									self.neural_network.new_neyron_options(temp_name.clone(), temp_weight_vec.clone(), 0.000001);
									self.value_buffer.push(String::new());
									self.object_buffer.push((temp_name.clone(), 0));	// (name, type) // 0 - нейрон, 1 -  объект, 2 - сервер
							
									temp_weight_vec = Vec::new();
									temp_values = String::new();
									temp_name = String::new();									
								},
								  _ => {
									temp_values.push(ch);
								},
							}
						} else if last_op[0] == 3 && last_op[1] == 0 && last_op[2] == 0 {//println!("добавили имя");
							let action_char: usize = Words::get_action_lite(self.words.clone(), ch.to_string());
							//println!("{} - action_char", action_char.clone());
							match action_char {
								15 => {//println!("добавили имя");
									//if last_op[1] != 13 {										
										last_op[1] = 13;
										//println!("добавили имя");
										temp_values = temp_buffer.clone();
										temp_buffer = String::new();
									//}
								},
								17 => {									
									temp_buffer.push(ch.clone());									
								},
								_  => { if last_op[1] == 13 { temp_buffer.push(ch.clone()); } },
							}						
                        } else if last_op[0] == 2 && last_op[1] == 0 {
                            match ch.clone(){
                                '=' => { 
                                    //println!("присваиаем серверу");
                                    temp_name = temp_buffer.clone(); 

                                    //println!("{}", temp_values.clone());
                                    temp_buffer = String::new();
                                    
                                    last_op[0] = 2; last_op[1] = 15;// ВЕРНИСЬ
                                    
                                }, 
							    _ => { temp_buffer.push(ch.clone()); },
                            } // если есть знак присваивания
                        } else if last_op[0] == 22 && last_op[1] == 0 && last_op[2] == 0 { 
                            //remove
                            temp_values.push(ch.clone());
                        } else if last_op[0] == 17 && last_op[1] == 15 && last_op[2] == 0 {
                            temp_buffer.push(ch.clone());
                        } else {
                            match ch.clone(){
                                '=' => { 
                                    //println!("зашли");
                                    
									if last_op[1] == 15 {
										temp_buffer.push(ch.clone());
									}

                                    if last_op[0] == 0 && last_op[1] == 0 && last_op[2] == 0 {
										temp_values = temp_buffer.clone();
										//println!("{}", temp_values.clone());
										temp_buffer = String::new();
                                        last_op[0] = 17; last_op[1] = 15;
                                    }
									if last_op[0] == 24 && last_op[1] != 15{
										last_op[1] = 15;
										temp_name = temp_buffer.clone();
										temp_values = String::new();
										temp_buffer = String::new();
									} else if last_op[0] == 17 && last_op[1] == 11 && last_op[2] == 0 {
										last_op[2] = 15;
									} //else if 
                                }, 
								'[' => {
									if last_op[0] == 0 {
										last_op[0] = 17;
										last_op[1] = 10;										
										temp_name = temp_buffer.clone();
										temp_buffer = String::new();
									} else if last_op[0] == 17 && last_op[1] == 11 && last_op[2] == 15 {
										temp_buffer.push(ch.clone());
									}
								},
								']' => {
									if last_op[0] == 17 && last_op[1] == 10 && last_op[2] == 0 {
										last_op[1] = 11;
										temp_values = temp_buffer.clone();
										temp_buffer = String::new();
									} else if last_op[0] == 17 && last_op[1] == 11 && last_op[2] == 15 {
										temp_buffer.push(ch.clone());
									}
								},
								'(' => {
									if last_op[0] == 20 && last_op[1] == 17 {
										last_op[2] = 17;
										//temp_values = temp_buffer.clone();
										temp_name = temp_buffer.clone().as_str().trim().to_string();
										temp_buffer = String::new();
										continue;
									} else if last_op[0] == 0 && last_op[1] == 0 && last_op[2] == 0 {
										//panic!("вызов функции");
										// 35 // служебное, только для вызова функции
										last_op[0] = 35;
										temp_name = temp_buffer.clone().as_str().trim().to_string();
										temp_buffer = String::new();
										continue;
									}																
									temp_buffer.push(ch.clone());
								},
								')' => {
									if last_op[0] == 20 && last_op[1] == 17 && last_op[2] == 17 {
										temp_values = temp_buffer.clone();
										last_op[2] = 33;
										continue;
									} else if last_op[0] == 35 {
										temp_values = temp_buffer.clone();
										last_op[1] = 35; 
										last_op[2] = 35;
										//panic!("в функции");
										continue;
									}									
									temp_buffer.push(ch.clone());
								},
							    _ => { 
									if ch == '\n' { } else {
										temp_buffer.push(ch.clone()); 
									}
								},
                            }
						}
						/*else if last_op[0] == 0 && last_op[1] == 0 && last_op[2] == 0 {
							action.push(ch);
						}*/
					},
					_  => { temp_buffer.push(ch.clone()); }
				}				
			}
			//println!("{:?}\n{:?}\n{:?}", self.object_buffer, self.value_buffer.clone(), self.neural_network.debug()); // ДЛЯ КОСЯКОВ
            0 // вывод 
		}

        pub fn i_have_u(&mut self, mut temp_buffer: String, mut temp_values: String, last_op: [usize; 3]) {
			let mut where_two_obj: bool = false;       
			
			// get_index_hight_data(имя, значение) // get_index_hight_data		
			let mut index_first_object: usize = 0;
			let struct_one_obj: Vec<&str> = temp_values.as_str().trim().split('[').collect(); // первая переменная
			if struct_one_obj.len() > 1 {
				let ar: Vec<&str> = struct_one_obj[1].trim().split(']').collect();
				index_first_object = match self.get_index_hight_data(
						struct_one_obj[0].to_string().clone(), 
						ar[0].to_string().clone()){
					Ok(A) => A,
					Err(e) => { panic!("first variable not found!!"); 0 },
				};
			} else {
				index_first_object = match self.get_index_hight_data(
						temp_values.clone(), 
						"".to_string()){
					Ok(A) => A,
					Err(e) => { panic!("first variable not found!!"); 0 },
				};
				println!("index_first_object: {}", index_first_object);
				//panic!("");
			}
			
			/*match self.get_index_r(temp_values.clone()){
				Ok(A) => A,
				Err(e) => { panic!("first variable not found!!"); 0 },
			};*/
			let mut index_second_object: usize = 0;
			let struct_two_obj: Vec<&str> = temp_buffer.as_str().trim().split('[').collect();
			println!("struct_two_obj: {:?}", struct_two_obj.clone());
			if struct_two_obj.len() > 1 {
				let ar: Vec<&str> = struct_two_obj[1].trim().split(']').collect();
				index_second_object = match self.get_index_hight_data(
						struct_two_obj[0].to_string().clone(), 
						ar[0].to_string().clone()){
					Ok(A) => { where_two_obj = true; A },
					Err(e) => { 0 },
				};
				println!("index_second_object: {}", index_second_object);
			} else {
				index_second_object = match self.get_index_hight_data(
						temp_buffer.clone(), 
						"".to_string()){
					Ok(A) => { where_two_obj = true; A },
					Err(e) => { 0 },
				};
				println!("index_second_object: {}", index_second_object);
				/*println!("index_first_object: {}", index_first_object);
				panic!("");*/
			}

			/*let mut index_second_object: usize = match self.get_index_r(temp_buffer.clone()){
				Ok(A) => { where_two_obj = true; A },
				Err(e) => { 

					

					0 
				},
			};*/
			
			let mut two_value_betwen_space: usize = 0;
			let mut temp_buffer_ = temp_buffer.clone().as_str().trim().to_string();
			
			if where_two_obj { // если объект всё же есть
				match self.object_buffer.clone()[index_first_object].1 {
					0 => { // neyron
						//index_first_object
						//index_second_object
						match self.object_buffer.clone()[index_second_object].1 { 
							0 => {                                             
								let mut index_first_object_neyron: usize = match self.get_index_by_type(temp_values.clone(), 0){
									Ok(A) => A,
									Err(e) => { panic!("net module error code: 701040"); 0 },
								};
								let mut index_second_object_neyron: usize = match self.get_index_by_type(temp_buffer.clone(), 0){
									Ok(A) => A,
									Err(e) => { panic!("net module error code: 701040"); 0 },
								};
								
								self.value_buffer[index_first_object.clone()] =
									self.neural_network.get_neyron_name(index_second_object_neyron);
								// neyron_from_string(&mut self, index: usize, st: String){
								self.neural_network.neyron_from_string(index_first_object_neyron.clone(),
									self.value_buffer[index_first_object.clone()].clone());
								self.value_buffer[index_first_object.clone()] = String::new();
							},
							1 => { // object
								let mut index_first_object_neyron: usize = match self.get_index_by_type(temp_values.clone(), 0){
									Ok(A) => A,
									Err(e) => { panic!("net module error code: 701040"); 0 },
								};
								self.neural_network.neyron_from_string(index_first_object_neyron.clone(),
									self.value_buffer[index_second_object.clone()].clone());
							},                                        
							_ => {  },
						}
					},
					1 => { // object                                    
						println!("self.object_buffer.clone()[index_second_object.clone()].1: {:?}", self.object_buffer.clone()[index_second_object.clone()].1.clone());
						match self.object_buffer.clone()[index_second_object.clone()].1 { // теперь роемся во втором объекте
							// определяем тип
							0 => { // тип второго объекта - нейрон
								//get_neyron_name
								//neural_network
								let mut index_second_object_neyron: usize = match self.get_index_by_type(temp_buffer_.clone(), 0){
									Ok(A) => A,
									Err(e) => { panic!("net module error code: 701040"); 0 },
								};
								/*for i in 0..self.object_buffer.len(){ // ищем index объекта в общей "куче" значений всех объектов
									if temp_buffer_.clone() == self.object_buffer.clone()[i.clone()].0 {
											break; // нашли, выходим из цикла
									}
									if temp_buffer_.clone() != self.object_buffer.clone()[i.clone()].0 &&
										self.object_buffer.clone()[i.clone()].1 == 0 {
										index_second_object_neyron += 1;
									}
								}*/

								self.value_buffer[index_first_object.clone()] =
									self.neural_network.get_neyron_name(index_second_object_neyron);
								//println!("values -> {:?}\n{:?}\nid {:?}", self.value_buffer.clone(), self.neural_network.debug(), index_second_object_neyron.clone());
								//println!("добавил");
							},                                        
							1 => { // тип второго объекта - объект 
								

								//println!("index_one {} index_two {}", index_first_object,
								//                    index_second_object.clone());
								let obj2 = self.value_buffer.clone()[index_second_object.clone()].clone();                                            
								self.value_buffer[index_first_object.clone()] = obj2;
								//println!("values -> {:?}", self.value_buffer.clone());
								//println!("добавил");
								// это заставляет код obj1 = obj2
								// где и то и другое типа object работать корректно
								// вначале ищем объекты, потом добавляем всё в кучу.
							}, 
							2 => { // server                                            
								// servers - Vec<TcpStream>
								
								//println!("index_one {} index_two {} serv_index 'un'", index_first_object.clone(),
								//                   index_second_object.clone());
								let obj2 = self.value_buffer.clone()[index_second_object.clone()].clone();                                            
								self.value_buffer[index_first_object.clone()] = obj2;
								//println!("values -> {:?}", self.value_buffer.clone());
								//println!("добавил");
							},
							24=> {
								let mut to_objs: usize = 0;
								let temp__: Vec<&str> = self.object_buffer[index_second_object.clone()].0.split('.').collect();
								println!("temp__: {:?}", temp__.clone());
								to_objs = match temp__[1].trim().parse() {
									Ok(A) => { A },
									Err(e)=> { panic!("вы не можете присвоить несуществующее значение"); 0 },
								};
								let mut temp__: String = String::new();
								if to_objs != 0 {
									for i in (index_second_object.clone()+1)..(index_second_object.clone()+to_objs+1) {
										temp__ += self.object_buffer.clone()[i].0.clone().as_str();
										temp__.push(',');
									}
								} else {
									temp__ = "".to_string();
								}
								self.value_buffer[index_first_object.clone()] = temp__.clone();
							},
							25=> {
								let obj2 = self.value_buffer.clone()[index_second_object.clone()].clone();                                            
								self.value_buffer[index_first_object.clone()] = obj2;
							},
							_ => { },// ошибки быть не может, ибо объект точно есть
						}
					},
					2 => {  // server
						match self.object_buffer.clone()[index_second_object.clone()].1 {
							1 => {
								//println!("index_one {} index_two {}", index_first_object.clone(),
								//                    index_second_object.clone());
								let obj2 = self.value_buffer.clone()[index_second_object.clone()].clone();                                            
								self.value_buffer[index_first_object.clone()] = obj2;
								//println!("values -> {:?}", self.value_buffer.clone());
								//println!("добавил");
								
							},
							2 => {
								//println!("index_one {} index_two {}", index_first_object.clone(),
								//                   index_second_object.clone());
								let obj2 = self.value_buffer.clone()[index_second_object.clone()].clone();                                            
								self.value_buffer[index_first_object.clone()] = obj2;
								//println!("values -> {:?}", self.value_buffer.clone());
								//println!("добавил");
							},
							_ => {  },
						}
					},
					24=> {
						match self.object_buffer.clone()[index_second_object.clone()].1 {
							1 | 3 => {
								//println!("index_one {} index_two {}", index_first_object.clone(),
								//                    index_second_object.clone());
								let obj2 = self.value_buffer.clone()[index_second_object.clone()].clone();                                            
								//self.value_buffer[index_first_object.clone()] = obj2;
								let names_: Vec<&str> = obj2.as_str().trim().split(',').collect();
								/*let name_: Vec<&str> = names_[0].split('_').collect();
								let mut name_: String = name_[0].to_string();
								name_.push('.');
								name_ += names_.len().to_string().as_str();*/
								{
									let ar_len: Vec<&str> = self.object_buffer[index_first_object.clone()].0
										.as_str().trim().split('.').collect();
									let ar_len: usize = match ar_len[1].clone().parse() {
										Ok(A) => { A },
										Err(e)=> { panic!("невозможная ошибка"); 0 },
									};
									if ar_len != 0 {
										for i in (index_first_object+1)..(index_first_object+ar_len+1) {
											//if i >= self.object_buffer.len() { break; }
											println!("удаляем {:?}", self.object_buffer[index_first_object+1].clone());
											self.object_buffer.remove(index_first_object+1);
											self.value_buffer.remove(index_first_object+1);
										}
									}
									/*println!("\n\nvalues -> {:?}", self.value_buffer.clone());
									println!("names -> {:?}\n\n", self.object_buffer.clone());*/
								}
								let mut add_: usize = 0;
								let mut i: usize = names_.len() - 1;
								loop {
									println!("пытаемся добавить: {:?}", names_[i].to_string().clone());
									if names_[i].to_string().clone() == "".to_string() { 	
										if i == 0 { break; }
										i -= 1;
										continue;
									}
									if (index_first_object+1) < self.object_buffer.len() {
										self.object_buffer.insert(index_first_object+1, (names_[i].to_string().clone(), 24));
										self.value_buffer.insert(index_first_object+1, names_[i].to_string().clone());
										add_ += 1;
									} else if (index_first_object+1) >= self.object_buffer.len() {
										self.object_buffer.push((names_[i].to_string().clone(), 25));
										self.value_buffer.push(names_[i].to_string().clone());
										add_ += 1;
									}									
									if i == 0 { break; }
									i -= 1;
								}
								let name_: String = self.object_buffer[index_first_object].0.clone();
								let name_: Vec<&str> = name_.as_str().trim().split('.').collect();
								let mut name_: String = name_[0].clone().to_string();
								name_.push('.');
								name_ += add_.to_string().as_str();
								self.object_buffer[index_first_object].0 = name_.clone();
								/*println!("len -> {}", self.value_buffer.len());
								println!("values -> {:?}", self.value_buffer.clone());
								println!("names -> {:?}", self.object_buffer.clone());
								//println!("добавил");
								panic!("");*/
							}, 
							_ => {  },
						}
					},
					_ => {  },
				}
			} else { 
				// если объект всё же не найден
				//println!("error404 in {}", index_first_object.clone());
				match self.object_buffer.clone()[index_first_object].1 {
					0 => {  // neyron
						let mut index_first_object_neyron: usize = 0;
						for i in 0..index_first_object.clone(){
							if self.object_buffer.clone()[i.clone()].1 == 0 {
								index_first_object_neyron += 1;
							}
						}  
						//println!("index->{}",index_first_object_neyron.clone());
						self.neural_network.neyron_from_string(index_first_object_neyron.clone(),
							temp_buffer.clone());
					},
					1 => {
						self.value_buffer[index_first_object.clone()] = temp_buffer;
					},
					2 => {
						self.value_buffer[index_first_object.clone()] = temp_buffer.as_str().trim().to_string();
					},
					_ => {},
				}
			}
        }
	pub fn unsafe_funtion_memory_add(&mut self, name: String, value: String, type_: usize){
		//object_buffer: Vec<(String, usize)>,              // наименования объектов 	
		// (name, type) // 0 - нейрон, 1 -  объект, 2 - сервер, 3 - массив, 4 - структура, 5 - array
	        //value_buffer: Vec<String>,                        // значения 
		self.object_buffer.push((name, type_));
		self.value_buffer.push(value);
	}
	pub fn unsafe_print_bufs_and_vals(&self){
		for i in 0..self.object_buffer.clone().len(){
			println!("variable: {}\nvalue: {}",self.object_buffer[i].clone().0, self.value_buffer[i].clone());
		}		
		self.neural_network.debug();
	}

        //---------------------------------------------------------------
        // all_row - полная строка со всеми математическими вычеслениями
        // math_type - тип математики (текстовый (+ и -) (0), 
        // процедурный (где 2+2*2 будет 8, а не 6) (1)
        // и стандартный (где будет 6 и со скобочками всякими) (2)
        // по умолчанию всегда 2
        //---------------------------------------------------------------
        // в нейронах плюсуются веса, сумма нейронов - сумма всех их весов, 
        // в объектах плюсуются значения
        // в серверах плюсуются порты
        //---------------------------------------------------------------
        // в потоках плюсуются полностью все объекты и открывается новый поток
        // с новыми значениями, если поток + 0, то просто копируется поток
        //---------------------------------------------------------------
        // сложение массивов - добавление к одному массиву все элементы другого
        // вычитание: если этот элемент под этим индексом есть - вычитаем
        //---------------------------------------------------------------
        // сложение строк - добавление к одной строке элементов другой
        // вычитание строк - удаление из одной строки всех символов другой 
        // деление строк - поиск индекса вхождения (где в строке 1 полностью втречается строка 2), удаление символов строки 2
        // и создание массива 
        // умножение строк - создание массива строк
        //---------------------------------------------------------------
        // деление на 0 даст Inf
        // деление на Inf даст 0
        // умножение на 0 даст 0
        //---------------------------------------------------------------
        // 
        pub fn math_(&mut self, all_row: String, math_type: u8) -> String{
			// if f64 ret_ex(&str)
			// if string 
			// внутри массива элементы разделяются через '\n'
			match math_type {
				1 => { // числовой тип
					return ret_ex(all_row.as_str());
				},
				2 => { 

				},
				_ => {  

				},

			}
            String::new()
        }

		pub fn string_to_usize(word: String) -> usize {
			let mut result: usize = 0;
			for ch in word.chars(){
				match ch {
					'0'=>{}, 
					'1'=>{ result += 1; },
					'2'=>{ result += 2; },
					'3'=>{ result += 3; },
					'4'=>{ result += 4; },
					'5'=>{ result += 5; }, 
					'6'=>{ result += 6; }, 
					'7'=>{ result += 7; }, 
					'8'=>{ result += 8; },
					'9'=>{ result += 9; },  
					_ =>{ return 0; },
				} result = result * 10;
			} result / 10
		}
		pub fn string_to_f32(word: String)->f32{
			let pie: f32 = match word.parse(){
				Ok(A)=>{ A },
				Err(e)=> { 0.0 },
			}; pie
		}
		pub fn get_action_lite(words: Vec<String>, word: String)->usize{
			let mut index: usize = 0;
			//let word_chs: Vec<char> = Words::to_vec(word);
			for word_ in words {
				if word == word_ {
					return index + 1;
				}
				index += 1;
			}
			17
		}
		pub fn to_vec(word: String)->Vec<char>{			
			let mut result: Vec<char> = Vec::new();
			for ch in word.chars(){
				result.push(ch);
			} result
		}
		pub fn get_action(words: Vec<String>, word: String)->usize{ //слово -> действие 
			
			let mut buffer: Vec<(usize, usize)> = Vec::new();//индекс, колво совпадений
			//let mut temp: String = String::new();

			
			
			let t = Words::to_vec(word.clone());			
			for i in 0..t.len(){				;
				for k in 0..words.clone().len() {
				//println!("Цикл 2");
					//if t[i] == ' ' { temp = String::new(); continue; }
					//temp.push(t[i]);
					//for k in i.clone()..t.clone().chars().len(){
					if k < words.len(){
					//println!("Условие 1");
					let temp: Vec<char> = Words::to_vec(words[k].clone());
						if temp.len() > i {						
							if temp[i] == t[i] {
							let mut for_for: bool = false;
								for j in 0..buffer.len(){								
									if buffer[j].0 == k {
										buffer[j].1 += 1;
										for_for = true;
										break;
									}
								}
								if for_for == false {
									buffer.push((k, 1));
								}
								
							}
						}
					}
					//}
				}				
			} 
			
			let mut index: usize = 16;//всякая дич
			let mut max: usize = 0;
			for i in 0..buffer.len(){
				if buffer[i].1 > max {
					max = buffer[i].1;
					index = buffer[i].0;
				}
			}
			
			if index < words.len() && (word.chars().count() > words[index].chars().count()){
				return 17;
			} else {
				//println!("index: {:?}", index.clone());
				let mut right: usize = 0;
				
				let temp: Vec<char> = Words::to_vec(word.clone());
				let temp1: Vec<char>= Words::to_vec(words[index].clone());
				//println!("temp: {:?}\ntemp1: {:?}", temp.clone(), temp1.clone());
				for i in 0..temp1.len(){
					
					if ((i < temp.len())&&(i < temp1.len())) && temp[i] == temp1[i]{
						right += 1;
					}
				}
				if right == temp.len(){
					return index + 1;
				}
			}			
			17
		}		
		fn Split(input: String, ch: char)->Vec<String>{
			let mut temp: String = String::new();
			let mut result: Vec<String> = Vec::new();
			
			for item in input.chars(){
				if item == ch{
					result.push(temp);
					temp = String::new();
				} else {
					temp.push(item);
				}
			} result
		}
	}
	
	pub struct Net{
		data_base: Vec<Neywork>,
		map_step: Vec<(usize, usize, usize)>,// нейрон1 - нейрон2 - вход
	}
	pub struct Neywork{
		weight: Vec<f32>,
		inputs: Vec<f32>,
		learn_speed: f32,

		result: f32,

		name: String,
	}
	pub fn new()->Net{
		Net { data_base: Vec::new(), map_step: Vec::new() }
	}

	impl Neywork{
		pub fn proceed(&mut self){
			let mut r: f32 = 0.0;

			for i in 0..self.inputs.len(){
				r += (self.inputs[i] * self.weight[i]) + self.learn_speed;
			}
			self.result = r;
		}
		pub fn error(&mut self, true_result: f32){
			let delta: f32 = true_result - self.result;
			for i in 0..self.inputs.len(){
				self.weight[i] = self.weight[i] + (self.inputs[i] * delta * self.learn_speed);				
			}
		}
		pub fn get_name(&self)->String{ self.name.clone() }

		pub fn debug(&self) -> (String, Vec<f32>, f32) { (self.name.clone(), self.weight.clone(), self.learn_speed.clone() ) }
        pub fn get_all_width(&self) -> String { 
            let mut result: String = "{".to_string();            
            for item in &self.weight { 
                result += item.to_string().as_str();
                result.push(',');
            }
			if result.clone().chars().count() > 1 { 
				result.remove(result.clone().chars().count() - 1);
			}
            result.push('}'); result.push('\0'); result
        }
	}
	impl Net{
		pub fn debug(&self){ for item in &self.data_base { println!(" neyron -> {:?}", item.debug()); } }
		pub fn neyron_from_string(&mut self, index: usize, mut st: String){
		    let ch_len: usize = st.clone().chars().count();
		    if ch_len == 0 { panic!("null string. error code 12001"); }
		    st.remove(ch_len - 1);
		    let mut v: Vec<&str> = st.as_str().split(',').collect();
		    let len = v.clone().len();
		    
		    let mut b1 = false;
		    let mut b2 = false;
		    
		    let len = v.len();
		    for i in 0..v.len(){
			for word in v[i].to_string().chars() {
			    if word == '{' { b1 = true; }
		    		
			     
			    if word == '}' { b2 = true; }
			}
		    }
		    
		    if (b1 && b2) == false { return; }
		    //println!("v -> {:?}", v.clone());
		    self.data_base[index.clone()].weight = Vec::new();
		    self.data_base[index.clone()].inputs = Vec::new();
		    for word in v {
		        if word == "}" || word == "{" || word == "" { continue; }          
		        let word = word.to_string();
		        let pie: f32 = match Net::Trim('{', &Net::Trim('}', &word.to_string())).as_str().trim().parse(){
					    Ok(A)=>{ A },
					    Err(e)=> { panic!("incorrect value! err code 1"); 0.0 },
				    };
		        self.data_base[index.clone()].weight.push(pie);
		        self.data_base[index.clone()].inputs.push(0.0);
		    }

		}
		pub fn Trim(ch: char, st: &String)->String{
			let mut result: String = String::new();
			for char_ in st.chars() {
				if char_ != ch { result.push(char_); }
			} result
		}
		pub fn get_neyron_name(&self, id: usize)->String { 
		    //println!(" id -> {} in {}", id.clone(), self.data_base.len());
		    if id.clone() < self.data_base.len() {
		         self.data_base[id.clone()].get_all_width()
		    } else { "NONE".to_string() } 
		}        
			pub fn new_neyron(&mut self, name: String, weight_count: usize, learn_speed: f32)->bool{
				let mut t1: Vec<f32> = Vec::new();
				for i in 0..weight_count{
					t1.push(0.0);
				}			
				let temp: Neywork = 
					Neywork{ weight: t1.clone(), inputs: t1.clone(), learn_speed: learn_speed, result: 0.0, name: name };
				self.data_base.push(temp);
				true
			}
			pub fn new_neyron_options(&mut self, name: String, weight: Vec<f32>, learn_speed: f32)->bool{
				
				let mut t1: Vec<f32> = Vec::new();
				for i in 0..weight.clone().len(){
					t1.push(weight.clone()[i]);
				}
				let temp: Neywork = 
					Neywork{ weight: weight.clone(), inputs: t1.clone(), learn_speed: learn_speed, result: 0.0, name: name };
				self.data_base.push(temp);

				true
			}
			pub fn remove_neyron(&mut self, index: usize){
				self.data_base.remove(index);
			}
			pub fn add_step(&mut self, neyron_output: usize, neyron_to: usize, neyron_to_inputID: usize){
				self.map_step.push((neyron_output, neyron_to, neyron_to_inputID));
			}
			pub fn remove_step(&mut self, index: usize){
				self.map_step.remove(index);
			}
			pub fn len(&self)->usize{ self.data_base.len() }
		pub fn get_neyron_to_index(&self, index: u8){}
	}
	// <MATH> CODE
	fn to_char_arr(input: &str)->Vec<String>{    
		let input = input.to_string();
		let mut result: Vec<String> = Vec::new();
		for ch in input.chars(){
			result.push(ch.to_string());
		} result
	}	

	fn ret_ex(ex: &str) -> String{
		let vec: Vec<String> = to_char_arr(ex);
		let mut re = Math { k: 0, result: 0.0 };
		re.expr(&vec);
		re.result.to_string()
	}
	struct Math{
		k: usize,
		result: f64,
	}
	impl Math {
		pub fn expr(&mut self, ex: &Vec<String>){
			//let mut indx: usize = self.k.clone();
			self.term(ex);
			let mut result = self.result.clone();
			while
				self.k < ex.len() && match ex[self.k.clone()].as_str() {
					"+" => {
						self.k += 1;
						self.term(ex);
						result += self.result;
						true
					},
					"-" => {
						self.k += 1;
						self.term(ex);
						result -= self.result;
						true
					},
					_ => { false },
				}
			{ } self.result = result;        
		}

		pub fn term(&mut self, ex: &Vec<String>){
			self.result = self.factor(&*ex);
			let mut result = self.result.clone();
			//let mut div: f64 = 0.0;
			while
				self.k < ex.len() && match ex[self.k.clone()].as_str(){
					"*"=>{ 
						//i += 1;
						self.k += 1;
						result *= self.factor(&*ex);
						true 
					},
					"/"=>{ 
						//i += 1;
						self.k += 1;
						let div: f64 = self.factor(&*ex);
						if div != 0.0 {
							result /= div;
						} else { self.result = 0.0; return; }
						true 
					},
					_=>{ false },
				}
			{ } self.result = result;
		}

		fn factor(&mut self, ex: &Vec<String>)->f64{ 
			//let result: f64 = self.result.clone();
			let mut result2: f64 = 0.0;
			let mut sign: f64 = 1.0;
			if ex[self.k.clone()].as_str() == "-" { sign = -1.0; }
			if ex[self.k.clone()].as_str() == "("{
				self.k += 1;
				self.expr(ex); 
				result2 = self.result.clone();
				//self.result = result;
				self.k += 1;
				//self.expr(ex);
			} else {
				result2 = self.number(ex);            
				//result2 = self.result.clone();
				//self.result = result;
			}
			result2 *= sign;
			result2.clone()         
		}
		pub fn number(&mut self, ex: &Vec<String>)->f64{
			let mut result: f64 = 0.0;
			let mut div: f64 = 0.0;
			let mut sign: f64 = 1.0;

			if ex[self.k.clone()].as_str() == "-" {
				sign = -1.0;
				self.k += 1;
			}
			while 
				match ex[self.k.clone()].as_str(){
					"0" => { 
						//result = result * 10.0 + (str[*idx] - '0');
						// прибавляем
						//++*idx;            
						result = result * 10.0 + 0.0;
						true
					},
					"1" => { 
						result = result * 10.0 + 1.0;
						true
					},
					"2" => { 
						result = result * 10.0 + 2.0;
						true
					},
					"3" => { 
						result = result * 10.0 + 3.0;
						true
					},
					"4" => { 
						result = result * 10.0 + 4.0;
						true
					},
					"5" => { 
						result = result * 10.0 + 5.0;
						true
					},
					"6" => { 
						result = result * 10.0 + 6.0;
						true
					},
					"7" => { 
						result = result * 10.0 + 7.0;
						true
					},
					"8" => { 
						result = result * 10.0 + 8.0;
						true
					},
					"9" => { 
						result = result * 10.0 + 9.0;
						true
					},                
					_ => { false },
				}
			{ self.k += 1; if self.k >= ex.len() { break; } }        
			if self.k.clone() < ex.len() {            
				if ex[self.k.clone()].as_str() == "." {
				self.k += 1;            
				while
					match ex[self.k.clone()].as_str(){
						"0" => { 
							//result = result * 10.0 + (str[*idx] - '0');
							// прибавляем
							//++*idx;            
							result = result + 0.0 / div;
							true
						},
						"1" => { 
							result = result + 1.0 / div;
							true
						},
						"2" => { 
							result = result + 2.0 / div;
							true
						},
						"3" => { 
							result = result + 3.0 / div;
							true
						},
						"4" => { 
							result = result + 4.0 / div;
							true
						},
						"5" => { 
							result = result + 5.0 / div;
							true
						},
						"6" => { 
							result = result + 6.0 / div;
							true
						},
						"7" => { 
							result = result + 7.0 / div;
							true
						},
						"8" => { 
							result = result + 8.0 / div;
							true
						},
						"9" => { 
							result = result + 9.0 / div;
							true
						},
						_ => { false },
					}
				{ self.k += 1; div *= 10.0; } }
			}
			sign * result
		}
	}
	//</MATH> CODE END
	
	
	
}