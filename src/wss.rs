use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message::{self};
use url::Url;

use crate::Config;

pub async fn wss_loop(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse("wss://stream.data.alpaca.markets/v1beta1/news")?;
    println!("Connecting to Alpaca News wss...");

    let (mut ws_stream, _) = tokio_tungstenite::connect_async(url.as_str()).await?;
    println!("Succesufull, ws handshake");

    let auth_msg = serde_json::json!({
        "action": "auth",
        "key": &config.api_key_id,
        "secret": &config.api_secret
    });

    let sub_msg = serde_json::json!({
        "action": "subscribe",
        "news": ["*"]
    });

    ws_stream.send(Message::Text(auth_msg.to_string().into())).await?;
    ws_stream.send(Message::Text(sub_msg.to_string().into())).await?;

    println!("Authenticated and Subsribed to ws, listening for data... \n");

    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(Message::Text(m)) => {
                println!("{}",m)
            },
            Ok(Message::Ping(ping)) => {
                let _ = ws_stream.send(Message::Pong(ping)).await;
            },
            Ok(Message::Close(_)) => {
                println!("Connection closed by server");
                break;
            },
            Err(e) => {
                println!("Error during stream: {}", e);
                break
            },
            _=>{}
        }
    }

    Ok(())
}