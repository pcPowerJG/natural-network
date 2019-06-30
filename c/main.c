#define _GNU_SOURCE
#include <dlfcn.h>
#include <stdio.h>
#include <stdlib.h>
//#include <dlfcn.h>

int mymain(int val){
    printf("Function mymain, val %i\n", val);
    return 1;
}
// "./mylib.so", libf
int linked(char * lib_path, char * lib_name){
    void *handle;
    int (*mylibf)(int val);
    //void *mylibf;

    if ( (handle=dlopen (lib_path, RTLD_LAZY)) == NULL )
        return 2;
    //printf("if 1\n");
    mylibf = dlsym(handle, lib_name);
    if ( dlerror() != NULL )
        return 2;
    //printf("if 2\n");
    return (int)(*mylibf)(30);
    //printf("libf addr is %p, ret value is %i\n", mylibf, (int)(*mylibf)(30));
    //return 0;
}
// gcc -o main  -fPIC -ldl -rdynamic main.c; gcc -o mylib.so -fPIC -shared mylib.c
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
}