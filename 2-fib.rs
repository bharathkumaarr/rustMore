fn main() {
    println!("{}", fib(4));
}

fn fib(n:u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    if n == 0 {
        return first;
    }
    if n == 1 {
        return second;
    }

    for _ in 0..n-1 {
        let temp = second;
        second = first + second;
        first = temp;
    }
    return second;
}
