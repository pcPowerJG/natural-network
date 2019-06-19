#define _GNU_SOURCE                                                             
#include <stdio.h>                                                              
#include <dlfcn.h>

int libf(int val) {
    int (*mymainf)(int val);

    printf("Function libf, val %i\n", val);
    mymainf=dlsym( RTLD_DEFAULT, "mymain" );
    if ( dlerror() != NULL )
        return 33;
    printf("libf addr is %p, ret value is %i\n", mymainf, (int)(*mymainf)(60));
    return 10;    
}
int say_hello_world(){
    printf("Hello, World!\n");
}

int main(){
    return 0;
}