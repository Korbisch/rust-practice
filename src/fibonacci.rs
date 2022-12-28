pub fn fibonacci(n: usize) -> u128 {
    let mut fib = vec![0, 1];

    for i in 2..n+1 {
        fib.push(fib[i-1] + fib[i-2]);
    }
    fib[n]
}
