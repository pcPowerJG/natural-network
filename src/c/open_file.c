#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define int64_t long int


size_t symbol_count(char* path) {
    FILE *fp;

    size_t count_ = 0;

    if ((fp = fopen(path, "r")) == NULL) {
        printf("Не удалось открыть файл");
        getchar();
        return 0;
    }
    while(fgetc(fp) != EOF) {
	count_ += 1;
    }

    fclose(fp);
    return count_;
}

char* read_from_file(char* file_name, size_t symbol_counts){
    FILE *fp;
    //char* file_name = "extern.c";
    //__int64_t coun_char = 0;
    //char name[] = "my.txt";
    if ((fp = fopen(file_name, "r")) == NULL) {
        printf("Не удалось открыть файл");
        return "exit(-2)\n";
    }

    int64_t i = 0;
   	char* text_in_file = (char*) malloc(sizeof(char) * symbol_counts);
	char ch;
	while((ch = fgetc(fp)) != EOF) {
		text_in_file[i] = ch;
		i += 1;
	}
    fclose(fp);
    return text_in_file;
}




char* read_file(char* path) {
	size_t count_ = symbol_count(path);
	return read_from_file(path, count_);
}



/*
int main(){
	size_t count_ = symbol_count("extern.c");
	printf("\nreturn: %s", read_from_file("extern.c", count_));
	return 0;
}*/