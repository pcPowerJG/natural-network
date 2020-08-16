pub mod language{
    pub struct Words {
    words: Vec<String>,     // сами ключевые слова		
    object_buffer: Vec<(String, usize)>,              // наименования переменных 	// (name, type)
    value_buffer: Vec<String>,                        // значения переменных
    import_handle: Vec<*const libc::c_void>, // для подключения внешних библиотек
    import_names: Vec<String>, // хранилище внешних библиотек
  }

  extern crate libc;
	use libc::size_t;
	use std::ffi::CString;
	use std::ptr;
	use std::ffi::CStr;
	use std::io;
	use std::str::FromStr;
	
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
		//char* char_rf_func(void* handle, char * func_name, char* arg){
		fn char_rf_func(handle: *const libc::c_void, func_name: *const libc::c_char, arg: *const libc::c_char) -> *const libc::c_char;
		// int_rf_char_func(void* handle, char * func_name, char* arg){
		fn int_rf_char_func(handle: *const libc::c_void, func_name: *const libc::c_char, arg: *const libc::c_char) -> i32;
		// int int_rf_func(void* handle, char * func_name){
		fn int_rf_void_func(handle: *const libc::c_void, func_name: *const libc::c_char) -> i32;
	}

  	pub fn create() -> Words {
		let mut words: Vec<String> = Vec::new(); 			
		words.push("object".to_string());//1	 // используется для создания объекта, который хранит значения в памяти	
		words.push("if".to_string());//2	// оператор условия, нужен для сравнения ДВУХ параметров		
		words.push("exit_()".to_string());//3//выход из приложения
		words.push("func".to_string());     //4//инициализация функции 
		words.push("print".to_string());//5 // вывод на консоль		
		words.push("remove".to_string());//6 //удаление
		/*
		    ПРИМЕР: 
		        object obj1                                 ; создали объект
		        remove obj1                               ; удалили объект
		*/		
		words.push("array".to_string());//7 // создание массива
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
		words.push("struct".to_string()); //8 // создание структуры
		/*
				struct st_one
					string s1
					int s2
					object s3
				end
		*/
		words.push("end".to_string());//9//end operation
		words.push("end_func".to_string()); // 10 // конец функции
		/*
			func func(void)
			end_func
			
		*/ 			
		words.push("return".to_string()); //  11
		words.push("!eq".to_string());//  12
		words.push(">".to_string());  //  13
		words.push("<".to_string());  //  14
		words.push("loop".to_string());// 15
		words.push("end_loop".to_string());// 16		
		words.push("_".to_string()); // 17 // для сравнений и просто в качестве НЕ ключевого слова
		words.push("break".to_string()); // 18
		words.push("true".to_string()); // 19
		words.push("false".to_string()); // 20
		Words{ 
			words: words,  
   			object_buffer: Vec::new(), 
			value_buffer: Vec::new(),
			import_handle: Vec::new(),
			import_names: Vec::new(),
		}
	}
	impl Words{
		fn get_action(&self, word: String)->usize{
			let word: Vec<&str> = word.as_str().split(';').collect();
			let word: String = word[0].to_string();
			let mut index: usize = 0;
			for word_ in self.words.clone() {
				if word == word_ {
					return index + 1;
				}
				index += 1;
			} // в случае если слово не найдено - возвращаем 17 (прочее значение)
			17
		}
		pub fn println(&self) {
			println!("object_buffer: {:?}", self.object_buffer);
			println!("value_buffer: {:?}", self.value_buffer);
		}
		fn add_vars(&mut self, vars_name: String, mut vars_value: String, vars_type: usize) {
			//object_buffer: Vec<(String, usize)>
			//value_buffer: Vec<String>
			if vars_value.clone().split('\"').collect::<Vec<&str>>().len() > 1 {
				vars_value = vars_value.split('\"').collect::<Vec<&str>>()[1].to_string();
			} else {
				vars_value = vars_value.clone().trim().to_string();
			}
			self.object_buffer.push((vars_name, vars_type));
			self.value_buffer.push(vars_value);
		}
		fn remove_vars(&mut self, vars_name: String) {
			for i in 0..self.object_buffer.len() {
				if self.object_buffer[i].0.clone() == vars_name {
					self.object_buffer.remove(i);
					self.value_buffer.remove(i);
					return;
				}
			}
		}
		fn set_value(&mut self, vars_name: String, mut vars_value: String) {
			for i in 0..self.object_buffer.len() {
				if self.object_buffer[i].0 == vars_name {
					if vars_value.clone().split('\"').collect::<Vec<&str>>().len() > 1 {
						vars_value = vars_value.split('\"').collect::<Vec<&str>>()[1].to_string();
					} else {
						vars_value = vars_value.clone().trim().to_string();
					}
					self.value_buffer[i] = vars_value.clone();
					return;
				}
			}
			
		}
		pub fn search_var(&self, vars_name: String) -> (String, usize, bool) {
			for i in 0..self.object_buffer.len() {
				if self.object_buffer[i].0 == vars_name {
					let value: String = self.value_buffer[i].clone();
					let type_: usize = self.object_buffer[i].1.clone();
					return (value, type_, true); 
				}
			}
			(String::new(), 0, false)
		}
		fn is_math(txt: String) -> bool {
			for ch in txt.chars() {
				match ch {
					'+' | '-' | '*' | '/' | '^' => { return true; },
					_ => { },
				}
			} false
		}		
		fn if_work(&self, text: String) -> bool {
			let mut bracets_count: i32 = -1;
			let mut result: bool = false;
			let mut text_to_recurs: String = String::new();
			let mut value_a: String = String::new();
			let mut value_b: String = String::new();
			let mut value_b_flag: bool = false; // показывает что записали первое значение			
			// a < (ret_value)
			let mut operation: u8 = 0;
			let mut operation1:u8 = 0;
			// &  - 1
			// && - 2
			// |  - 3
			// || - 4
			// > -  5
			// >= - 6
			// <  - 7
			// <= - 8
			// == - 9
			// != - 10
			// '='- 11
			// '!'- 12
			for ch in text.chars() {
				match ch {
					' ' | '\t' => { continue; },
					'\n' => {
						//fn match_operation(operation: u8, operation1: u8, value_a: &mut String, value_b: &mut String, result: &mut bool) {
						match operation {
							1 | 2 => { 
								result = if value_a == 1i32.to_string() && value_b == 1i32.to_string() { true } 
									else { false };
							},
							3 | 4 => {
								result = if value_a == 1i32.to_string() || value_b == 1i32.to_string() { true } 
									else { false };
							},
							5 => {
								result = 
								if value_a.as_str().to_string().parse::<f64>().expect("не мат выражение") > value_b.to_string().parse::<f64>()
									.expect("не мат выражение") { true } 
									else { false };
							},
							6 => {
								result = 
								if value_a.parse::<f64>().expect("не мат выражение") >= value_b.parse::<f64>()
									.expect("не мат выражение") { true } 
									else { false };
							},
							7 => {
								result = 
								(if value_a.parse::<f64>().expect("не мат выражение") < value_b.parse::<f64>()
									.expect("не мат выражение") { true } 
									else { false });
							},
							8 => {
								result = 
								(if value_a.parse::<f64>().expect("не мат выражение") <= value_b.parse::<f64>()
									.expect("не мат выражение") { true } 
									else { false });
							},
							9 => {
								result = 
								(if value_a.parse::<f64>().expect("не мат выражение") == value_b.parse::<f64>()
									.expect("не мат выражение") { true } 
									else { false });
							},
							10 => {
								result = 
								(if value_a.parse::<f64>().expect("не мат выражение") != value_b.parse::<f64>()
									.expect("не мат выражение") { true } 
									else { false });
							},
							12 => {
								if value_b == 0i32.to_string() { value_b = 1i32.to_string() } else { value_b = 0i32.to_string() }
								match operation1 {
									1 | 2 => { 
										result = if value_a == 1i32.to_string() && (value_b == 1i32.to_string()) { true } 
											else { false };
									},
									3 | 4 => {
										result = if value_a == 1i32.to_string() || (value_b == 1i32.to_string()) { true } 
											else { false };
									},
									5 => {
										result = 
										(if value_a.parse::<f64>().expect("не мат выражение") > value_b.parse::<f64>()
											.expect("не мат выражение") { true } 
											else { false });
									},
									6 => {
										result = 
										(if value_a.parse::<f64>().expect("не мат выражение") >= value_b.parse::<f64>()
											.expect("не мат выражение") { true } 
											else { false });
									},
									7 => {
										result = 
										(if value_a.parse::<f64>().expect("не мат выражение") < value_b.parse::<f64>()
											.expect("не мат выражение") { true } 
											else { false });
									},
									8 => {
										result = 
										(if value_a.parse::<f64>().expect("не мат выражение") <= value_b.parse::<f64>()
											.expect("не мат выражение") { true } 
											else { false });
									},
									9 => {
										result = 
										(if value_a.parse::<f64>().expect("не мат выражение") == value_b.parse::<f64>()
											.expect("не мат выражение") { true } 
											else { false });
									},
									10 => {
										result = 
										(if value_a.parse::<f64>().expect("не мат выражение") != value_b.parse::<f64>()
											.expect("не мат выражение") { true } 
											else { false });
									},
									_ => { 									
										panic!("some ops: {}", operation1);
									},
								}
							},
							_ => { 
								result = if (value_a == 1i32.to_string()) && (value_b == 1i32.to_string()) { true } 
									else { false };
								//panic!("some ops. operation: {}", operation);
							},
						}
						//println!("result: {}", result);
						return result;
					},
					'(' => {
						if bracets_count != -1 {
							text_to_recurs.push(ch.clone());
						}
						if bracets_count == -1 {
							bracets_count = 1;
							continue;
						} bracets_count += 1;
					},
					')' => {						
						bracets_count -= 1;
						if bracets_count == 0 {
							//println!("text_to_recurs: {}", text_to_recurs);
							text_to_recurs.push('\n');							
							if self.if_work(value_b.clone() + text_to_recurs.clone().as_str()) {
								if value_a != "".to_string() && value_a != String::new() {
									value_b = 1i32.to_string();
								} else {
									value_a = 1i32.to_string();
								}
							} else {
								if value_a != "".to_string() && value_a != String::new() {
									value_b = 0i32.to_string();
								} else {
									value_a = 0i32.to_string();
								}
							} 
							if operation != 0 {
								match operation {
									1 | 2 => { 
										result = (if (value_a == 1i32.to_string()) && (value_b == 1i32.to_string()) { true } 
											else { false });
									},
									3 | 4 => {
										result = (if (value_a == 1i32.to_string()) || (value_b == 1i32.to_string()) { true } 
											else { false });
									},
									5 => {
										result = 
										(if value_a.parse::<f64>().expect("не мат выражение") > value_b.parse::<f64>()
											.expect("не мат выражение") { true } 
											else { false });
									},
									6 => {
										result = 
										(if value_a.parse::<f64>().expect("не мат выражение") >= value_b.parse::<f64>()
											.expect("не мат выражение") { true } 
											else { false });
									},
									7 => {
										result = 
										(if value_a.parse::<f64>().expect("не мат выражение") < value_b.parse::<f64>()
											.expect("не мат выражение") { true } 
											else { false });
									},
									8 => {
										result = 
										(if value_a.parse::<f64>().expect("не мат выражение") <= value_b.parse::<f64>()
											.expect("не мат выражение") { true } 
											else { false });
									},
									9 => {
										result = 
										(if value_a.parse::<f64>().expect("не мат выражение") == value_b.parse::<f64>()
											.expect("не мат выражение") { true } 
											else { false });
									},
									10 => {
										result = 
										(if value_a.parse::<f64>().expect("не мат выражение") != value_b.parse::<f64>()
											.expect("не мат выражение") { true } 
											else { false });
									},
									_ => { 
										panic!("some ops. operation: {}", operation);
									},
								}
							}
							text_to_recurs = String::new();
							bracets_count -= 1;
						} else {
							text_to_recurs.push(ch.clone());
						}
					},
					'&' => {
						if bracets_count != -1 {
							text_to_recurs.push(ch.clone());
							continue;
						}
						if !value_b_flag {
							value_b_flag = true; // начали работу логики
						} else if value_b_flag {
							// &  - 1
							// && - 2
							// |  - 3
							// || - 4
							// > -  5
							// >= - 6
							// <  - 7
							// <= - 8
							// == - 9							
							match operation {
								1 | 2 => { 
									result = (if (value_a == 1i32.to_string()) && (value_b == 1i32.to_string()) { true } 
										else { false });
								},
								3 | 4 => {
									result = (if (value_a == 1i32.to_string()) || (value_b == 1i32.to_string()) { true } 
										else { false });
								},
								5 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") > value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								6 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") >= value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								7 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") < value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								8 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") <= value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								9 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") == value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								10 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") != value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								12 => {
									if value_b == 0i32.to_string() { value_b = 1i32.to_string() } else { value_b = 0i32.to_string() }
									match operation1 {
										1 | 2 => { 
											result = (if (value_a == 1i32.to_string()) && (value_b == 1i32.to_string()) { true } 
												else { false });
										},
										3 | 4 => {
											result = (if (value_a == 1i32.to_string()) || (value_b == 1i32.to_string()) { true } 
												else { false });
										},
										5 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") > value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										6 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") >= value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										7 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") < value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										8 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") <= value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										9 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") == value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										10 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") != value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										_ => { 									
											panic!("some ops: {}", operation1);
										},
									}
								},
								_ => { 									
									panic!("some ops: {}", operation);
								},
							}							
							value_a = (if result { 1i32.to_string() } else { 0i32.to_string() });
							value_b = String::new();
						}
						//if operation == 0 {
						operation = 1;
						operation1 = operation;
						//} else {
						//	operation = 2;
						//}
					},
					'|' => {
						if bracets_count != -1 {
							text_to_recurs.push(ch.clone());
							continue;
						}
						if !value_b_flag {
							value_b_flag = true; // начали работу логики
						} else if value_b_flag {
							// &  - 1
							// && - 2
							// |  - 3
							// || - 4
							// > -  5
							// >= - 6
							// <  - 7
							// <= - 8
							// == - 9							
							match operation {
								1 | 2 => { 
									result = (if (value_a == 1i32.to_string()) && (value_b == 1i32.to_string()) { true } 
										else { false });
								},
								3 | 4 => {
									result = (if (value_a == 1i32.to_string()) || (value_b == 1i32.to_string()) { true } 
										else { false });
								},
								5 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") > value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								6 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") >= value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								7 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") < value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								8 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") <= value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								9 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") == value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								10 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") != value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								12 => {
									if value_b == 0i32.to_string() { value_b = 1i32.to_string() } else { value_b = 0i32.to_string() }
									match operation1 {
										1 | 2 => { 
											result = (if (value_a == 1i32.to_string()) && (value_b == 1i32.to_string()) { true } 
												else { false });
										},
										3 | 4 => {
											result = (if (value_a == 1i32.to_string()) || (value_b == 1i32.to_string()) { true } 
												else { false });
										},
										5 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") > value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										6 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") >= value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										7 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") < value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										8 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") <= value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										9 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") == value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										10 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") != value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										_ => { 									
											panic!("some ops: {}", operation1);
										},
									}
								},
								_ => { 
									panic!("some ops");
								},
							}
							value_a = (if result { 1i32.to_string() } else { 0i32.to_string() });
							value_b = String::new();
						}
						operation = 3;
						operation1 = 3;
					},
					'>' => {
						if bracets_count != -1 {
							text_to_recurs.push(ch.clone());
							continue;
						}
						if !value_b_flag {
							value_b_flag = true; // начали работу логики
						} else if value_b_flag {
							// &  - 1
							// && - 2
							// |  - 3
							// || - 4
							// > -  5
							// >= - 6
							// <  - 7
							// <= - 8
							// == - 9							
							match operation {
								1 | 2 => { 
									result = (if (value_a == 1i32.to_string()) && (value_b == 1i32.to_string()) { true } 
										else { false });
								},
								3 | 4 => {
									result = (if (value_a == 1i32.to_string()) || (value_b == 1i32.to_string()) { true } 
										else { false });
								},
								5 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") > value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								6 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") >= value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								7 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") < value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								8 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") <= value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								9 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") == value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								10 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") != value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								12 => {
									if value_b == 0i32.to_string() { value_b = 1i32.to_string() } else { value_b = 0i32.to_string() }
									match operation1 {
										1 | 2 => { 
											result = (if (value_a == 1i32.to_string()) && (value_b == 1i32.to_string()) { true } 
												else { false });
										},
										3 | 4 => {
											result = (if (value_a == 1i32.to_string()) || (value_b == 1i32.to_string()) { true } 
												else { false });
										},
										5 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") > value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										6 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") >= value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										7 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") < value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										8 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") <= value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										9 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") == value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										10 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") != value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										_ => { 									
											panic!("some ops: {}", operation1);
										},
									}
								},
								_ => { 
									panic!("some ops");
								},
							}
							value_a = (if result { 1i32.to_string() } else { 0i32.to_string() });
							value_b = String::new();
						}
						operation = 5;
						operation1 = 5;
					},
					'<' => {
						if bracets_count != -1 {
							text_to_recurs.push(ch.clone());
							continue;
						}
						if !value_b_flag {
							value_b_flag = true; // начали работу логики
						} else if value_b_flag {
							// &  - 1
							// && - 2
							// |  - 3
							// || - 4
							// > -  5
							// >= - 6
							// <  - 7
							// <= - 8
							// == - 9							
							match operation {
								1 | 2 => { 
									result = (if (value_a == 1i32.to_string()) && (value_b == 1i32.to_string()) { true } 
										else { false });
								},
								3 | 4 => {
									result = (if (value_a == 1i32.to_string()) || (value_b == 1i32.to_string()) { true } 
										else { false });
								},
								5 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") > value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								6 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") >= value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								7 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") < value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								8 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") <= value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								9 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") == value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								10 => {
									result = 
									(if value_a.parse::<f64>().expect("не мат выражение") != value_b.parse::<f64>()
										.expect("не мат выражение") { true } 
										else { false });
								},
								12 => {
									if value_b == 0i32.to_string() { value_b = 1i32.to_string() } else { value_b = 0i32.to_string() }
									match operation1 {
										1 | 2 => { 
											result = (if (value_a == 1i32.to_string()) && (value_b == 1i32.to_string()) { true } 
												else { false });
										},
										3 | 4 => {
											result = (if (value_a == 1i32.to_string()) || (value_b == 1i32.to_string()) { true } 
												else { false });
										},
										5 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") > value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										6 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") >= value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										7 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") < value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										8 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") <= value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										9 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") == value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										10 => {
											result = 
											(if value_a.parse::<f64>().expect("не мат выражение") != value_b.parse::<f64>()
												.expect("не мат выражение") { true } 
												else { false });
										},
										_ => { 									
											panic!("some ops: {}", operation1);
										},
									}
								},
								_ => { 
									panic!("some ops");
								},
							}
							value_a = (if result { 1i32.to_string() } else { 0i32.to_string() });
							value_b = String::new();
						}
						operation = 7;
						operation1 = 7;
					},
					'=' => {
						if bracets_count != -1 {
							text_to_recurs.push(ch.clone());
							continue;
						}
						if !value_b_flag {
							value_b_flag = true; // начали работу логики
						}					
						match operation {
							5 | 7 => { operation += 1; },
							11    => { operation = 9; },
							12    => { operation = 10; },
							_ => { operation = 11; },
						} 
						operation1 = operation;
						//println!("operation: {}", operation);
						continue;
					},
					'!' => {
						if bracets_count != -1 {
							text_to_recurs.push(ch.clone());
							continue;
						}
						operation = 12;
					},
					_ => {
						if bracets_count == -1 {
							if !value_b_flag {
								value_a.push(ch.clone());
							} else {
								value_b.push(ch.clone());
							}
						} else {
							text_to_recurs.push(ch.clone());
						}
					},
				}
			}
			false
		}
		fn is_digit(text: String) -> bool {			
			for ch in text.chars() {
				match ch {
					'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | ' ' | '.' => {
					},
					_ => {
						return false;
					},
				}
			} true
		}
		fn trim(text: String) -> String {
			let mut result_string: String = String::new();
			for ch in text.chars() {
				match ch {
					' ' | '\t' => {

					},
					_ => {
						result_string.push(ch.clone());
					},
				}
			} result_string
		}
		// конвертирует строку с переменными в строку
		fn math_work(&self, text: String) -> String {
			let text: String = Words::trim(text.clone());
			//println!("text: {}", text); panic!("");
			let mut result_string: String = String::new();
			let mut temp_string: String = String::new();
			for ch in text.chars() {
				match ch {
					'+' | '-' | '/' | '*' | '(' | ')' | '&' | '|' | '!' | '=' | '<' | '>' => {
						//fn is_digit(text: String) -> bool {
						if Words::is_digit(temp_string.clone()) {
							result_string += temp_string.clone().as_str();							
						} else {
							//pub fn search_var(&self, vars_name: String) -> (String, usize, bool) {
							result_string += self.search_var(temp_string).0.clone().as_str();
						}
						result_string.push(ch.clone());
						temp_string = String::new();
					},
					_ => {
						temp_string.push(ch.clone());
					},
				}
			} 
			let (value, type_, _temp) = self.search_var(temp_string.clone());
			if _temp {
				result_string += value.as_str();
			} else {
				result_string += temp_string.clone().as_str();
			} result_string
		}
		fn func_work(&mut self/*, text: String,*/, func_name: String, func_arg: String, func_text: String) {
			let temp_values = self.search_var(func_name.clone()).0;
			// убрать текст			
			// func name & func arg рабочие
			//panic!("func_arg:\n {}\n\n************\nfunc_text:\n{}", func_arg, func_text);
			let mut func_flag: bool = false;
			let mut my_func_flag: bool = false;			
			let mut args_name: Vec<&str> = func_arg.split(',').collect::<Vec<&str>>();
			let _n_text: Vec<&str> = func_text.split('\n').collect::<Vec<&str>>(); // не работает
			// вначале мы ищем фунцию по имени, затем сеём аргумент, клонируем текст и запусаем
			let mut i: usize = 0;
			for _ in 0.._n_text.len() {	
				let _s_text: Vec<&str> = _n_text[i].split(' ').collect::<Vec<&str>>();
				func_flag = false;
				for j in 0.._s_text.len() {					
					if _s_text[j] == "func" { 
						func_flag = true; 
						continue;
					}
					if func_flag {
						if _s_text[j].split('(').collect::<Vec<&str>>()[0] == func_name.as_str() {
							my_func_flag = true;
							let mut args_value: Vec<String> = Vec::new();
							if args_name.len() > 0 {
								for _j in 0..args_name.len() {
									args_value.push(self.search_var(args_name[_j].to_string()).0.clone());									
								}
							} 
							args_name = _s_text[j].split('(').collect::<Vec<&str>>()[1].split(')')
								.collect::<Vec<&str>>()[0].split(',').collect::<Vec<&str>>();
							for h in 0..args_value.len(){
								//fn add_vars(&mut self, vars_name: String, vars_value: String, vars_type: usize)
								self.add_vars(args_name[h].to_string(), args_value[h].clone(), 1);
							}
							break;
						} else {
							//func_flag = false; break;
						}
					}
				}
				i += 1;
				if my_func_flag { break; }				
			}
			if !my_func_flag {
				panic!("function '{}' is not found", func_name);
			}
			let mut text: String = String::new();
			text.push('\n');
			for j in i.._n_text.len() {
				if _n_text[j].trim() != "end_func" {
					text += _n_text[j].clone();
					text.push('\n');
				} else {
					break;
				}
			}
			self.start_(func_text.clone()+text.as_str());
			for name_ in args_name {
				self.remove_vars(name_.to_string());
			}
			self.remove_vars(func_name.clone());
		}
		#[warn(unreachable_code)]
		pub fn start_(&mut self, text: String) -> u8 { // возвращаем ошибку
			 let mut temp_values: String = String::new();			//	ВРЕМЕННЫЕ ПЕРЕМЕННЫЕ
			 let mut temp_name: String = String::new();		        //	...
			 let mut temp_buffer: String = String::new();			//	...
			 let mut func_text: String = String::new();
			 let mut last_op: [usize; 3] = [0; 3]; // храним три последних действия
			 // ----------------------------------------------
			 let mut if_count: usize = 0;
			 let mut if_result: bool = false; // ответ на условие
			 let mut struct_flag: bool = false; // это структура а не условие
			 let mut function_inactive_flag: bool = false;
			 for ch in text.chars() {				
				if ch == ' ' || ch == '\t' {
					if function_inactive_flag {
						if temp_values.trim() != "end_func" {							
							func_text += temp_values.clone().as_str();
							func_text.push(' ');
							temp_values = String::new();
							last_op = [0; 3];
							continue;
						} else {
							function_inactive_flag = false;
							temp_buffer = temp_name.split('(').collect::<Vec<&str>>()[1].split(')').collect::<Vec<&str>>()[0].to_string();
							temp_name = temp_name.clone().split('(').collect::<Vec<&str>>()[0].to_string();
							self.add_vars(temp_name, temp_buffer, 4);
							temp_values = String::new();
							temp_name = String::new();
							temp_buffer = String::new();
							last_op = [0; 3];
							continue;							
						}						
					}
					match temp_values.trim() {
						"print" => { 
							last_op[1] = 4;							
							temp_values = String::new();
							continue;
						},
						"object" => {
							last_op[0] = 1;
							temp_values = String::new();
							temp_name = String::new();
							temp_buffer = String::new();
						},
						"if" => {
							if if_count == 0 { 
								last_op[0] = 2;
							}
							if_count += 1;
							temp_values = String::new();
							temp_name = String::new();
							temp_buffer = String::new();
						},
						"end" => {
							//println!("if_count: {}", if_count);
							if if_count != 0 {
								if_count -= 1; 
								last_op[0] = 2; //panic!("if_count: {}", if_count);
								temp_values = String::new();
								temp_name = String::new();
								temp_buffer = String::new();
								continue;
							}
							if if_count == 0 && !struct_flag {
								//panic!("if_result: {}", if_result);
								if_result = false;
								temp_values = String::new();
								temp_name = String::new();
								temp_buffer = String::new();
								last_op = [0; 3];
								continue;
							}
						},
						//------------------
						"func" => {
							func_text.push('\n');
							func_text += "func";
							func_text.push(' ');
							last_op[0] = 4;
							last_op[1] = 17;
							temp_values = String::new();
							temp_name = String::new();
							temp_buffer = String::new();							
						},
						"break" => {
							last_op[0] = 18;
							//panic!("");
						},
						"prt_stact" => {
							self.println();
						},
						//------------------
						_ => {
							temp_values.push(ch);
						},
					}
				} else if ch == '\n' {
					//self.println(); 
					//println!("temp_values: {:?}", temp_values); println!("temp_name: {:?}", temp_name); println!("last_op: {:?}", last_op);
					//println!("if_count: {:?}", if_count); println!("if_result: {:?}", if_result);
					//println!("-------------------------------------------");
					//-------------------------------------------
					if function_inactive_flag {
						if temp_values.trim() != "end_func" {
							func_text += temp_values.clone().as_str();
							func_text.push('\n');							
							temp_values = String::new();
							last_op = [0; 3];
							continue;
						} else {
							func_text += "end_func";
							func_text.push('\n');
							//println!("func_text: \n{}", func_text.clone());
							//panic!("asd");
							function_inactive_flag = false;
							temp_buffer = temp_name.split('(').collect::<Vec<&str>>()[1].split(')').collect::<Vec<&str>>()[0].to_string();
							temp_name = temp_name.clone().split('(').collect::<Vec<&str>>()[0].to_string();
							self.add_vars(temp_name, temp_buffer, 4);
							temp_values = String::new();
							temp_name = String::new();
							temp_buffer = String::new();
							last_op = [0; 3];
							continue;							
						}						
					}
					if (!if_result) && (if_count > 0) && last_op[0] != 2 {
						match Words::trim(temp_values.clone()).as_str() {
							"end" => {
								if if_count > 0 {
									if_count -= 1; 
									last_op[0] = 2;
								}
								if if_count == 0 && !struct_flag {
									//panic!("if_result: {}", if_result);
									if_result = false;
								}
							},
							_ => {
								//panic!("temp_values: {}", temp_values);
							},
						}
						temp_values = String::new();
						temp_name = String::new();
						temp_buffer = String::new();
						last_op = [0; 3];
						continue;
					}
					//--------------------------------------
					if last_op[1] == 4 {
						//search_var(&self, var_name: String) -> (String, usize, bool)
						let (value, type_, temp_) = self.search_var(temp_values.clone().trim().to_string());
						//self.println();
						//panic!("temp_: {}", temp_);
						if temp_ {
							println!("{}", value);
						} else {
							if temp_values.clone().split('\"').collect::<Vec<&str>>().len() > 1 {
								temp_values = temp_values.split('\"').collect::<Vec<&str>>()[1].to_string();
							} else {
								temp_values = temp_values.clone().trim().to_string();
							}
							println!("{}", temp_values);
						}						
						temp_values = String::new();
						temp_name = String::new();
						temp_buffer = String::new();
						last_op = [0; 3]
					} else if last_op[0] == 1 && last_op[1] != 17 {
						if last_op[1] != 1 {
							temp_name = temp_values.clone();
							//add_vars(&mut self, vars_name: String, vars_value: String, vars_type: usize)
							self.add_vars(temp_name.clone(), "".to_string(), 1);
							temp_values = String::new();
							temp_name = String::new();
							temp_buffer = String::new();
							last_op = [0; 3]
						} else {
							self.add_vars(temp_name.clone(), temp_values.clone(), 1);
							temp_values = String::new();
							temp_name = String::new();
							temp_buffer = String::new();
							last_op = [0; 3]
						}
					} else if last_op[0] == 1 && last_op[1] == 17 {
						if Words::is_math(temp_values.clone()) != true {
							self.set_value(temp_name.clone(), temp_values.clone());
						} else {
							// матан
							let r: String = self.math_work(temp_values.clone());
							let mut _temp_var: Vec<char> = Vec::new();
							for ch in r.chars() {
								_temp_var.push(ch.clone());
							}
							let mut result: f32 = Words::eval(_temp_var);/*
							let a: &str = r.as_str();
							let prompt = match CString::new(a){
								Ok(A) => { A },
								Err(e)=> { panic!("приводимая строка имеет неправильный вид"); CString::new("").unwrap() },
							};
							//unsafe {
							let rl_prompt = prompt.as_ptr();
							let mut result: f32 = 0.0;
							unsafe {
								result = eval(rl_prompt);
							}*/
							self.set_value(temp_name.clone(), result.to_string());
						}
						temp_values = String::new();
						temp_name = String::new();
						temp_buffer = String::new();
						last_op = [0; 3]
					} else if last_op[0] == 4 && last_op[1] == 17 {
						function_inactive_flag = true;
						func_text += temp_values.clone().as_str();
						func_text.push('\n');
						temp_name = temp_values.clone();
						temp_values = String::new();
					} else if last_op[0] == 0 && last_op[1] == 0 {
						if temp_values.trim().split('(').collect::<Vec<&str>>().len() > 1 {
							temp_name = temp_values.trim().split('(').collect::<Vec<&str>>()[0].to_string();
							temp_buffer = temp_values.trim().split('(').collect::<Vec<&str>>()[1]
											.split(')').collect::<Vec<&str>>()[0].to_string();
							//println!("text: {:?}", text);
							self.func_work(/*text.clone(),*/ temp_name.clone(), temp_buffer.clone(), func_text.clone());
							temp_values = String::new();
							temp_name = String::new();
							temp_buffer = String::new();
							last_op = [0; 3];
							continue;
						}
						match Words::trim(temp_values.clone()).as_str() {
							"end" => {
								temp_values = String::new();
								temp_name = String::new();
								temp_buffer = String::new();
								last_op = [0; 3];
								continue;		
							},
							"break" => {
								return 0;
							},
							_ => {
								//panic!("temp_values: {}", temp_values);
							},
						}
							
					} else if last_op[0] == 2 {
						//println!("temp_values: {}", temp_values.clone()+"\n");
						temp_values = self.math_work(temp_values.clone());
						// конвертирует строку с переменными в строку
						//fn math_work(&self, text: String) -> String {
						if_result = self.if_work(temp_values.clone()+"\n");
						if if_result {
							if_count -= 1;
						}
						//println!("if_result: {}", if_result);
						temp_values = String::new();
						temp_name = String::new();
						temp_buffer = String::new();
						last_op = [0; 3];
					} else if last_op[0] == 18 {
						//panic!("");
						//println!("temp_values: {}", temp_values);
						return 0;
					}
				} else if ch == '=' {
					if last_op[0] == 1 {
						last_op[1] = 1;
						temp_name = temp_values.clone().trim().to_string();
						temp_values = String::new();
					} else if last_op[0] == 0 && !function_inactive_flag{
						let (value, type_, temp_) = self.search_var(temp_values.clone().trim().to_string());
						if temp_ { 
							last_op[0] = 1; // нашли переменную
							last_op[1] = 17; // присваиваем текст
							temp_name = temp_values.clone().trim().to_string();
						}
						temp_values = String::new();
					} else if last_op[0] == 2 {
						temp_values.push(ch);
					} else { 
						temp_values.push(ch);
					}
				} else {
					temp_values.push(ch);
				}
				
			 }
			 return 0;
		}
		fn eval(str_: Vec<char>) -> f32 {
			let mut i: usize = 0;
			Words::expr(str_, &mut i)
		}

		fn plus_one(u: &mut usize) {
			*u += 1;
		}

		fn number(ch_: Vec<char>, idx: &mut usize) -> f32 {
			let mut result: f32 = 0.0;
			//float result = 0.0;
			let mut div: f32 = 10.0;
			let mut sign: f32 = 1.0;
			if ch_[*idx] == '-'{
				sign = -1.0;
				*idx += 1;
			}
			
			while *idx < ch_.len() &&
				match ch_[*idx] {
					'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => { true },
					_ => { false }
				}
			{
				result = result * 10.0 + (f32::from_str(&ch_[*idx].to_string()).expect("не удалось форматировать строку"));
				
				*idx += 1;
			}
			
			if *idx < ch_.len() && (ch_[*idx] == '.'){
				*idx += 1;        
				while *idx < ch_.len() &&
					match ch_[*idx] {
						'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => { true },
						_ => { false }
					} 
				{
					result = result + (f32::from_str(&ch_[*idx].to_string()).expect("не удалось форматировать строку")) / div;
					div *= 10.0;
					*idx += 1;
				}
			}
			sign * result
		}

		fn expr(ch_: Vec<char>, idx: &mut usize) -> f32 {
			let mut result: f32 = Words::term(ch_.clone(), idx);    
			while *idx < ch_.len() && (ch_[*idx] == '+' || ch_[*idx] == '-') {
				match ch_[*idx] {
					'+' => {
						*idx += 1;
						result += Words::term(ch_.clone(), idx);
					},
					'-' => {
						*idx += 1;    
						result -= Words::term(ch_.clone(), idx);
					},
					_ => {},
				} 
			} result
		}

		fn term(ch_: Vec<char>, idx: &mut usize) -> f32 {
			let mut result: f32 = Words::factor(ch_.clone(), idx);
			let mut div: f32 = 0.0;
		
			while *idx < ch_.len() && (ch_[*idx] == '*' || ch_[*idx] == '/') {
				match ch_[*idx] {
					'*' => {
						*idx += 1;
						result *= Words::factor(ch_.clone(), idx);
					},
					'/' => {
						*idx += 1;    
						div = Words::factor(ch_.clone(), idx);    
						if (div != 0.0) {
							result /= div;
						} else {
							panic!("Division by zero!\n");                    
						}
					},
					_ => {},
				}
			} result
		}
		
		fn factor(ch_: Vec<char>, idx: &mut usize) -> f32 {
			let mut result: f32 = 0.0;
			let mut sign: f32 = 1.0;
		
			if (ch_[*idx] == '-') {
				sign = -1.0;
				*idx += 1;
			}
		
			if (ch_[*idx] == '(') {
				*idx += 1;
				result = Words::expr(ch_.clone(), idx);
		
				if (ch_[*idx] != ')') {
					panic!("Brackets unbalanced!\n");
				}
				*idx += 1;
			} else { result = Words::number(ch_, idx); }
			/*if (ch_[*idx] == '^')
			{
				*idx += 1;
		
				result = pow(result, factor(ch_, idx));
			}*/
			sign * result
		}
	}
}