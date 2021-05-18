
algorithm(prime, num) {
    auto max_pow, result, curr_pow;

    max_pow = log(num) / log(prime);
    result = 0;
    curr_pow = 1;

    while (--max_pow) {
        result =+ num / (curr_pow * prime);
    }

    return result;
}

log(n) {
    auto r;
    r = 0;

    while (n=>>1) r++;

    return r;
}

main() {
    auto a, b, c;
    a = 7;
    b = 16;
    c = algorithm(a, b);
    puts(format("algorithm({}, {}) = {}", [a, b, c]));
}