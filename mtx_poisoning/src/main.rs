use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;

const N: usize = 10;
fn main() {
    let data = Arc::new(Mutex::new(0));

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
            std::mem::drop(data);
            // the lock is unlocked here when `data` goes out of scope.
        });
    }

    assert_ne!(*data.lock().unwrap(), N);

    rx.recv().unwrap();

    assert_eq!(*data.lock().unwrap(), N);

    println!("{}", *data.lock().unwrap());
}
