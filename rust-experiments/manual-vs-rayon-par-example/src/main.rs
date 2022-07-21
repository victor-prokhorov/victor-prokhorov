#![feature(test)]
#![allow(dead_code)]

extern crate test;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

fn main() {
    let input: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6]];

    assert_eq!(sum_of_squares_rayon_par(&input), 91);
    assert_eq!(sum_of_squares_seq(&input), 91);
    assert_eq!(sum_of_squares_par_manual(&input), 91);
}

fn sum_of_squares_rayon_par(input: &Vec<Vec<i32>>) -> i32 {
    let threads_ids = Arc::new(Mutex::new(Vec::with_capacity(2)));
    let result = input
        .par_iter()
        .map(|i| {
            let id = std::thread::current().id();
            let threads_ids = Arc::clone(&threads_ids);
            threads_ids.lock().unwrap().push(id);
            i.iter().map(|&i| i * i).sum::<i32>()
        })
        .sum();

    let threads_ids = threads_ids.lock().unwrap();
    println!("{:?}", threads_ids);
    // cargo bench -- --nocapture
    // will fail following assertion
    // TODO: try with proper benchmarking lib
    // assert_ne!(threads_ids[0], threads_ids[1]);
    result
}

fn sum_of_squares_seq(input: &Vec<Vec<i32>>) -> i32 {
    let mut threads_ids = Vec::with_capacity(2);
    let result = input
        .iter()
        .map(|i| {
            let id = std::thread::current().id();
            threads_ids.push(id);
            i.iter().map(|&i| i * i).sum::<i32>()
        })
        .sum();
    assert_eq!(threads_ids[0], threads_ids[1]);
    println!("{:?}", threads_ids);
    result
}

fn sum_of_squares_par_manual(input: &Vec<Vec<i32>>) -> i32 {
    let threads_ids = Arc::new(Mutex::new(Vec::<std::thread::ThreadId>::with_capacity(2)));

    // TODO:
    // find solution to borrow
    // this is very clearly a suboptimal solution
    let input = Arc::new(input.clone());
    // https://stackoverflow.com/questions/28599334/how-do-i-run-parallel-threads-of-computation-on-a-partitioned-array
    // let input = Arc::new(input);

    let mut handles = vec![];

    for i in 0..input.len() {
        let threads_ids = Arc::clone(&threads_ids);
        let input = Arc::clone(&input);
        handles.push(std::thread::spawn(move || {
            let id = std::thread::current().id();
            threads_ids.lock().unwrap().push(id);
            input[i].iter().map(|&i| i * i).sum::<i32>()
        }));
    }
    let mut result = 0;
    for handle in handles {
        let r = handle.join().unwrap();
        result += r;
    }
    let threads_ids = threads_ids.lock().unwrap();
    assert_ne!(threads_ids[0], threads_ids[1]);
    println!("{:?}", threads_ids);
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn sum_of_squares_rayon_par_bench(b: &mut Bencher) {
        let input: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6]];
        b.iter(|| assert_eq!(sum_of_squares_rayon_par(&input), 91))
    }

    #[bench]
    fn sum_of_squares_seq_bench(b: &mut Bencher) {
        let input: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6]];
        b.iter(|| assert_eq!(sum_of_squares_seq(&input), 91))
    }

    #[bench]
    fn sum_of_squares_par_manual_bench(b: &mut Bencher) {
        let input: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6]];
        b.iter(|| assert_eq!(sum_of_squares_par_manual(&input), 91))
    }
}
