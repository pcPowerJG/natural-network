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

pub mod Language{
    use ServersModule;
	// endpointautomate
	use std::net::*;
	pub struct Words{
		words: Vec<String>,                               //буква (номер от 1 (a-z)), слово
        neural_network: Net,        			          // сама сеть
		servers: Vec<ServersModule::Thread>,              // сервера
		//buffer_action: Vec<[usize; 3]>,                 // буффер для действий
		object_buffer: Vec<(String, usize)>,              // наименования объектов 	// (name, type) // 0 - нейрон, 1 -  объект, 2 - сервер
	    value_buffer: Vec<String>,                        // значения 
	}
	pub fn on_create()->Words{
		let mut words: Vec<String> = Vec::new(); 
		words.push("create".to_string());//1 // используется для создания нового нейрона
		/*
			ПРИМЕР:
				create one { 0.005, 0.123, 0.576 }			; создаёт нейрон с именем 'one' и значением весов: 0.005, 0.123, 0.576
				create two [3]								; создаёт нейрон с 3мя пустыми (нулевыми) весами
		*/
		words.push("server".to_string());//2   // используется для явного указания создания сервера и вывод нейросети в отдельный поток
		/*
			ПРИМЕР: 
				serv server1 = 192.168.0.1:8085 
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

			    if one == two {
					send server1	one + two: String		; отправляем ответ серверу в который помещаем сложение строк объектов one и two
					send server1	ont + two: Float		; отправляем ответ серверу в который помещаем сложение чисел объектов one и two
				}
		*/
		words.push("else".to_string());//5		// оператор условия, если первое условие не выполнилось
		/*
			ПРИМЕР: 
				serv server1 = 192.168.0.1:8085				; создаём сервер, чтобы принять парамерты
				object one = server1 [0]					; принимаем первый параметр с сервера
				object two = server1 [1]					; принимаем второй параметр с сервера

			    if one == two {								; если значение one и two равны, то
					send server1	one + two: String		; отправляем ответ серверу в который помещаем сложение строк объектов one и two
					send server1	ont + two: Float		; отправляем ответ серверу в который помещаем сложение чисел объектов one и two
				} else {
					exit_()									; иначе завершаем приложение
				}
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
		words.push("funct".to_string());//20//инициализация функции (param PARAM_NAME [PARAM_COUNT]) для приёма с сервера
		/*
			ПРИМЕР:
				param parametrs [2]							; создаём 2 параметра
				serv server1 = 192.168.0.1:8085				; создаём сервер
				server1 -> parametrs						; помещаем значения параметров в 'parametrs'

				object obj1 = parametrs [0]					; помещаем значение первого параметра в 'obj1'
				
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

		Words{ words: words,  neural_network: new(), servers: Vec::new(), object_buffer: Vec::new(), value_buffer: Vec::new() }
	}
	//impl
    pub fn clone(to: &mut Words, from: &Words){
        
    }
	impl Words{        
		pub fn get_(&mut self, text: String) -> u8 {
						
			//let mut variables_name: Vec<String> = Vec::new();
			
			//let mut object_type_string_buffer: Vec<Vec<String>> = Vec::new();		// буффер для объектов
			//let mut flag: u8 = 0;
			//-----------------------------------------------------------------------------------------------------------------
			let mut temp_values: String = String::new();			//	ВРЕМЕННЫЕ ПЕРЕМЕННЫЕ
			let mut temp_name:	 String = String::new();			//	...
			let mut temp_buffer: String = String::new();			//	...
			//let mut temp_usize_value: usize = 0;


			let mut temp_weight_vec: Vec<f32> = Vec::new();			//	...
			

			let mut last_op: [usize; 3] = [0; 3];					//  ...
			//-----------------------------------------------------------------------------------------------------------------
			for ch in text.chars() {			
				//Split(input: String, ch: char) ДЛЯ КОСЯКОВ
				//println!("ch - {:?}\n last_op - {:?}\ntemp_buffer - {:?}\ntemp_values - {:?}\nself.value_buffer.len() - {:?}\nself.object_buffer - {:?}", ch.clone(), last_op.clone(), temp_buffer.clone(), temp_values.clone(), self.value_buffer.clone().len(), self.object_buffer.clone());
				if ch == ' ' || ch == '\t' {
					if last_op[0] == 0 && last_op[1] == 0 && last_op[2] == 0 {
						let action: usize = Words::get_action_lite(self.words.clone(), temp_buffer.clone());
					//	println!("action - {:?}", action.clone());
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
							_ => {
								
							},
						} temp_buffer = String::new();		
					}
                    if last_op[1] == 15 {  
                        // none
                    } else if last_op[0] == 3 && last_op[1] == 13 {
						temp_buffer.push(ch.clone());
					} else { continue; }					 
				} else if ch == '\n' {
					// код осуществляющий работу
					if last_op[0] == 2 && last_op[1] == 15 {
                        //println!("name {}", temp_name.clone());
                        self.object_buffer.push((temp_name.clone(), 2));
                        self.value_buffer.push(temp_buffer.clone());
                       


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
                    } else if last_op[0] == 1 {
						last_op[0] = 0;	last_op[1] = 0;	last_op[2] = 0;
						continue;
					} else if last_op[0] == 3 && last_op[1] == 13 {
						self.value_buffer.push(temp_buffer.clone());
						self.object_buffer.push((temp_values.clone(), 1));	// (name, type) // 0 - нейрон, 1 -  объект, 2 - сервер
							
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
                        let mut index_first_object: usize = 0;
                        let mut index_second_object: usize = 0;
                        let mut where_two_obj: bool = false;
                        let mut two_value_betwen_space: usize = 0;
                        let temp_buffer_ = temp_buffer.clone();
                        let mut v: Vec<&str> = temp_buffer_ .as_str().split(' ').collect();

                        for item in v {
                            if item != "" {
                                two_value_betwen_space += 1;
                                //println!("!= \"\"");
                            }
                        }
                        if two_value_betwen_space < 2 {
                            temp_buffer = temp_buffer.as_str().trim().to_string();
                        for i in 0..self.object_buffer.len(){
                            if temp_values.clone() == self.object_buffer.clone()[i.clone()].0 {
                                index_first_object = i.clone();
                            } 
                            if temp_buffer.clone() == self.object_buffer.clone()[i.clone()].0 {
                                where_two_obj = true;
                                index_second_object = i.clone();
                            }
                        } } //println!("one -> {} two -> {}", temp_values.clone(), temp_buffer.clone());
                        if where_two_obj { // если объект всё же есть
                            match self.object_buffer.clone()[index_first_object].1 {
                                0 => { // neyron
                                    //index_first_object
                                    //index_second_object
                                    match self.object_buffer.clone()[index_second_object].1 { 
                                        0 => {                                             
                                            let mut index_first_object_neyron: usize = 0;
                                            let mut index_second_object_neyron: usize = 0;// номер сервера
                                            for i in 0..index_first_object.clone(){
                                                if self.object_buffer.clone()[index_first_object.clone()].1 == 0 {
                                                    index_first_object_neyron += 1;
                                                }
                                            }
                                            for i in 0..self.object_buffer.len(){ // ищем index объекта в общей "куче" значений всех объектов
                                                if temp_buffer.clone() != self.object_buffer.clone()[i.clone()].0 {
                                                               // не нашли, прибавляем смотри ниже
                                                } else if temp_buffer.clone() == self.object_buffer.clone()[i.clone()].0 {
                                                        break; // нашли, выходим из цикла
                                                }
                                                if temp_buffer.clone() != self.object_buffer.clone()[i.clone()].0 &&
                                                    self.object_buffer.clone()[i.clone()].1 == 0 {
                                                    index_second_object_neyron += 1;
                                                }
                                            }

                                            self.value_buffer[index_first_object.clone()] =
                                                self.neural_network.get_neyron_name(index_second_object_neyron);
                                            // neyron_from_string(&mut self, index: usize, st: String){
                                            self.neural_network.neyron_from_string(index_first_object_neyron.clone(),
                                                self.value_buffer[index_first_object.clone()].clone());
                                            self.value_buffer[index_first_object.clone()] = String::new();
                                        },
                                        1 => { // object
                                            let mut index_first_object_neyron: usize = 0;
                                            for i in 0..index_first_object.clone(){
                                                if self.object_buffer.clone()[index_first_object.clone()].1 == 0 {
                                                    index_first_object_neyron += 1;
                                                }
                                            }  
                                            self.neural_network.neyron_from_string(index_first_object_neyron.clone(),
                                                self.value_buffer[index_second_object.clone()].clone());
                                        },                                        
                                        _ => {  },
                                    }
                                },
                                1 => { // object                                    
                                    match self.object_buffer.clone()[index_second_object.clone()].1 { // теперь роемся во втором объекте
                                        // определяем тип
                                        0 => { // тип второго объекта - нейрон
                                            //get_neyron_name
                                            //neural_network
                                            //let mut index_second_object_data: usize = 0;
                                            let mut index_second_object_neyron: usize = 0;// номер сервера
                                            for i in 0..self.object_buffer.len(){ // ищем index объекта в общей "куче" значений всех объектов
                                                if temp_buffer.clone() != self.object_buffer.clone()[i.clone()].0 {
                                                               // не нашли, прибавляем смотри ниже
                                                } else if temp_buffer.clone() == self.object_buffer.clone()[i.clone()].0 {
                                                        break; // нашли, выходим из цикла
                                                }
                                                if temp_buffer.clone() != self.object_buffer.clone()[i.clone()].0 &&
                                                    self.object_buffer.clone()[i.clone()].1 == 0 {
                                                    index_second_object_neyron += 1;
                                                }
                                            }
                                            //println!("index_one {} index_two {}", index_first_object,
                                            //                    index_second_object.clone());
                                            let obj2 = self.neural_network.get_neyron_name(index_second_object_neyron);                                      
                                            self.value_buffer[index_first_object] = obj2;
                                            //println!("values -> {:?}", self.value_buffer.clone());
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
                                _ => {  },
                            }
                        } else { 
                            // если объект всё же не найден
                            match self.object_buffer.clone()[index_first_object].1 {
                                0 => {  // neyron
                                    let mut index_first_object_neyron: usize = 0;
                                    for i in 0..index_first_object.clone(){
                                        if self.object_buffer.clone()[index_first_object.clone()].1 == 0 {
                                            index_first_object_neyron += 1;
                                        }
                                    }  
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
                        temp_buffer = String::new(); // ВЕРНИСЬ
                        temp_values = String::new();
                        last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
                    } else if last_op[0] == 22 && last_op[1] == 0 && last_op[2] == 0 {
                        // remove
                        for i in 0..self.value_buffer.len() {
                            if self.object_buffer[i].0 == temp_values {
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
                        {
                            let t = temp_buffer.as_str().trim();
                            for i in 0..self.object_buffer.len(){
                                if t == self.object_buffer[i].0.as_str(){
                                    println!("{}", self.value_buffer[i]);
                                    break;
                                }
                            }
                        }
                        last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
                        temp_buffer = String::new();
						temp_weight_vec = Vec::new();
						temp_values = String::new();
						temp_name = String::new();
                        //return 0;
                    }
				}
				let action: usize = Words::get_action_lite(self.words.clone(), temp_buffer.clone());  
				match action {					// self.object_buffer (name, type) // 0 - нейрон, 1 -  объект, 2 - сервер
					17 => { 
						if (ch == ' ' || ch == '\t' || ch == '\n') && last_op[1] != 15 { continue; } // обработка происходит наверху. Тут на всякий случай стоит. 
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
									temp_buffer.push(ch);// включает обработку на имя (типо после имени сразу может быть '{' и т.п.)
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
						} else if last_op[0] == 1 && last_op[1] == 10{
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
                        } else {
                            match ch.clone(){
                                '=' => { 
                                    //println!("зашли");
                                    temp_values = temp_buffer.clone();
                                    //println!("{}", temp_values.clone());
                                    temp_buffer = String::new();
                                    if last_op[0] == 0 && last_op[1] == 0 && last_op[2] == 0 {
                                        last_op[0] = 17; last_op[1] = 15;
                                    }
                                }, 
							    _ => { temp_buffer.push(ch.clone()); },
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
		pub fn string_to_usize(word: String)->usize{
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
		pub fn on_error(&mut self, true_result: f32){
			let delta: f32 = true_result - self.result;
			for i in 0..self.inputs.len(){
				self.weight[i] = self.weight[i] + (self.inputs[i] * delta * self.learn_speed);				
			}
		}
		pub fn get_name(&self)->String{ self.name.clone() }

		pub fn debug(&self) -> (String, Vec<f32>, f32) { (self.name.clone(), self.weight.clone(), self.learn_speed.clone() ) }
        pub fn get_all_width(&self) -> String { 
            let mut result: String = "{ ".to_string();            
            for item in &self.weight { 
                result += item.to_string().as_str();
                result.push(' ');
            } 
            result.push('}'); result
        }
	}
	impl Net{
		pub fn debug(&self){ for item in &self.data_base { println!(" neyron -> {:?}", item.debug()); } }
        pub fn neyron_from_string(&mut self, index: usize, st: String){
            let mut v: Vec<&str> = st.as_str().split(' ').collect();
            let len = v.clone().len();
            
            let mut b1 = false;
            let mut b2 = false;

            for word in v.clone() { 
                if word == "{" { 
                    b1 = true;
                }
                if word == "}" {
                    b2 = true;
                }
            } if (b1 && b2) == false { return; }
            //println!("v -> {:?}", v.clone());
            self.data_base[index.clone()].weight = Vec::new();
            self.data_base[index.clone()].inputs = Vec::new();
            for word in v {
                if word == "}" || word == "{" || word == "" { continue; }          
                let word = word.to_string();
                let pie: f32 = match word.parse(){
				    Ok(A)=>{ A },
				    Err(e)=> { return; 0.0 },
			    };
                self.data_base[index.clone()].weight.push(pie);
                self.data_base[index.clone()].inputs.push(0.0);
            }

        }
        pub fn get_neyron_name(&self, id: usize)->String { 
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
	
	
	
	
	
}