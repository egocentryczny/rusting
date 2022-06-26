fn main() {
    let x = fibonacci_generator(10);
    println!("X {} ", x);
}


fn fibonacci_generator(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    fibonacci_generator(n-1) + fibonacci_generator(n-2)
}