use futures_util::{StreamExt as _, pin_mut};
use redis::AsyncCommands;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut publish_conn = client.get_async_connection().await?;
    let mut pubsub_conn = client.get_async_connection().await?.into_pubsub();

    pubsub_conn.subscribe("wavephone").await?;
    let pubsub_stream = pubsub_conn.on_message();
    pin_mut!(pubsub_stream);

    publish_conn.publish("wavephone", "banana").await?;

    let pubsub_msg: String = pubsub_stream.next().await.unwrap().unwrap().get_payload()?;
    assert_eq!(&pubsub_msg, "banana");

    Ok(())
}
