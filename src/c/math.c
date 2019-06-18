#include <stdio.h>
#include <stdlib.h>
#define BUF_SIZE 1024
 
// <цифра> ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
// <число> ::= <цифра> { <цифра> } [ '.' <цифра> { <цифра> } ]
// 
// <выражение> ::= <слагаемое> [ ( '+' | '-' ) <слагаемое> ]
// <слагаемое> ::= <множитель> [ ( '*' | '/' ) <множитель> ]
// <множитель> ::= ( <число> | '(' <выражение> ')' ) [ '^' <множитель> ]


int say_hello_math(int val) {
    printf("Hello From Math\n");
    return val * val;
}

float eval(char *str);
float number(char *, unsigned *);
float expr(char *, unsigned *);
float term(char *, unsigned *);
float factor(char *, unsigned *);
 
/*int main() {
    char str[BUF_SIZE];
 
    printf("Enter expression: ");
    fgets(str, BUF_SIZE, stdin);
 
    printf("Result: %lf\n", eval(str));
 
    return 0;
}*/

float eval(char *str) {
    unsigned i = 0;
 
    return expr(str, &i);
}
 
float number(char *str, unsigned *idx) {
    float result = 0.0;
    float div = 10.0;
    int sign = 1;
 
    if (str[*idx] == '-'){
        sign = -1;
        ++*idx;
    }
 
    while (str[*idx] >= '0' && str[*idx] <= '9'){
        result = result * 10.0 + (str[*idx] - '0');
        
        ++*idx;
    }
 
    if (str[*idx] == '.'){
        ++*idx;
 
        while (str[*idx] >= '0' && str[*idx] <= '9'){
            result = result + (str[*idx] - '0') / div;
            div *= 10.0;
 
            ++*idx;
        }
    }
 
    return sign * result;
}
 
float expr(char *str, unsigned *idx) {
    float result = term(str, idx);
 
    while (str[*idx] == '+' || str[*idx] == '-') {
        switch (str[*idx]) {
            case '+':
                ++*idx;
    
                result += term(str, idx);
                
                break;
            case '-':
                ++*idx;
    
                result -= term(str, idx);
    
                break;
        }
    }
 
    return result;
}
 
float term(char *str, unsigned *idx) {
    float result = factor(str, idx);
    float div;
 
    while (str[*idx] == '*' || str[*idx] == '/') {
        switch (str[*idx]) {
            case '*':
                ++*idx;
    
                result *= factor(str, idx);
    
                break;
            case '/':
                ++*idx;
    
                div = factor(str, idx);
    
                if (div != 0.0)
                {
                    result /= div;
                }
                else
                {
                    printf("Division by zero!\n");
                    exit(-1);
                }
    
                break;
        }
    }
 
    return result;
}
 
float factor(char *str, unsigned *idx) {
    float result;
    int sign = 1;
 
    if (str[*idx] == '-') {
        sign = -1;
 
        ++*idx;
    }
 
    if (str[*idx] == '(') {
        ++*idx;
 
        result = expr(str, idx);
 
        if (str[*idx] != ')')
        {
            printf("Brackets unbalanced!\n");
            exit(-2);
        }
 
        ++*idx;
    }
    else
        result = number(str, idx);
 
    /*if (str[*idx] == '^')
    {
        ++*idx;
 
        result = pow(result, factor(str, idx));
    }*/
 
    return sign * result;
}

