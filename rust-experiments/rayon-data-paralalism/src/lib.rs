#![feature(test)]
#![allow(dead_code)]

extern crate test;

const N: u32 = 32;
const FN: u32 = 2_178_309;

fn add_two(n: i32) -> i32 {
    n + 2
}
fn fib_iterative(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }
    a
}

fn fib_recursive(n: u32) -> u32 {
    if n < 2 {
        return n;
    }

    fib_recursive(n - 1) + fib_recursive(n - 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
    }

    #[bench]
    fn bench_another(b: &mut Bencher) {
        b.iter(|| {
            println!("another bench");
            0
        });
    }
    #[bench]
    /// Compute the Fibonacci number recursively, without any parallelism.
    fn fibonacci_recursive(b: &mut test::Bencher) {
        b.iter(|| assert_eq!(fib_recursive(test::black_box(N)), FN));
    }

    #[bench]
    /// Compute the Fibonacci number recursively, using rayon::join.
    /// The larger branch F(N-1) is computed first.
    fn fibonacci_join_1_2(b: &mut test::Bencher) {
        fn fib(n: u32) -> u32 {
            if n < 2 {
                return n;
            }

            let (a, b) = rayon::join(|| fib(n - 1), || fib(n - 2));
            a + b
        }

        b.iter(|| assert_eq!(fib(test::black_box(N)), FN));
    }

    #[bench]
    /// Compute the Fibonacci number recursively, using rayon::join.
    /// The smaller branch F(N-2) is computed first.
    fn fibonacci_join_2_1(b: &mut test::Bencher) {
        fn fib(n: u32) -> u32 {
            if n < 2 {
                return n;
            }

            let (a, b) = rayon::join(|| fib(n - 2), || fib(n - 1));
            a + b
        }

        b.iter(|| assert_eq!(fib(test::black_box(N)), FN));
    }

    #[bench]
    /// Compute the Fibonacci number recursively, using rayon::iter::split to parallelize.
    fn fibonacci_split_recursive(b: &mut test::Bencher) {
        fn fib(n: u32) -> u32 {
            use rayon::iter::ParallelIterator;

            rayon::iter::split(n, |n| {
                if n < 2 {
                    (n, None)
                } else {
                    (n - 2, Some(n - 1))
                }
            })
            .map(fib_recursive)
            .sum()
        }

        b.iter(|| assert_eq!(fib(test::black_box(N)), FN));
    }

    #[bench]
    /// Compute the Fibonacci number iteratively, using rayon::iter::split to parallelize.
    fn fibonacci_split_iterative(b: &mut test::Bencher) {
        fn fib(n: u32) -> u32 {
            use rayon::iter::ParallelIterator;

            rayon::iter::split(n, |n| {
                if n < 2 {
                    (n, None)
                } else {
                    (n - 2, Some(n - 1))
                }
            })
            .map(fib_iterative)
            .sum()
        }

        b.iter(|| assert_eq!(fib(test::black_box(N)), FN));
    }

    #[bench]
    /// Compute the Fibonacci number iteratively, just to show how silly the others
    /// are. Parallelism can't make up for a bad choice of algorithm.
    fn fibonacci_iterative(b: &mut test::Bencher) {
        b.iter(|| assert_eq!(fib_iterative(test::black_box(N)), FN));
    }
}
