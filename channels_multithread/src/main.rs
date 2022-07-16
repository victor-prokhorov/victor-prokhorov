use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const NO_CHANNELS: usize = 5;
const NUMBERS_LEN: i32 = 100;

fn main() {
    let numbers: Vec<i32> = (0..NUMBERS_LEN).collect();
    let arc = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    let (tx, rx) = mpsc::channel();

    // arc
    for _ in 0..8 {
        let arc_clone = arc.clone();
        joinhandles.push(thread::spawn(move || {
            // double reference |x| **x or |&x| *x or |&&x| x
            // let sum: i32 = clone.iter().filter(|&n| *n != 0).sum();
            let vec: Vec<i32> = arc_clone.iter().map(|&n| n + 100).collect();
            assert_eq!(vec[0], arc_clone[0] + 100);
            println!("{}", vec[0]);
        }));
    }

    // https://users.rust-lang.org/t/passing-channels-to-threads/48808/5
    // channel
    for _ in 0..NO_CHANNELS {
        let arc_clone = arc.clone();
        let tx = tx.clone();
        thread::spawn(move || {
            // tx.send(i.to_string()).unwrap();
            let vec: Vec<i32> = arc_clone.iter().map(|&n| n + 100).collect();
            for val in vec.iter() {
                tx.send(val.to_string()).unwrap();
                thread::sleep(Duration::from_millis(1));
            }
        });
    }

    // iter is blocking as recv
    // iter return None instead of panic
    // as opposite to try_recv (you have to create your own routine to check)
    for received in rx.iter().take(NO_CHANNELS * NUMBERS_LEN as usize) {
        println!("Got: {}", received);
    }
    println!("after rx iter");

    for handle in joinhandles.into_iter() {
        if let Err(_) = handle.join() {
            println!("panic");
        }
    }
    println!("after joinhandles into_iter");
}
