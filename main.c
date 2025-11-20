#include <stdio.h>

extern int func();

int main() {
    int res1 = func();
    printf("result 'func': %d\n", res1);
    return 0;
}