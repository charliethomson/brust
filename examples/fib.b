
main() {
    auto i;
    i = 0;
    while ( i<10 ) {
        puts(format("fib({}) = {}", [i, fib(i)]));
        i++;
    }
}

fib(i) {
    return i <= 1 ? i : fib(i-1) + fib(i-2);
}