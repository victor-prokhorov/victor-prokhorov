use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

// TODO:
// make is testable
// share array with and write value to it
// check that the order is correct

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _response = race().await?;
    let _response = parallel().await?;
    let _response = serial().await?;
    Ok(())
}

enum TypeOfAsync {
    Concurrent,
    Serial,
    Race,
}

async fn serial() -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    use TypeOfAsync::*;
    let mut result: Vec<i32> = vec![];
    // 5, 3, 1
    // cargo test
    // finished in 9.01s
    let r = delay(5, Serial).await;
    result.push(r);
    let r = delay(3, Serial).await;
    result.push(r);
    let r = delay(1, Serial).await;
    result.push(r);

    return Ok(result);
}
async fn parallel() -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    use TypeOfAsync::*;

    let result: Vec<i32> = vec![];
    let arc = Arc::new(Mutex::new(result));

    // TODO:
    // concurrently run all now
    // 1, 3, 5
    // cargo test
    // finished in 5.00s
    let mut handles = vec![];
    for i in 0..3 {
        let a = arc.clone();
        // called `task`
        // same syntax as `thread`
        // we can use Arc and Mutex almost the same way
        handles.push(tokio::spawn(async move {
            let n = match i {
                0 => delay(5, Concurrent).await,
                1 => delay(3, Concurrent).await,
                2 => delay(1, Concurrent).await,
                _ => panic!(),
            };
            let mut guard = a.lock().unwrap();
            let v = &mut *guard;
            v.push(n);
        }));
    }
    for handle in handles {
        handle.await.unwrap();
    }
    let guard = arc.lock().unwrap();
    // deref to get value and not mutex
    // borrow and not move then clone
    let v = &*guard;
    Ok(v.to_vec())
}

// TODO:
// label should be wraped in Some
async fn delay(delay: u64, label: TypeOfAsync) -> i32 {
    sleep(Duration::from_secs(delay)).await;
    match label {
        TypeOfAsync::Serial => println!("serial{}", delay),
        TypeOfAsync::Concurrent => println!("concurrent{}", delay),
        TypeOfAsync::Race => println!("race{}", delay),
    }
    delay.try_into().unwrap()
}

async fn race() -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut result: Vec<i32> = vec![];
    let out: i32 = tokio::select! {
        val = delay(5, TypeOfAsync::Race) => val,
        val = delay(3, TypeOfAsync::Race) => val,
        Ok(v) = async {
            let res = delay(1, TypeOfAsync::Race).await;
            Ok::<_, io::Error>(res)
        } => v
        // val = delay(1, TypeOfAsync::Race) => val,
        // else => {}
    };
    result.push(out);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn race_test() {
        let response = race().await.unwrap();
        assert_eq!(response[0], 1);
    }

    #[tokio::test]
    async fn tasks_was_run_concurently() {
        let response = parallel().await.unwrap();
        assert_eq!(response[0], 1);
        assert_eq!(response[1], 3);
        assert_eq!(response[2], 5);
    }

    #[tokio::test]
    async fn tasks_was_run_serial() {
        let response = serial().await.unwrap();
        assert_eq!(response[0], 5);
        assert_eq!(response[1], 3);
        assert_eq!(response[2], 1);
    }
}
