# natural-network
[description]
[EN]
Hello! Who are you?
I am a person who writes a diploma and is looking for work according to his mind and abilities.

I will be glad to any help, I can be contacted in several ways:
1 - via telephone +79954226535,
2 - via Skype and e-mail ?????????,
3 - via VK https://vk.com/earth_satellite_alex.
3.1 - my helper https://vk.com/unicode_72

I want to create a new programming language designed to quickly create and modify neural networks. At the moment I am writing a kernel on Rust, I ask you for any feasible help.

Below is a complete description of the functions currently available and a general development team. Language version: 0.01 alpha.

[RU]
Привет! А кто ты?
Я человек который пишет диплом и ищет работу по уму и способностям.

Буду рад любой помощи, со мной можно связаться несколькими путями:
1 - через телефон +79954226535,
2 - через Скайп или почту ????????????????,
3 - через VK https://vk.com/earth_satellite_alex.
3.1 - моя сестра и главный помошник https://vk.com/unicode_72

Я хочу создать новый язык программирования, предназначенный для быстрого создания и изменения нейросетей. На данный момент я пишу ядро на Rust, прошу Вас о любой посильной помощи.

Ниже будет полное описание функций доступных на данный момент и общая команда разработки. Версия языка: 0.01 альфа.
[/description]

[std]
[ENG]
object -> is used to create an object that stores values in memory.

        The value of this type can be used as and anywhere.
        EXAMPLE1, using a value of type object to dynamically create a server

          serv server1 = 192.168.0.1:8085		; we create the server to accept parameters
          object server2_text = 192.168.0.1:8084	
	  
          server1 = server2_text			; change the ip and server port to the one that is in the object

remove -> removes an object, server or neuron from memory. EXAMPLE:
	
	object obj =hello
	remove obj
	
print -> prints the value of a variable

	object obj =Hello World!
	print obj					; will print 'Hello World'
[RU]
object -> используется для создания объекта, который хранит значения в памяти
		
	Значение этого типа можно использовать как и где угодно. 
		ПРИМЕР1, импользование значения типа object для динамического создания сервера 

		serv server1 = 192.168.0.1:8085			; создаём сервер, чтобы принять парамерты
		object server2_text = 192.168.0.1:8085		

		server1 = server2_text				; меняем ip и порт сервера на тот что находится в объекте 
		
remove -> любой объект, сервер или нейрон из памяти
		ПРИМЕР:
		
		object obj1 = asdasqwqt
		remove obj1
		
print -> печатает значение переменной
	
		object obj1=Hello World!
		print obj1					; напечатает Hello World

[en]
launch -> in development will allow launching the second module of the project. send -> sending the variable value to the server, waiting for the implementation of the second project module. fucnt -> allows you to create a method, the first module will be implemented after the basic arithmetic operations on objects. if / else -> after funct.


[ru]


launch -> в разработке, позволит запустить второй модуль проекта. send -> отправка значения переменной на сервер, ждёт реализации второго модуля проекта. fucnt -> позволяет создать метод, первый модуль, будет реализован после базовых арифметических операций над объектами. if/else -> после funct.
[/std]
