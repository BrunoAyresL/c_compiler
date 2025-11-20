int func() {
    int x = 1;
    int y = 10;
    int z = 20;
    int w = 30;
    int a = x + y + z;
    if (y > x) {
        w = 100;
    } else if (w < z) {
        w = 0;
    } 
    if (w == w) {
        x = 200;
    }
 
    w = 0;
    for (int i = 0; i < a; i = i + 1) {
        w = i;
    }

    return w;
}

int next() {

    int x = 0;
    int y = 100;
    while (x < y) {
        x = x + 2;
        y = y + 1;
    }
    return x;
}