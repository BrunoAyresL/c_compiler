
int func(int a, int b) {
    int z = a / b + 2;
    return z - 1;
}


int main() {
    int x = 0;
    int y = 10;
    int s = 25;
    if (x + 5 / 3) {
        y = y + 2;
        x = y * 5;
        s = 200;
    } else if (y < 2) {
        x = 10000 % y << s;
    } else {
        y = 999;
    }
    func(x-2, 2);
}
