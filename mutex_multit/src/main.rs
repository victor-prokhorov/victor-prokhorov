use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::Builder::new()
            .name(format!("thread{}", i))
            .spawn(move || {
                let mut num = counter.lock().unwrap();

                println!(
                    "from {:?} counter is {}, but i is {i}",
                    thread::current().name().unwrap(),
                    *num
                );

                *num += 1;
            });

        handles.push(handle.unwrap());
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
