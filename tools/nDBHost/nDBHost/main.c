#define _WINSOCK_DEPRECATED_NO_WARNINGS
#define _CRT_SECURE_NO_WARNINGS

//#include <WinSock2.h>
#include <stdio.h>
#include <conio.h>
#include <stdlib.h>
//---------------------
#include <malloc.h>
// free(���������); // ����������� ������
// int sizeof(���); // ������ � ������
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
			// ������� ���������� ��� �������� ������ 
			SOCKADDR_IN address;

			// �������� ��:
			//ZeorMemory(&address, sizeof(address));
			// ��� ������ (TCP/IP)
			address.sin_family = AF_INET;
			//����� �������. �.�. TCP/IP ������������ ������ � �������� ����, �� ��� �������� 
			// ������ ���������� ������� inet_addr.
			address.sin_addr.S_un.S_addr = inet_addr("127.0.0.1");//ip fast-table technologi
			// ����. ���������� ������� htons ��� �������� ������ ����� �� �������� � //TCP/IP �������������.
			address.sin_port = htons(1234);

			// ������ ��������� ����������:
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
 int *a;  // ��������� �� ������
 int i, n;
 system("chcp 1251");
 system("cls");
 printf("������� ������ �������: ");
 //	scanf("%d", &n);
 // ��������� ������
 a = (int*)malloc(n * sizeof(int));
 // ���� ��������� �������
 for (i = 0; i<n; i++)
 {
 printf("a[%d] = ", i);
 //		scanf("%d", &a[i]);
 }
 // ����� ��������� �������
 for (i = 0; i<n; i++)
 printf("%d ", a[i]);
 free(a);
 getchar();   getchar(); */
	return 0;
}