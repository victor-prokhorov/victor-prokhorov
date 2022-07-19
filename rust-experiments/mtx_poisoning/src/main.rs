use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;

const N: usize = 10;

fn main() {
    let data = Arc::new(Mutex::new(0));

    // mutex poisoning recovery
    let lock = Arc::new(Mutex::new(0_u32));
    let lock2 = Arc::clone(&lock);
    let _ = thread::spawn(move || -> () {
        let _guard = lock2.lock().unwrap();
        panic!();
    })
    .join();
    let mut guard = match lock.lock() {
        Ok(guard) => guard,
        Err(poison_err) => {
            println!("{:?}", poison_err);
            poison_err.into_inner()
        }
    };
    *guard += 1;
    // recovered

    let (tx, rx) = channel();
    for _ in 0..N {
        let (data, tx) = (Arc::clone(&data), tx.clone());
        thread::spawn(move || {
            let mut data = data.lock().unwrap();

            *data += 1;
            if *data == N {
                tx.send(()).unwrap();
            }
            // manually drop
            // std::mem::drop(data);
            drop(data);
            // the lock is unlocked here when `data` goes out of scope.
        });
    }

    assert_ne!(*data.lock().unwrap(), N);

    rx.recv().unwrap();

    assert_eq!(*data.lock().unwrap(), N);

    println!("{}", *data.lock().unwrap());
}
