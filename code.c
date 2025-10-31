int func(int x, int y) {
    return 2;
}

int main() {
    int x = 0;
    int y = 2 + x;
    if (func(x,y)) {
        return x > x;
    }
    int z = -1;
    return x * y - z;
}

