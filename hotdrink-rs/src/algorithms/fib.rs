/// Computes the nth number in the Fibonacci series, but slowly.
pub fn slow_fib(n: i32) -> i32 {
    if n <= 1 {
        n
    } else {
        slow_fib(n - 1) + slow_fib(n - 2)
    }
}
