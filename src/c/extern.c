//#define _GNU_SOURCE
#include "dlfcn.h"
#include <stdio.h>
#include <stdlib.h>
//#include <dlfcn.h>

int mymain(int val){
    printf("Function mymain, val %i\n", val);
    return 1;
}
// "./mylib.so", libf
int linked(char *lib_path, char *func_name){
    void *handle;
    int (*mylibf)(int val);
    if ((handle=dlopen (lib_path, RTLD_LAZY)) == NULL)
        return -1;
    mylibf = dlsym(handle, func_name);
    if (dlerror() != NULL)
        return -2;
    return (int)(*mylibf)(30);
    //printf("libf addr is %p, ret value is %i\n", mylibf, (int)(*mylibf)(30));
    //return 0;
}
int close(void * handle){
    dlclose(handle);
    return 0;
}
void *open(char * lib_path) {
    void *handle;
    if ((handle=dlopen (lib_path, RTLD_LAZY)) == NULL){        
        printf("Error in lib '");
        printf(lib_path);        
        printf("' ");
        printf(": 'не удалось подключить библиотеку, проверьте путь...'\n");
        return handle;
    }
    return handle;
}
char* char_rf_func(void* handle, char *func_name, char* arg){
    char* (*mylibf)(char* val);
    mylibf = dlsym(handle, func_name);
    if (dlerror() != NULL){
        printf("Не удалось обратится к функции ");
        printf(func_name);
        printf("\n");
        return "-1";
    }
    return (char*)(*mylibf)(arg);  
}
int int_rf_char_func(void* handle, char * func_name, char* arg){
    int (*mylibf)(char* val);
    mylibf = dlsym(handle, func_name);
    if (dlerror() != NULL){
        printf("Не удалось обратится к функции ");
        printf(func_name);
        printf("\n");
        return -1;
    }
    return (int)(*mylibf)(arg);
}
int int_rf_func(void* handle, char * func_name, int arg){
    int (*mylibf)(int val);
    mylibf = dlsym(handle, func_name);
    if (dlerror() != NULL){
        printf("Не удалось обратится к функции ");
        printf(func_name);
        printf("\n");
        return -1;
    }
    return (int)(*mylibf)(arg);
}
int int_rf_void_func(void* handle, char * func_name){
    int (*mylibf)();
    mylibf = dlsym(handle, func_name);
    if (dlerror() != NULL){
        printf("Не удалось обратится к функции ");
        printf(func_name);
        printf("\n");
        return -1;
    }
    return (int)(*mylibf)();
}
int lib_call(char* lib_path, char* func_name, char* arg_func){
    void *handle;
    int (*mylibf)(char *val);
    if ((handle=dlopen (lib_path, RTLD_LAZY)) == NULL)
        return -1;
    mylibf = dlsym(handle, func_name);
    if (dlerror() != NULL)
        return -2;
    
    return (int)(*mylibf)(arg_func);
}
// строка сборки (флаги для gcc)
// gcc -o main -fPIC -ldl -rdynamic main.c; gcc -o mylib.so -fPIC -shared mylib.c
/*
int main(){
    void *handle;
    int (*mylibf)(int val);
    //void *mylibf;

    if ( (handle=dlopen ("./mylib.so", RTLD_LAZY)) == NULL )
        return 2;
    printf("if 1\n");
    mylibf = dlsym(handle, "libf");
    if ( dlerror() != NULL )
        return 2;
    printf("if 2\n");
    printf("libf addr is %p, ret value is %i\n", mylibf, (int)(*mylibf)(30));
    return 0;    
}*/