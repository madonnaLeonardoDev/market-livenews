use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message::{self};
use crate::data_formatting::{ValueExt};
use url::Url;
use std::collections::{HashMap};

use crate::{Config, data_formatting::RawPayload};

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

    println!("Authenticated and Subscribed to ws, listening for data... \n");

    let mut session_news_set: HashMap<String, String> = HashMap::new();

    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(Message::Text(m)) => {
                let values: Vec<serde_json::Value> = match serde_json::from_str(m.as_str()) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("Failed deserializng incoming value: {}", e);
                        continue;
                    }
                };

                for val in values {
                    let msg_type = val.get("T").and_then(|t| t.as_str()).unwrap_or_default();

                    if msg_type == "n" /*NEWS TYPE */   {
                        let payload: RawPayload = val.parse()?;


                        let should_print = match session_news_set.get(&payload.id){
                            Some(ua) => ua != &payload.updated_at,
                            None => true
                        };

                        if should_print {
                            session_news_set.insert(payload.id.clone(), payload.updated_at.clone());
                            let news= payload.into_news()?;
                            println!("{}", news.as_pretty_str())
                        }

                    } else {
                        println!("[NON FORMATTED DATA]: {}", m);
                    }

                }
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