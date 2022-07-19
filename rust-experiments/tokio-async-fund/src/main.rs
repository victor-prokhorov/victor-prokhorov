//use std::sync::{Arc, Mutex};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    // netcat 127.0.0.1 8080
    let arr = [[0; 1024]; 3];

    let history = Arc::new(Mutex::new(arr));

    loop {
        println!("enterring loop");
        let (mut socket, _) = listener.accept().await?;
        let history = history.clone();

        tokio::spawn(async move {
            println!("spawn");
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                do_work().await;
                // retuning how many bytes were read
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                let mut data_history = history.lock().await;
                for i in 0..1024 {
                    data_history[0][i] = data_history[1][i];
                }
                for i in 0..1024 {
                    data_history[1][i] = data_history[2][i];
                }
                for (place, data) in data_history[2].iter_mut().zip(buf.iter()) {
                    *place = *data
                }
                // drop(data_history);
                for value in data_history.iter() {
                    // for testing just print first char
                    if let Ok(s) = std::str::from_utf8(&value[0..1]) {
                        println!("{:?}", s);
                    } else {
                        println!("error while converting to string");
                    }
                }

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
                println!("end of the loop");
            }
        });
    }
}

async fn do_work() {
    sleep(Duration::from_secs(1)).await;
    println!("some work was done");
}
