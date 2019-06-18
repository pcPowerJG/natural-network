use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::str;

use std::net::TcpStream;

extern crate libc;
use libc::size_t;

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
	extern crate libc;
	use libc::size_t;
	use std::ffi::CString;
	use std::ptr;

	#[link(name = "math")]
	extern {
		fn eval(text: *const libc::c_char)->f32;
	}

	#[link(name="to_extern")]
	extern {
		// int linked(char * lib_path, char * lib_name){
		fn linked(lib_path: *const libc::c_char, func_name: *const libc::c_char) -> i32;
		//void *open(char * lib_path) {
		fn open(lib_path: *const libc::c_char) -> *const libc::c_void;
		//
		fn close(lib: *const libc::c_void) -> i32;
		//int int_rf_func(void* handle, char * func_name, int arg){
		fn int_rf_func(handle: *const libc::c_void, func_name: *const libc::c_char, arg: libc::c_int) -> i32;
	}
	
	// endpointautomate
	use std::net::*;
	pub struct Words{
			words: Vec<String>,                               // буква (номер от 1 (a-z)), слово
        	neural_network: Net,        			  		  // сама сеть
			servers: Vec<ServersModule::Thread>,              // сервера
			//buffer_action: Vec<[usize; 3]>,                 // буффер для действий
			object_buffer: Vec<(String, usize)>,              // наименования объектов 	// (name, type) // 0 - нейрон, 1 -  объект, 2 - сервер, 3 - массив, 4 - структура, 10 - функция
	        value_buffer: Vec<String>,                        // значения 
			import_handle: Vec<*const libc::c_void>,
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
		words.push("eq".to_string()); //  36
		words.push("!eq".to_string());//  37
		words.push(">".to_string());  //  38
		words.push("<".to_string());  //  39
		words.push("loop".to_string());// 40
		words.push("end_loop".to_string());// 41
		words.push("break".to_string()); // 42
		// 4 - if, 33 - end, 36 - eq, 37 - !eq, 38 - >, 39 - <
		Words{ 
			words: words,  
			neural_network: new(), 
			servers: Vec::new(), 
			object_buffer: Vec::new(), 
			value_buffer: Vec::new(),
			import_handle: Vec::new(),
		}
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
						//println!("{:?}", index_if.clone());
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
						//println!("{:?}", index_if.clone());
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
			//println!("temp_name: {:?}\ntemp_values: {:?}", temp_name.clone(), temp_values.clone());
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
					Err(e)=> { return Err(()); 0 },
				};
				return self.get_value_to_index(indx_);
			} else {
				let name__: String = what_is_that[0].to_string();
				let indx_: usize = match self.get_index(name__.clone()){
					Ok(A) => { A },
					Err(e)=> { return Err(()); 0 },
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
			let mut i: usize = self.object_buffer.len().clone() - 1;
			loop {
				let temp_: Vec<&str> = self.object_buffer[i].0.as_str().trim().split('|').collect();
				if temp_[0].clone().to_string() == temp_name.clone() {
					return Ok(self.object_buffer[i].clone().0);
				}
				if i == 0 { break; }
				i -= 1;
			} Err(())
		}
		pub fn search_fn_index(&self, temp_name: String) -> Result<usize, ()> {
			for i in 0..self.object_buffer.len().clone() {
				let temp_: Vec<&str> = self.object_buffer[i].0.as_str().trim().split('|').collect();
				if temp_[0].clone().to_string() == temp_name.clone() {
					return Ok(i.clone());
				}
			} Err(())
		}

		pub fn delete_all_fn(&mut self){
			let mut i: usize = 0;
			while i < self.object_buffer.len().clone() {
				let temp_: Vec<&str> = self.object_buffer[i].0.as_str().trim().split('|').collect();
				if temp_.len() > 1{
					self.object_buffer.remove(i.clone());
					self.value_buffer.remove(i.clone());
					i = 0;
				}
				i += 1;				
			}
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
		pub fn to_bool(&self, left: String, operator: String, right: String) -> bool {
			//println!("\n\nleft: {:?}\noperator: {:?}\nright: {:?}", left.clone(), operator.clone(), right.clone());
			match operator.as_str() {
				"!eq" | "!=" => {
					if self.get_value_from_name(left.clone()).unwrap() != self.get_value_from_name(right.clone()).unwrap() {
						return true;
					} else {
						let left_: f32 = self.get_value_from_name(left.clone()).unwrap().parse().unwrap();
						let right_: f32 = self.get_value_from_name(right.clone()).unwrap().parse().unwrap();
						if left_ != right_ {
							return true;
						}
					}
				},
				"eq" | "==" => {
					if self.get_value_from_name(left.clone()).unwrap() == self.get_value_from_name(right.clone()).unwrap() {
						return true;
					} else {
						let left_: f32 = self.get_value_from_name(left.clone()).unwrap().parse().unwrap();
						let right_: f32 = self.get_value_from_name(right.clone()).unwrap().parse().unwrap();
						if left_ == right_ {
							return true;
						}
					}
				}, 
				">" => {
					let left_: f32 = match self.get_value_from_name(left.clone()) {
						Ok(A) => { 
							let f: f32 = match A.parse(){
								Ok(B) => { B },
								Err(e)=> { panic!("нельзя сравнивать текстовые значения"); 0.0 },
							}; 
							f
						},
						Err(e)=>{
							let left = Words::trim(left.clone(), " \t");
							let f: f32 = match left.parse(){
								Ok(C) => { C },
								Err(e)=> { panic!("нельзя сравнивать текстовые значения"); 0.0 },
							};
							f
						},
					};
					let right_: f32 = match self.get_value_from_name(right.clone()){
						Ok(A) => { 
							let f: f32 = match A.parse(){
								Ok(B) => { B },
								Err(e)=> { panic!("нельзя сравнивать текстовые значения"); 0.0 },
							}; 
							f
						},
						Err(e)=>{
							let right = Words::trim(right.clone(), " \t");
							let f: f32 = match right.parse(){
								Ok(C) => { C },
								Err(e)=> { panic!("нельзя сравнивать текстовые значения"); 0.0 },
							};
							f
						},
					};
					if left_ > right_ {
						//println!("return true");
						return true;
					}
				},
				"<" => {
					let left_: f32 = self.get_value_from_name(left.clone()).unwrap().parse().unwrap();
					let right_: f32 = self.get_value_from_name(right.clone()).unwrap().parse().unwrap();
					if left_ < right_ {
						return true;
					}
				},
				_ => { panic!("неопознанный оператор в выражение if"); },
			} false			  
		}
		pub fn get_all_func(&self, text_programm: String, mut func_need: String) -> String {
			// значит буду передавать в тексте цикла ещё и функции, просто потому что могу			
			let mut result_row: String = String::new();
			if func_need != "".to_string() && func_need != "_".to_string() {
				let to_space: Vec<&str> = text_programm.as_str().split('\n').collect();
				func_need = Words::trim(func_need.clone(), " \t");
				let funks: Vec<&str> = func_need.as_str().split(',').collect();
				let mut i: usize = 0;
				let mut temp_bool: bool = false;
				//println!("spaces: {:?}", to_space.clone());
				//panic!("");
				for space in to_space {	
					let end_f: Vec<&str> = space.clone().trim().split(' ').collect();	
					//println!("end_f: {:?}", end_f.clone());			
					for end_ in end_f {
						let temp_: Vec<&str> = end_.clone().split('(').collect();
						if i < funks.len().clone() && temp_[0] == funks[i] && !temp_bool {
							temp_bool = true;
							result_row += "func";
							result_row.push(' ');
							result_row += funks[i].clone();
							result_row.push('(');
							result_row += temp_[1].clone();
						} else if temp_bool && end_ != "end_func" {							
							result_row += end_.clone();
							result_row.push(' ');
						} else if temp_bool && end_ == "end_func"{
							result_row += end_.clone();
							result_row.push('\n');
							temp_bool = false;
							i += 1;
						}
					} 
					if temp_bool {
						result_row.push('\n');
					}
				}
			} else {
				let to_space: Vec<&str> = text_programm.as_str().split('\n').collect();
				//func_need = Words::trim(func_need.clone(), " \t");
				//let funks: Vec<&str> = func_need.as_str().split(',').collect();
				let mut i: usize = 0;
				let mut temp_bool: bool = false;
				//println!("spaces: {:?}", to_space.clone());
				//panic!("");
				for space in to_space {	//println!("косяк");
					let end_f: Vec<&str> = space.clone().trim().split(' ').collect();	
					//println!("end_f: {:?}", end_f.clone());			
					let mut b: bool = false;
					for end_ in end_f {
						if end_ == "func" { b = true; continue; }
						let temp_: Vec<&str> = end_.clone().split('(').collect();						
						if /*i < funks.len().clone() &&*/ b && !temp_bool {
							temp_bool = true;
							result_row += "func";
							result_row.push(' ');
							result_row += end_.clone();
						} else if temp_bool && end_ != "end_func" {							
							result_row += end_.clone();
							result_row.push(' ');
						} else if temp_bool && end_ == "end_func"{
							result_row += end_.clone();
							result_row.push('\n');
							temp_bool = false;
							i += 1;
						}
					} 
					if temp_bool {
						result_row.push('\n');
					}
				}
			}
			result_row
		}
		pub fn get_(&mut self, text: String, mut call_to_fn: String, mut loop_active: bool, looper: usize) -> u8 {
						
			//let mut variables_name: Vec<String> = Vec::new();
			
			//let mut object_type_string_buffer: Vec<Vec<String>> = Vec::new();		// буффер для объектов
			//let mut flag: u8 = 0;
			//-----------------------------------------------------------------------------------------------------------------
			let mut temp_values: String = String::new();			//	ВРЕМЕННЫЕ ПЕРЕМЕННЫЕ
			let mut temp_name:	 String = String::new();			//	...
			let mut temp_buffer: String = String::new();			//	...
			let mut temp_doubler: String = String::new();			// для циклов
			let mut temp_to_func: String = String::new();			// храним функции
			//let mut temp_usize_value: usize = 0;
			let mut func_inactive: bool = true;
			//let mut this_row: usize = 0;			
			let mut call_func: bool = false;
			let mut fn_active: bool = false;
			let mut bools_var: bool = false;			
			let mut loop_active_:bool=false;
			//
			let mut looper_value: usize = 0;
			let mut end_looper: bool = false;
			//let mut this_loop_active: bool=false;
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
					self.object_buffer.push((name_args_in_f[i - 1].clone().to_string(), 3));
					self.value_buffer.push(value_.clone());
					vars_to_func.push(',');
					vars_to_func += name_args_in_f[i - 1].clone();
				}
			}
			//let mut buffer_this_row: usize = 0;			

			let mut temp_weight_vec: Vec<f32> = Vec::new();			//	...
			

			let mut last_op: [usize; 3] = [0; 3];					//  ...
			//-----------------------------------------------------------------------------------------------------------------
			for ch in text.chars() {				
				//println!("ch - {:?}\n last_op - {:?}\ntemp_buffer - {:?}\ntemp_values - {:?}\ntemp_name - {:?}\nself.value_buffer.len() - {:?}\nself.value_buffer: {:?}\nself.object_buffer - {:?}\nloop_active: {}\nbools_var: {}", ch.clone(), last_op.clone(), temp_buffer.clone(), temp_values.clone(), temp_name.clone(), self.value_buffer.clone().len(), self.value_buffer.clone(), self.object_buffer.clone(), loop_active.clone(), bools_var.clone());
				if loop_active_ {
					temp_doubler.push(ch.clone());
				}
				if ch == ' ' || ch == '\t' {	
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
								if last_op[0] == 29 { 
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
							4 => {
								last_op[0] = 4;
								temp_buffer = String::new();
								temp_values = String::new();
								temp_name = String::new();
							},
							40 => { loop_active_ = true; },
							42 => { 
								if looper == looper_value {
									return 2;
								}
							},
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
					if last_op[0] == 3 && last_op[1] == 13 {
						temp_buffer.push(ch.clone());					
					} else if last_op[0] == 4 && last_op[1] == 0 && last_op[2] == 0 {
						temp_name = temp_buffer.clone();
						last_op[1] = 17;
						temp_buffer = String::new();
						// 4 - if, 33 - end, 36 - eq, 37 - !eq, 38 - >, 39 - <
					} else if last_op[0] == 4 && last_op[1] == 17 && last_op[2] == 0 {
						temp_values = temp_buffer.clone();
						last_op[2] = 17;
						temp_buffer = String::new();
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
					} else {  }					 
				} else if ch == '\n' {
					//
					// код осуществляющий работу
					//
					if last_op[0] == 0 && last_op[1] == 0 && last_op[2] == 0 {
						//panic!("");
						//let temp_buffer_vec: Vec<&str> = temp_buffer.as_str().split(' ').collect();
						//temp_buffer = temp_buffer_vec[0].clone().to_string();
						let action: usize = Words::get_action_lite(self.words.clone(), Words::trim(temp_buffer.clone(), " \t\n"));
						//println!("action: {}", action.clone());
						if action == 33 {
							bools_var = false;
							//panic!("");
							temp_buffer = String::new();
							temp_weight_vec = Vec::new();
							temp_values = String::new();
							temp_name = String::new();
							//continue;
							/*
								40 => { loop_active_ = true; },
								
									words.push("loop".to_string());// 40
									words.push("end_loop".to_string());// 41
									words.push("break".to_string()); // 42
								
							*/
						} else 
						if action == 40 && func_inactive { 					
							loop_active_ = true;
							looper_value += 1;
							/*temp_buffer = Words::trim(temp_buffer.clone(), " \t\n");
							let funks: Vec<&str> = temp_buffer.as_str().split('#').collect();
							if funks.len() > 1 {
								temp_to_func = funks[1].clone().to_string();
								temp_to_func = self.get_all_func(text.clone(), temp_to_func.clone());
							} else {
								temp_to_func = self.get_all_func(text.clone(), "".to_string());
							}							
							temp_doubler += temp_to_func.clone().as_str();*/
							last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
							temp_buffer = String::new();
							temp_weight_vec = Vec::new();
							temp_values = String::new();
							temp_name = String::new();
						} else if action == 41 && func_inactive {							
							looper_value -= 1;
							
							if looper_value == 0 {
								let mut strn_: String = String::new();
								let mut spaces: Vec<&str> = temp_doubler.as_str().split('\n').collect();
								let mut i: usize = spaces.len().clone() - 1;
								let mut b: bool = true;
								loop {
									if !b {
										
									} else if b && (spaces[i].trim() == "end_loop" || spaces[i].trim() == "end_loop#") {
										b = false;	
										spaces.remove(i.clone());
										break;
									} else {
										
									}								
									if i == 0 {
										break;
									}
									i -= 1;
								}
								for sp in spaces.clone() {
									//if sp != "loop" {
									strn_ += sp.clone();
									strn_.push('\n');
									//}
								}
								strn_ = self.get_all_func(text.clone(), "".to_string()) + strn_.as_str();
								//println!("strn_:\n{}", strn_);
								//println!("-----------------");
								//println!("looper: {}\t result: {}",looper, 
								while self.get_(strn_.clone(), "".to_string(), false, looper + 1) != 2 {

								}
								//panic!("");
								loop_active_ = false;
								/*if looper == 2 {									
									//panic!("");
								}*/
							}
							last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
							temp_buffer = String::new();
							temp_weight_vec = Vec::new();
							temp_values = String::new();
							temp_name = String::new();
							continue;
							loop_active = false;
							let funks: Vec<&str> = temp_buffer.as_str().split('#').collect();
							if funks.len() > 1 {
								//println!("temp_doubler: \n{}", temp_doubler);
								temp_doubler = text.clone();
								//panic!("");
							} else {
								//temp_doubler += temp_buffer.as_str().clone();
								//temp_doubler.push('\n');
								last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
								temp_buffer = String::new();
								temp_weight_vec = Vec::new();
								temp_values = String::new();
								temp_name = String::new();
								continue;
							}
							/*if looper_value == looper {
								panic!("end loop");
								loop_active = true;								
								//end_looper -= 1;
								println!("temp_doubler: \n{}", temp_doubler);	
								//panic!("");
							} 
							if end_looper {
								end_looper = false;
							}*/

							let mut strn_: String = String::new();
							let mut spaces: Vec<&str> = temp_doubler.as_str().split('\n').collect();
							let mut i: usize = spaces.len().clone() - 1;
							let mut b: bool = true;
							
							
							//println!("strn_: \n{}", strn_.clone());
							//println!("spaces: \n{:?}", spaces.clone());

							//panic!("");
							while self.get_(strn_.clone(), "".to_string(), true, looper + 1) != 2 {
								
							} 
							last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
							temp_buffer = String::new();
							temp_weight_vec = Vec::new();
							temp_values = String::new();
							temp_name = String::new();
							continue;
							println!("looper: {}", looper);
							panic!("");
							if looper != 0 {
								return 2;
							}
							//loop_active = true;
							//loop_active_ = true;
							//panic!("end loop");
							println!("41 looper_value: {} \t looper: {}",looper_value , looper);
							//panic!("end loop");
							//temp_buffer = Words::trim(temp_buffer.clone(), " \t\n");
							//let luper_: usize = temp_buffer.as_str().split('#').collect();
							//end_looper += 1;
							/*if looper_value == looper {
								return 1;
							}*/
						} else if action == 42 && !bools_var && !loop_active_ {
								return 2;
							last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
							temp_buffer = String::new();
							temp_weight_vec = Vec::new();
							temp_values = String::new();
							temp_name = String::new();
							continue;
						} else {
							//continue;
						}
					}
					if loop_active_ {
						last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
						temp_buffer = String::new();
						temp_weight_vec = Vec::new();
						temp_values = String::new();
						temp_name = String::new();
						continue;
					}
					if bools_var {
						let action: usize = Words::get_action_lite(self.words.clone(), Words::trim(temp_buffer.clone(), " \t\n"));
						// 4 - if, 33 - end, 36 - eq, 37 - !eq, 38 - >, 39 - <
						if action == 33 {
							bools_var = false;
						}
						last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
						temp_buffer = String::new();
						temp_weight_vec = Vec::new();
						temp_values = String::new();
						temp_name = String::new();
						continue;
					} else {
						let action: usize = Words::get_action_lite(self.words.clone(), Words::trim(temp_buffer.clone(), " \t\n"));
						// 4 - if, 33 - end, 36 - eq, 37 - !eq, 38 - >, 39 - <
						if action == 33 {
							continue;
						}
						if last_op[0] == 4 && last_op[1] == 17 && last_op[2] == 17 {
							//if looper == 0 && looper_value == 0 { panic!(""); }
							let mut vec_: Vec<&str> = temp_buffer.as_str().split(' ').collect();
							let mut i: usize = 0;
							while i < vec_.len() {
								if vec_[i] == "" {
									vec_.remove(i);
									i = 0;
								}
								i += 1;
							}							
							if !self.to_bool(temp_values.clone(), vec_[0].to_string().clone(), vec_[1].to_string().clone()) {
								bools_var = true;
								//println!("{}", bools_var);
								//panic!("");
							}
							//println!("{}", bools_var);
							//panic!("");
							last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
							temp_buffer = String::new();
							temp_weight_vec = Vec::new();
							temp_values = String::new();
							temp_name = String::new();
						}														
					}					
					if call_func {
						let action: usize = Words::get_action_lite(self.words.clone(), temp_buffer.clone());
						if action == 34 && fn_active {
							self.remove_some_objects(name_args_in_f.clone());
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
                    } else if last_op[0] == 40 && looper == looper_value {	
						last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
						temp_buffer = String::new();
						temp_weight_vec = Vec::new();
						temp_values = String::new();
						temp_name = String::new();
						//return 2;
						/*
								40 => { loop_active_ = true; },
								
								words.push("loop".to_string());// 40
								words.push("end_loop".to_string());// 41
								words.push("break".to_string()); // 42
								
						*/

					} else if last_op[0] == 4 && last_op[1] == 17 && last_op[2] == 17 {
						let mut vec_: Vec<&str> = temp_buffer.as_str().split(' ').collect();
						let mut i: usize = 0;
						while i < vec_.len() {
							if vec_[i] == "" {
								vec_.remove(i);
								i = 0;
							}
							i += 1;
						}

						if !self.to_bool(temp_values.clone(), vec_[0].to_string().clone(), vec_[1].to_string().clone()) {
							bools_var = true;
							//println!("{}", bools_var);
							//panic!("");
						}
						//println!("{}", bools_var);
						//panic!("");
						last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
						temp_buffer = String::new();
						temp_weight_vec = Vec::new();
						temp_values = String::new();
						temp_name = String::new();
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
						/*if !loop_active { 
							temp_buffer = String::new();
							temp_weight_vec = Vec::new();
							temp_values = String::new();
							temp_name = String::new();		
							
							last_op[0] = 0;	last_op[1] = 0;	last_op[2] = 0;
							continue; 
						}*/
						if !loop_active && looper == 0 {
							self.value_buffer.push(String::new());// было: temp_buffer.clone()
							self.object_buffer.push((temp_values.clone(), 1));	// (name, type) // 0 - нейрон, 1 -  объект, 2 - сервер
							//  i_have_u(&mut self, mut temp_buffer: String, mut temp_values: String, last_op: [usize; 3])
							//println!("self.object_buffer: ", self.object_buffer.clone());
							last_op[0] = 17; last_op[1] = 15; last_op[2] = 0;
							
							
							
							self.i_have_u(temp_buffer.clone(), temp_values.clone(), last_op.clone());
						}
                        //println!("{:?}\n{:?}\n{:?}", self.object_buffer, self.value_buffer.clone(), self.neural_network.debug()); // ДЛЯ КОСЯКОВ
                        temp_buffer = String::new();
						temp_weight_vec = Vec::new();
						temp_values = String::new();
						temp_name = String::new();		
						
						last_op[0] = 0;	last_op[1] = 0;	last_op[2] = 0;

					} else if last_op[0] == 3 && last_op[1] != 13 {
						/*if !loop_active { 
							temp_buffer = String::new();
							temp_weight_vec = Vec::new();
							temp_values = String::new();
							temp_name = String::new();		
							
							last_op[0] = 0;	last_op[1] = 0;	last_op[2] = 0;
							continue; 
						}*/
						if !loop_active && looper == 0 {
							self.value_buffer.push(String::new());
							self.object_buffer.push((temp_buffer.clone(), 1));
						}
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
							
							let a = match self.get_index_hight_data(t, "".to_string()) { 
								Ok(A) => { A },
								Err(e) => { panic!("переменной не существует"); 0 },
							};
							let a = match self.get_value_to_index(a) {
								Ok(A) => { A },
								Err(e)=> { panic!("нет значения. print"); String::new() },
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
                                        }*/
                                        /*if u_for_neyron != 0 {
                                            u_for_neyron -= 1;
                                        }*/
                                  //      println!("{}", self.neural_network.get_neyron_name(u_for_neyron));
                                  //  }
                                  //  break;
                                //}
                            //}
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
						if !loop_active && looper == 0 {
							//println!("looper: {}", looper.clone())
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
						}
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
						//println!("text: \n{}", text.clone());
						/*
							temp_buffer - "lib,lib1,lib2" // аргументы
							temp_values - "lib,lib1,lib2" // аргументы
							temp_name - "import" // имя функции
						*/
						let mut spec_func: bool = false;
						let mut spec_func_name: String = String::new();
						let spec_func_names: Vec<&str> = vec![
							"import", "close_import", "extern_func", "exit"
						];
						panic!("");
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
						//println!("{:?}", run_row);
						//panic!("");
						//pub fn get_(&mut self, text: String, mut call_to_fn: String) -> u8
						//println!("вызываем функцию, передаём:");
						//println!("temp_buffer: {}\ntext: \n{}", temp_buffer.clone(), text.clone());
						let temp__ = self.get_(text.clone(), run_row.clone(), false, looper);
						if /*temp_buffer.as_str().trim() == "end_loop" 
								&&*/ temp__ != 1 && temp__ != 0 {
							println!("temp__: {}", temp__.clone());
							//println!("temp_buffer: {}\ntext: \n{}", temp_buffer.clone(), text.clone());
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
				} 
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
				let action: usize = Words::get_action_lite(self.words.clone(), temp_buffer.clone());  
				match action { // блок занимающийся прочей обработкой	
					17 => { 
						if (ch == ' ' || ch == '\t' ) && (last_op[1] != 15 && last_op[1] != 11) { 
							// else {
								continue; 
							//}
						} // обработка происходит наверху. если симвом не '=', и не ']', 
						  // то считаем что пробел/таб должен игнорироваться
						if last_op[0] == 1 && last_op[1] == 0 && last_op[2] == 0 {
							//let action_char: usize = Words::get_action_lite(self.words.clone(), ch.to_string());
							match ch {
								'{' => {						
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
						} else if last_op[0] == 3 && last_op[1] == 0 && last_op[2] == 0 { //добавили имя (в кучу)
							let action_char: usize = Words::get_action_lite(self.words.clone(), ch.to_string());
							match action_char {
								15 => {									
										last_op[1] = 13;
										temp_values = temp_buffer.clone();
										temp_buffer = String::new();
								},
								17 => {									
									temp_buffer.push(ch.clone());									
								},
								_  => { if last_op[1] == 13 { temp_buffer.push(ch.clone()); } },
							}						
                        } else if last_op[0] == 2 && last_op[1] == 0 {
                            match ch.clone(){
                                '=' => { 
                                    temp_name = temp_buffer.clone(); 
                                    temp_buffer = String::new();
                                    last_op[0] = 2; last_op[1] = 15;                                    
                                }, 
							    _ => { temp_buffer.push(ch.clone()); },
                            } // если есть знак присваивания
                        } else if last_op[0] == 22 && last_op[1] == 0 && last_op[2] == 0 { 
                            //remove
                            temp_values.push(ch.clone());
                        } else if last_op[0] == 17 && last_op[1] == 15 && last_op[2] == 0 {
                            temp_buffer.push(ch.clone()); // для переменных/математики
                        } else {
                            match ch.clone(){
                                '=' => { 
									if last_op[1] == 15 {
										temp_buffer.push(ch.clone());
									}
                                    if last_op[0] == 0 && last_op[1] == 0 && last_op[2] == 0 {
										temp_values = temp_buffer.clone();
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
										temp_name = temp_buffer.clone().as_str().trim().to_string();
										temp_buffer = String::new();
										continue;
									} else if last_op[0] == 0 && last_op[1] == 0 && last_op[2] == 0 {
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
					},
					_  => { temp_buffer.push(ch.clone()); }
				}				
			}
            0 // вывод 
		}
		fn trim<'a>(text: String, to_: &'a str) -> String {
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
		pub fn is_math(&self, st: String) -> bool {
			let math_in: Vec<&str> = st.as_str().trim().split(':').collect();
			//println!("math_in: {:?}", math_in.clone());
			if math_in.len() <= 1 { return false; }
			match math_in[1].trim() {
				"digit" => { return true },
				"float" => { return true },
				"string" => { return true },
				"str" => { return true },
				"math" => { return true },
				_ => { return false },
			}
		}
		pub fn math_formating(&self, from_math_row: String) -> (String, u8) {
			//println!("trim_row_: {:?}", Words::trim(from_math_row.clone(), " \t\n"));
			let from_math_row: Vec<&str> = from_math_row.as_str().split(':').collect();
			let from_math_row: String = from_math_row[0].to_string().clone();
			let trim_row_: String = Words::trim(from_math_row.clone(), " \t\n");
			let mut variable_: String = "".to_string();
			let mut result_row: String = String::new();
			let mut result_type: u8 = 0;
			for ch in trim_row_.chars() {
				match ch {
					'('|')'|'+'|'-'|'*'|'/' => {
						if variable_ == "".to_string() {
							//
						} else {
							let temp_: Vec<&str> = variable_.as_str().split('[').collect();
							if temp_.len() == 1 {
								let mut b: bool = false;
								let indx: usize = match self.get_index_hight_data(temp_[0].to_string().clone(), "".to_string()) {
									Ok(A) => { b = true; A },
									Err(e)=> { 0 },
								};
								if b {
									let value_: String = match self.get_value_to_index(indx) {
										Ok(A) => { A },
										Err(e)=> { panic!("что-то пошло не так, ошибка математики. ошибка 450"); String::new() },
									};
									result_row += value_.as_str();
								} else {
									result_row += variable_.as_str().clone();
									result_type = 1;
								}
							} else {
								let temp_t: Vec<&str> = temp_[1].split(']').collect();
								let mut b: bool = false;
								let indx: usize = match self.get_index_hight_data(temp_[0].to_string().clone(), temp_t[0].to_string().clone()) {
									Ok(A) => { b = true; A },
									Err(e)=> { 0 },
								};
								if b {
									let value_: String = match self.get_value_to_index(indx) {
										Ok(A) => { A },
										Err(e)=> { panic!("что-то пошло не так, ошибка математики. ошибка 450"); String::new() },
									};
									result_row += value_.as_str();
								} else {
									result_row += variable_.as_str().clone();
									result_type = 1;
								}
							}
						}
						variable_ = "".to_string();
						result_row.push(ch.clone());
					},
					_ => { variable_.push(ch.clone()); },
				}
			}
			if variable_ == "".to_string() {
							//
			} else {
				let temp_: Vec<&str> = variable_.as_str().split('[').collect();
				if temp_.len() == 1 {
					let mut b: bool = false;
					let indx: usize = match self.get_index_hight_data(temp_[0].to_string().clone(), "".to_string()) {
						Ok(A) => { b = true; A },
						Err(e)=> { 0 },
					};
					if b {
						let value_: String = match self.get_value_to_index(indx) {
							Ok(A) => { A },
							Err(e)=> { panic!("что-то пошло не так, ошибка математики. ошибка 450"); String::new() },
						};
						result_row += value_.as_str();
					} else {
						result_row += variable_.as_str().clone();
						result_type = 1;
					}
				} else {
					let temp_t: Vec<&str> = temp_[1].split(']').collect();
					let mut b: bool = false;
					let indx: usize = match self.get_index_hight_data(temp_[0].to_string().clone(), temp_t[0].to_string().clone()) {
						Ok(A) => { b = true; A },
						Err(e)=> { 0 },
					};
					if b {
						let value_: String = match self.get_value_to_index(indx) {
							Ok(A) => { A },
							Err(e)=> { panic!("что-то пошло не так, ошибка математики. ошибка 450"); String::new() },
						};
						result_row += value_.as_str();
					} else {
						result_row += variable_.as_str().clone();
						result_type = 1;
					}
				}
			}
			//println!("result_row: {:?}", result_row.clone());
			(result_row, result_type) // 0 - float, 1 - string // ROW, MATH_TYPE
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
				//println!("index_first_object: {}", index_first_object);
				//panic!("");
			}
			
			/*match self.get_index_r(temp_values.clone()){
				Ok(A) => A,
				Err(e) => { panic!("first variable not found!!"); 0 },
			};*/
			let mut index_second_object: usize = 0;
			let struct_two_obj: Vec<&str> = temp_buffer.as_str().trim().split('[').collect();
			//println!("struct_two_obj: {:?}", struct_two_obj.clone());
			if struct_two_obj.len() > 1 {
				let ar: Vec<&str> = struct_two_obj[1].trim().split(']').collect();
				index_second_object = match self.get_index_hight_data(
						struct_two_obj[0].to_string().clone(), 
						ar[0].to_string().clone()){
					Ok(A) => { where_two_obj = true; A },
					Err(e) => { 0 },
				};
				//println!("index_second_object: {}", index_second_object);
			} else {
				index_second_object = match self.get_index_hight_data(
						temp_buffer.clone(), 
						"".to_string()){
					Ok(A) => { where_two_obj = true; A },
					Err(e) => { 0 },
				};
				//println!("index_second_object: {}", index_second_object);
				/*println!("index_first_object: {}", index_first_object);
				panic!("");*/
			}
			if !where_two_obj {
				if self.is_math(temp_buffer.clone()) {
					temp_buffer = self.math_formating(temp_buffer.clone()).0;
					//let ret_: String = ret_ex(temp_buffer.as_str());
					//println!("ret_: {:?}", ret_);
					let a: &str = temp_buffer.as_str();
					let prompt = match CString::new(a){
						Ok(A) => { A },
						Err(e)=> { panic!("приводимая строка имеет неправильный вид"); CString::new("").unwrap() },
					};
					//unsafe {
					let rl_prompt = prompt.as_ptr();
					let mut result: f32 = 0.0;
					unsafe {
						result = eval(rl_prompt);
						//println!("result: {}", result.clone());
					}
					if result != 0.0 {
						temp_buffer = result.clone().to_string();
					}
					/*unsafe {
						math_text_ = prompt.as_ptr();

						println!("{:?}", math_text_);

						math_text_ = ptr::null();
					}*/
					//panic!("вошли в матан");
				}
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
						//println!("self.object_buffer.clone()[index_second_object.clone()].1: {:?}", self.object_buffer.clone()[index_second_object.clone()].1.clone());
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
								//println!("temp__: {:?}", temp__.clone());
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
											//println!("удаляем {:?}", self.object_buffer[index_first_object+1].clone());
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
									//println!("пытаемся добавить: {:?}", names_[i].to_string().clone());
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
										self.object_buffer.push((names_[i].trim().to_string().clone(), 25));
										self.value_buffer.push(names_[i].trim().to_string().clone());
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
			let word: Vec<&str> = word.as_str().split('#').collect();
			let word: String = word[0].to_string();
			let mut index: usize = 0;
			for word_ in words {
				if word == word_ {
					return index + 1;
				}
				index += 1;
			} // в случае если слово не найдено - возвращаем 17 (прочее значение)
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
			let t = Words::to_vec(word.clone());			
			for i in 0..t.len(){				//;
				for k in 0..words.clone().len() {
					if k < words.len(){
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
				}				
			} 			
			let mut index: usize = 16;
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
				let mut right: usize = 0;
				let temp: Vec<char> = Words::to_vec(word.clone());
				let temp1: Vec<char>= Words::to_vec(words[index].clone());
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