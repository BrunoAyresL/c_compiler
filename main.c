#include <stdio.h>

extern int func();
extern int next();

int main() {
    int res1 = func();
    printf("result 'func': %d\n", res1);
    int res2 = next();
    printf("result 'next': %d\n", res2);
    return 0;
}