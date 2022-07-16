#![allow(unreachable_code)]
use std::error::Error;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

struct Point2D {
    x: i32,
    y: i32,
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point2D {
    fn print(&self) {
        println!("{} {}", self.x, self.y);
    }
}
impl Point3D {
    fn print(&self) {
        println!("{} {} {}", self.x, self.y, self.z);
    }
}

trait About {
    fn about(&self) -> &'static str;
}

impl About for Point2D {
    fn about(&self) -> &'static str {
        "2d"
    }
}

impl About for Point3D {
    fn about(&self) -> &'static str {
        "3d"
    }
}

trait Move {
    // Self will refer to the impl struct
    fn move_x_to(&mut self, x: i32) -> &Self;
}

impl Move for Point2D {
    fn move_x_to(&mut self, x: i32) -> &Self {
        self.x = x;
        self
    }
}

// we don't know which one at compile time
// we need to use `dyn`
fn new_point(d: u32) -> Box<dyn About> {
    if d == 2 {
        Box::new(Point2D { x: 0, y: 0 })
    } else {
        Box::new(Point3D { x: 0, y: 0, z: 0 })
    }
}

static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);

fn main() -> Result<(), Box<dyn Error>> {
    let mut p_2d = Point2D { x: 0, y: 0 };
    p_2d.move_x_to(1);
    p_2d.print();
    let p_3d = Point3D { x: 0, y: 0, z: 0 };
    p_3d.print();

    let boxed_p_2d = new_point(2);
    println!("{}", (*boxed_p_2d).about());
    // or the same but relying on autoderef
    println!("{}", boxed_p_2d.about());

    let v = vec![1, 2, 3, 4, 5];
    let arc_v = Arc::new(v);

    let mut joinhandles: Vec<JoinHandle<()>> = Vec::new();

    // use move to capture env with closure
    for thread_number in 0..10 {
        // we clone before moving
        let cloned_arc_v_for_thread = arc_v.clone();

        let handle = thread::spawn(move || {
            let old_thread_count = GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
            println!("live threads: {}", old_thread_count + 1);

            if thread_number == -1 {
                panic!("panic in thread 0");
                // panic inside a thread
                // will not stop other thread spawns
                // will not stop main executionuse std::sync::atomic::{AtomicUsize, Ordering};
            }

            cloned_arc_v_for_thread;

            for i in 1..10 {
                println!("tick {} thread {}", i, thread_number);
                thread::sleep(Duration::from_secs(1));
            }
        });

        joinhandles.push(handle);
    }

    let cloned_arc_v_for_main = arc_v.clone();
    for i in 1..5 {
        println!("tick {} main thread", i);
        cloned_arc_v_for_main;
        thread::sleep(Duration::from_secs(1));
    }
    // println!("before joinhandles iter");
    // println!("joinhandles: {:?}", joinhandles);
    for handle in joinhandles.into_iter() {
        // println!("hanlde: {:?}", handle);
        // handle.join().unwrap();
        if let Err(panic) = handle.join() {
            println!("Thread had an error: {panic:?}");
        }
    }

    Ok(())
}
