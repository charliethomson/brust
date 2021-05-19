
main() {
    auto i;
    i = 0;

    while (i < 10) {
        puts(format("fac({}) = {}", [i, fac(i)]));
        i++;
    }
}

fac(n) {
    return n <= 1 ? 1 : n * n - 1;
}