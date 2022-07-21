#![feature(test)]

fn main() {}

extern crate test;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_pow(b: &mut Bencher) {
        // Optionally include some setup
        let x: f64 = 211.0 * 11.0;
        let y: f64 = 301.0 * 103.0;

        b.iter(|| {
            // Inner closure, the actual test
            for i in 1..100 {
                black_box(x.powf(y).powf(x));
            }
        });
    }
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
}
