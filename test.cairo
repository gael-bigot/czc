func fibonacci(n) {
    if (n == 0) {
        return 0;
    }
    if (n == 1) {
        return 1;
    }
    let a = fibonacci(n-1);
    let b = fibonacci(n-2);
    return a + b;
}

func main() {
    let n = fibonacci(0);
    assert n = 0;
    let n = fibonacci(1);
    assert n = 1;
    let n = fibonacci(7);
    assert n = 13;
    let n = fibonacci(10);
    assert n = 55;
    return ();
}