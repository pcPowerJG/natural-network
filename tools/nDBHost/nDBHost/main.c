#define _WINSOCK_DEPRECATED_NO_WARNINGS
#define _CRT_SECURE_NO_WARNINGS

//#include <WinSock2.h>
#include <stdio.h>
#include <conio.h>
#include <stdlib.h>
//---------------------
#include <malloc.h>
// free(указатель); // освобождаем память
// int sizeof(тип); // размер в байтах
//---------------------

// codenet.ru/progr/os/
#include <WinSock2.h>

#define i32 int
#define f32 float
#define usize unsigned int


int serv() {
	WSADATA ws;
	//...
	if (FAILED(WSAStartup(MAKEWORD(1, 1), &ws)))
	{
		// Error...
		//error = WSAGetLastError();
		//...
	}
	else {
		SOCKET s;
		if (INVALID_SOCKET == (s = socket(AF_INET, SOCK_STREAM, 0)))
		{
			// Error...
			//error = WSAGetLastError();
			printf("error socket 1");
			// ... 
		}
		else {
			// Объявим переменную для хранения адреса 
			SOCKADDR_IN address;

			// Заполним ее:
			//ZeorMemory(&address, sizeof(address));
			// тип адреса (TCP/IP)
			address.sin_family = AF_INET;
			//адрес сервера. Т.к. TCP/IP представляет адреса в числовом виде, то для перевода 
			// адреса используем функцию inet_addr.
			address.sin_addr.S_un.S_addr = inet_addr("127.0.0.1");//ip fast-table technologi
			// Порт. Используем функцию htons для перевода номера порта из обычного в //TCP/IP представление.
			address.sin_port = htons(1234);

			// Дальше выполняем соединение:
			if (SOCKET_ERROR == (connect(s, (SOCKADDR *)&address, sizeof(address))))
			{
				// Error...
				//error = WSAGetLastError();
				printf("error socket 2");
				// ... 
			}
			else {

			}
		}
	}
	return 0;
}


int main() {
	


	getchar();
	getchar();
	return 0;
}














int malList()
{/*
 int *a;  // указатель на массив
 int i, n;
 system("chcp 1251");
 system("cls");
 printf("Введите размер массива: ");
 //	scanf("%d", &n);
 // Выделение памяти
 a = (int*)malloc(n * sizeof(int));
 // Ввод элементов массива
 for (i = 0; i<n; i++)
 {
 printf("a[%d] = ", i);
 //		scanf("%d", &a[i]);
 }
 // Вывод элементов массива
 for (i = 0; i<n; i++)
 printf("%d ", a[i]);
 free(a);
 getchar();   getchar(); */
	return 0;
}