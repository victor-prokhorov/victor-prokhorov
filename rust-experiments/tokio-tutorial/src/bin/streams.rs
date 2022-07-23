use mini_redis::client;
use tokio::time::sleep;
use tokio_stream::StreamExt;

async fn publish() -> mini_redis::Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    // await so we get all messages
    sleep(std::time::Duration::from_secs(1)).await;

    // Publish some data
    client.publish("numbers", "1".into()).await?;
    client.publish("numbers", "two".into()).await?;
    client.publish("numbers", "3".into()).await?;
    sleep(std::time::Duration::from_secs(3)).await;
    client.publish("numbers", "four".into()).await?;
    client.publish("numbers", "five".into()).await?;
    client.publish("numbers", "6".into()).await?;
    Ok(())
}

async fn subscribe() -> mini_redis::Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;
    let subscriber = client.subscribe(vec!["numbers".to_string()]).await?;
    // let messages = subscriber
    //     .into_stream()
    //     .filter(|msg| match msg {
    //         Ok(msg) if msg.content.len() < 4 => true,
    //         _ => false,
    //     })
    //     .map(|msg| msg.unwrap().content)
    //     .take(3);
    // tokio::pin!(messages);

    let messages = subscriber
        .into_stream()
        .map(|msg| msg.unwrap().content)
        .take(6);

    tokio::pin!(messages);

    while let Some(msg) = messages.next().await {
        // println!("got = {:?}", msg);
        // transform byte strings into string slice
        println!("got = {:?}", std::str::from_utf8(&msg).unwrap());
    }

    Ok(())
}

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    tokio::spawn(async { publish().await });
    subscribe().await?;

    Ok(())
}
