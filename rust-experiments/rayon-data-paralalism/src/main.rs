#![feature(test)]
extern crate test;
use std::{thread, time};

fn main() {
    fibonacci_split_iterative()
}

const N: u32 = 5;
const FN: u32 = 5;

fn fib_iterative(n: u32) -> u32 {
    println!("fib_iterative - n - {n}");
    let mut a = 0;
    let mut b = 1;
    for i in 0..n {
        println!("i - {i}");
        let c = a + b;
        a = b;
        b = c;
    }
    println!("a - {a}");
    a
}

fn fibonacci_split_iterative() {
    fn fib(n: u32) -> u32 {
        use rayon::iter::ParallelIterator;

        rayon::iter::split(n, |n| {
            println!("rayon::iter::split - n - {n}");

            if n == 3 {
                println!("sleep");
                let ten_millis = time::Duration::from_secs(3);
                let now = time::Instant::now();
                thread::sleep(ten_millis);
                assert!(now.elapsed() >= ten_millis);
            }
            if n < 2 {
                (n, None)
                // roughly
                // 3. (1, None) -> fib_iterative(1) -> 1
                //
                // 4, 2, 0 -> fib_iterative(0) -> 0
                // etc.
            } else {
                (n - 2, Some(n - 1))
                // 1. (3, 4)
                // 2. (1, 3)
            }
        })
        .map(fib_iterative)
        .sum()
    }

    assert_eq!(fib(test::black_box(N)), FN);
}
