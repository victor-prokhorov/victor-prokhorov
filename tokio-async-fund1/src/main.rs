use std::time::Duration;

use tokio::time::sleep;

// TODO:
// make is testable
// share array with and write value to it
// check that the order is correct

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 5, 3, 1 at 5+3+1
    delay5().await;
    delay3().await;
    delay1().await;

    // TODO:
    // concurrently run all now
    // 1, 3, 5 at 5
    let mut handles = vec![];
    for i in 0..3 {
        // called `task`
        // same syntax as `tread`
        // we can use Arc and Mutex almost the same way
        handles.push(tokio::spawn(async move {
            match i {
                0 => delay5().await,
                1 => delay3().await,
                2 => delay1().await,
                _ => panic!(),
            }
        }));
    }
    for handle in handles {
        handle.await.unwrap();
    }

    println!("main");
    Ok(())
}

async fn delay5() {
    sleep(Duration::from_secs(5)).await;
    println!("awaited 5");
}
async fn delay3() {
    sleep(Duration::from_secs(3)).await;
    println!("awaited 3");
}
async fn delay1() {
    sleep(Duration::from_secs(1)).await;
    println!("awaited 1");
}
