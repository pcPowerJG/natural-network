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

pub mod Language{
    use ServersModule;
	// endpointautomate
	use std::net::*;
    use std::fs::File;
    use std::io::prelude::*;

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
		words.push("add".to_string());//1 // используется для создания нового нейрона
		/*
			ПРИМЕР:
				add table { td1, td2, td3, tdN }			; создаёт строку таблицы				
		*/
		words.push("in".to_string());//2 // используется при поиске объекта (аналог FROM)
        words.push("search".to_string());//3 // аналог SELECT 
        words.push("param".to_string());//4 //аналог where
        words.push("remove".to_string());//5 //delete
        words.push("backup".to_string());//6 //backup
        words.push("load".to_string());//7 //load data base
        words.push("GIVE_ALL_TABLE_NAME".to_string());//8 //give all table

		Words{ words: words,  neural_network: new(), servers: Vec::new(), object_buffer: Vec::new(), value_buffer: Vec::new() }
	}
	//impl
    pub fn clone(to: &mut Words, from: &Words){
        
    }
	impl Words{        
		pub fn get_(&mut self, text: String) -> String {
						
			//let mut variables_name: Vec<String> = Vec::new();
			
			//let mut object_type_string_buffer: Vec<Vec<String>> = Vec::new();		// буффер для объектов
			//let mut flag: u8 = 0;
			//-----------------------------------------------------------------------------------------------------------------
			let mut temp_values: String = String::new();			//	ВРЕМЕННЫЕ ПЕРЕМЕННЫЕ
			let mut temp_name:	 String = String::new();			//	...
			let mut temp_buffer: String = String::new();			//	...
            let mut temp_table:  String = String::new();
            let mut temp_param:  String = String::new();
			//let mut temp_usize_value: usize = 0;


			let mut temp_weight_vec: Vec<f32> = Vec::new();			//	...
			

			let mut last_op: [usize; 3] = [0; 3];					//  ...
			//-----------------------------------------------------------------------------------------------------------------
			for ch in text.chars() {			
				//Split(input: String, ch: char)
				//println!("ch - {:?}\n last_op - {:?}\ntemp_buffer - {:?}\ntemp_values - {:?}\nself.value_buffer.len() - {:?}\nself.object_buffer - {:?}\ntemp_table - {:?}", ch.clone(), last_op.clone(), temp_buffer.clone(), temp_values.clone(), self.value_buffer.clone().len(), self.object_buffer.clone(), temp_table.clone());
				if ch == ' ' || ch == '\t' {
					if last_op[0] == 0 && last_op[1] == 0 && last_op[2] == 0 {
						let action: usize = Words::get_action_lite(self.words.clone(), temp_buffer.clone());
						//println!("action - {:?}", action.clone());
						match action {
                            /*
                                words.push("add".to_string());//1 // используется для создания нового нейрона		                        
		                        words.push("in".to_string());//2 // используется при поиске объекта (аналог FROM)
                                words.push("search".to_string());//3 // аналог SELECT 
                                words.push("param".to_string());//4 //аналог where
                                remove - 5
                            */
							1 => { // add
								last_op[0] = 1; last_op[1] = 0; last_op[2] = 0; 
                            },							
                            3 => {
                                last_op[0] = 3;
                            },                            
                            5 => {
                                last_op[0] = 5;
                            },
                            6 => { // backup
                                last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
                                temp_name = String::new();
                                temp_buffer = String::new();
                                temp_values = String::new();
                                temp_table = String::new();

                                //get_save_string
                                //println!("\n\nsave string -> {}\n\n",self.neural_network.get_save_string());
                                let mut file = match File::create("database.ngorge") {
                                    Ok(A) => { A },
                                    Err(e) => { println!("ERROR BACKUP DATABASE CRITICAL ER: {:?}", e); return "ERROR".to_string(); File::create("database.ngorge").unwrap() }
                                };
                                let v = self.neural_network.get_save_string();
                                match file.write_all(v.as_bytes()) {
                                    Ok(A) => { println!("backup ok! no error"); },
                                    Err(e) => { println!("BACKUP CRITICAL ERROR: {:?}", e); }
                                };
                            },
                            7 => { // load
                                last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
                                temp_name = String::new();
                                temp_buffer = String::new();
                                temp_values = String::new();
                                temp_table = String::new();
                                let mut file = match File::open("database.ngorge") {
                                    Ok(A) => { A },
                                    Err(e) => { println!("CRITICAL LOAD DATA_BASE: {:?}", e); return "ERROR".to_string(); File::open("database.ngorge").unwrap(); },
                                };
                                let mut contents = String::new();
                                match file.read_to_string(&mut contents) {
                                    Ok(A) => { },
                                    Err(e) => { println!("CRITICAL LOAD DATA_BASE: {:?}", e); return "ERROR".to_string(); },
                                };
                                self.get_(contents);
                            },
                            8 => {
                                last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
                                temp_name = String::new();
                                temp_buffer = String::new();
                                temp_values = String::new();
                                temp_table = String::new();
                                return self.neural_network.give_all_table();
                            },
                            17 => { continue; },                            
							_ => {  },
						} temp_buffer = String::new();		
					}
                    if last_op[1] == 15 {  
                        // none                    
                    } else if (last_op[0] == 3 || last_op[0] == 5) && last_op[1] == 0 && last_op[2] == 0 {
						temp_buffer = String::new();
                        last_op[1] = 17;
					} else if (last_op[0] == 3 || last_op[0] == 5) && last_op[1] == 17 && last_op[2] == 0 {
                        temp_values = temp_buffer;
                        temp_buffer = String::new(); last_op[1] = 255;
                    } else if (last_op[0] == 3 || last_op[0] == 5) && last_op[1] == 2 && last_op[2] == 0 {
                        temp_table = temp_buffer.clone();
                        last_op[2] = 255;
                        temp_buffer = String::new();                    
                    } else {
                        let action: usize = Words::get_action_lite(self.words.clone(), temp_buffer.clone());
						//println!("action - {:?}", action.clone());
						match action {
                            /*
                                words.push("add".to_string());//1 // используется для создания нового нейрона		                        
		                        words.push("in".to_string());//2 // используется при поиске объекта (аналог FROM)
                                words.push("search".to_string());//3 // аналог SELECT 
                                words.push("param".to_string());//4 //аналог where
                            */
							1 => { // add
								last_op[0] = 1; last_op[1] = 0; last_op[2] = 0; 
                            },
							2 => {
                                last_op[1] = 2;
                            },
                            3 => {
                                last_op[0] = 3;
                            },
                            4 => {
                                last_op[2] = 4;
                            },
                            17 => { continue; },
                            
							_ => {
								
							},
						} temp_buffer = String::new();		
                         continue; 
                    }					 
				} else if ch == '\n' {
					// код осуществляющий работу
					if last_op[0] == 3 && last_op[1] == 2 && last_op[2] == 4 {
                        // temp_buffer - после парам
                        // temp_values - что выбираем (ищем)
                        // temp_table - таблица с короторой работаем
                        // .get_rows() 
                        // neural_network
                        //get_rows(&self, 
                        //select_row: String, table_name: String, 
                        //args: Vec<usize>, values: Vec<String>)->Vec<String>
                        let tem_b = temp_buffer.clone();
                        let mut v: Vec<&str> = tem_b.as_str().split('&').collect();

                        for i in 0..v.len() { 
                            v[i] = v[i].trim();
                        }
                        
                        let c = temp_buffer.clone();
                        let mut v: Vec<&str> = c.as_str().split("&").collect();
                        let mut args: Vec<usize> = Vec::new();
                        let mut values: Vec<String> = Vec::new();
                        for item in v {
                            let temp: Vec<&str> = item.split("=").collect();
                            let arg: usize = match temp[0].parse(){
                                Ok(A) => { A },
                                Err(Er) => { break; 0 }
                            };
                            let value = temp[1].to_string();
                            args.push(arg);
                            values.push(value);
                        }

                        let result = self.neural_network.get_rows(temp_values.clone(), temp_table.clone(), 
                            args, values).0;
                        let mut r_result: String = String::new();
                        for item in result {
                            r_result += item.as_str();
                            r_result.push('\n');
                        }      
                        let len = r_result.chars().count();
                        if len > 0 {
                            r_result.remove(len-1);
                        }
                        last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
                        temp_name = String::new();
                        temp_buffer = String::new();
                        temp_values = String::new();
                        temp_table = String::new();
                        return r_result;
                    } else if last_op[0] == 5 && last_op[1] == 2 && last_op[2] == 4 {
                        // temp_buffer - после парам
                        // temp_values - что выбираем (ищем)
                        // temp_table - таблица с короторой работаем
                        // .get_rows() 
                        // neural_network
                        //get_rows(&self, 
                        //select_row: String, table_name: String, 
                        //args: Vec<usize>, values: Vec<String>)->Vec<String>
                        let tem_b = temp_buffer.clone();
                        let mut v: Vec<&str> = tem_b.as_str().split('&').collect();

                        for i in 0..v.len() { 
                            v[i] = v[i].trim();
                        }
                        
                        let c = temp_buffer.clone();
                        let mut v: Vec<&str> = c.as_str().split("&").collect();
                        let mut args: Vec<usize> = Vec::new();
                        let mut values: Vec<String> = Vec::new();
                        for item in v {
                            let temp: Vec<&str> = item.split("=").collect();
                            let arg: usize = match temp[0].parse(){
                                Ok(A) => { A },
                                Err(Er) => { break; 0 }
                            };
                            let value = temp[1].to_string();
                            args.push(arg);
                            values.push(value);
                        }

                        let mut result = self.neural_network.get_rows(temp_values.clone(), temp_table.clone(), 
                            args, values).1;
                        
                        let len_ = result.len();
                        for i in 0..len_ .clone(){
                            if len_ > 1 {
                                for k in 0..len_ .clone(){                                
                                    for j in 0..len_ .clone(){
                                        if result[k] > result[j] {
                                            let temp = result[j].clone();
                                            result[j] = result[k];
                                            result[k] = temp;
                                        }
                                    }
                                }
                            }
                        }
                        //remove_row
                        for index in result.clone() {
                            self.neural_network.remove_row(index);
                        }
                        
                        last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
                        temp_name = String::new();
                        temp_buffer = String::new();
                        temp_values = String::new();
                        temp_table = String::new();
                    } else if last_op[0] == 3 && last_op[1] == 2 && last_op[2] == 255 {
                        // temp_buffer - после парам
                        // temp_values - что выбираем (ищем)
                        // temp_table - таблица с короторой работаем
                        // .get_rows() 
                        // neural_network
                        //get_rows(&self, 
                        //select_row: String, table_name: String, 
                        //args: Vec<usize>, values: Vec<String>)->Vec<String>
                        let tem_b = temp_buffer.clone();
                        
                        let result = self.neural_network.get_all_rows(temp_values.clone(), temp_table.clone()).0;
                        let mut r_result: String = String::new();
                        for item in result {
                            r_result += item.as_str();
                            r_result.push('\n');
                        }      
                        let len = r_result.chars().count();
                        if len > 0 {
                            r_result.remove(len-1);
                        }
                        last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
                        temp_name = String::new();
                        temp_buffer = String::new();
                        temp_values = String::new();
                        temp_table = String::new();
                        return r_result;
                    }
				}
				let action: usize = Words::get_action_lite(self.words.clone(), temp_buffer.clone());  
				match action {					// self.object_buffer (name, type) // 0 - нейрон, 1 -  объект, 2 - сервер
					17 => { 
						if (ch == ' ' || ch == '\t' || ch == '\n') && last_op[1] != 15 { continue; } // обработка происходит наверху. Тут на всякий случай стоит. 
						
                        if last_op[0] == 3 && last_op[1] == 0 && last_op[2] == 0 {
                            temp_name.push(ch.clone());
                        } else if last_op[0] == 1 && last_op[1] == 17 && last_op[2] == 0 {
                            temp_buffer.push(ch);
                        } else if last_op[0] == 1 && last_op[1] == 2 && last_op[2] == 255 {
                            temp_buffer.push(ch);
                        } else if last_op[0] == 1 && last_op[1] == 0 && last_op[2] == 0 {
							//let action_char: usize = Words::get_action_lite(self.words.clone(), ch.to_string());
							match ch {
								'{' => {												
									last_op[1] = 6;
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
									
									// ВЕРНИСЬ
									self.neural_network.new_row(temp_name.clone(), temp_values.clone());
									self.value_buffer.push(temp_name.clone());
									self.object_buffer.push((temp_name.clone(), 0));	

									temp_name = String::new();
                                    temp_buffer = String::new();
									temp_values = String::new();
                                    last_op[0] = 0; last_op[1] = 0; last_op[2] = 0;
								},
								',' => {
									temp_values.push(',');
								},

								  _ => {
									temp_values.push(ch);
								},
							}
						} else {
                            match ch.clone(){
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
			//println!("{:?}\n{:?}\n{:?}", self.object_buffer, self.value_buffer.clone(), self.neural_network.debug());
            String::new() // вывод
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
	
	pub struct Net {
		data_base: Vec<Row>,		
	}
	pub struct Row{
		row_arg: Vec<String>,		
		table_name: String,
	}
	pub fn new()->Net{
		Net { data_base: Vec::new() }
	}

	impl Row {		
		pub fn get_table_name(&self)->String{ self.table_name.clone() }
		pub fn debug(&self) -> (String, Vec<String>) { (self.table_name.clone(), self.row_arg.clone() ) }
        pub fn get_row(&self) -> String { 
            let mut result: String = "{ ".to_string();            
            for item in &self.row_arg { 
                result += item.to_string().as_str();
                result.push('\t');
            } 
            result.push('}'); result
        }
        pub fn clone(&self)->Row { Row { row_arg: self.row_arg.clone(), table_name: self.table_name.clone() } }
	}
	impl Net {
		pub fn debug(&self){ for item in &self.data_base { println!(" row -> {:?}", item.debug()); } }            
		pub fn new_row(&mut self, table_name: String, row: String)->bool {
				/*
                    pub struct Row{
		                row_arg: Vec<String>,		
		                table_name: String,
	                }
                */
            let mut v: Vec<&str> = row.as_str().split(',').collect();
            let mut row_arg: Vec<String> = Vec::new();
            for item in v {
                row_arg.push(item.to_string());
            }
			let temp: Row = 
				Row { row_arg: row_arg, table_name: table_name };
			self.data_base.push(temp);
			true
		}
		pub fn remove_row(&mut self, index: usize){
			self.data_base.remove(index);
		}
        pub fn get_rows(&self, select_row: String, table_name: String, args: Vec<usize>, values: Vec<String>)->(Vec<String>, Vec<usize>) {
            //let mut result: Vec<String> = Vec::new();
            let mut table_rows: Vec<String> = Vec::new();
            let mut table_rows_usize: Vec<usize> = Vec::new();
            if select_row == "_".to_string() {
                for i in 0..self.data_base.len() {
                    let temp: Row = self.data_base[i].clone();
                    if temp.table_name == table_name {
                        let temp1 = temp.row_arg;
                       // for item_i in 0..temp1.clone().len() {
                        let mut tr_a: usize = 0;
                        for arg in args.clone() {
                            for value in values.clone() {
                                if temp1[arg] == value {
                                    tr_a += 1;
                                }
                            }
                        }
                        if tr_a >= args.len() {
                            let mut s: String = String::new();
                            for i_tem in temp1.clone() {
                                s += i_tem.as_str();
                                s.push('\t');
                            }
                            let len = s.clone().chars().count() - 1;
                            s.remove(len);
                            table_rows_usize.push(i);
                            table_rows.push(s);
                        }
                        //}
                    }
                }
            }
            ( table_rows, table_rows_usize )
        }        
        pub fn get_all_rows(&self, select_row: String, table_name: String)->(Vec<String>, Vec<usize>){
            let mut table_rows: Vec<String> = Vec::new();
            let mut table_rows_usize: Vec<usize> = Vec::new();
            if select_row == "_".to_string() {
                for i in 0..self.data_base.len() {
                    let temp: Row = self.data_base[i].clone();
                    if temp.table_name == table_name {
                        let temp1 = temp.row_arg;
                       // for item_i in 0..temp1.clone().len() {
                        let mut s: String = String::new();
                        for i_tem in temp1.clone() {
                            s += i_tem.as_str();
                            s.push('\t');
                        }
                        let len = s.clone().chars().count() - 1;
                        s.remove(len);
                        table_rows_usize.push(i);
                        table_rows.push(s);                        
                        //}
                    }
                }
            }
            ( table_rows, table_rows_usize )
        }
        pub fn get_save_string(&self)->String{
            let mut result: String = String::new();     
            result.push('\n');       
            for i in 0..self.data_base.len() {                
                result += "add ";
                result += self.data_base[i].table_name.as_str();
                result += " { ";
                for k in 0..self.data_base[i].row_arg.len() {
                    result += self.data_base[i].row_arg[k].as_str();
                    result.push(',');
                }
                let len = result.chars().count();
                result.remove(len-1);
                result += " }\n";
            }
            result
        }
        pub fn give_all_table(&self) -> String {
            let mut result: String = String::new();
            for i in 0..self.data_base.len(){
                result += self.data_base[i].table_name.as_str();
                result.push('\n');
            } result
        }
        pub fn len(&self)->usize{ self.data_base.len() }
        pub fn get_neyron_to_index(&self, index: u8){  }
	}
	
	
	
	
	
}