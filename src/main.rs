use crate::config::Config;
use crate::wss::wss_loop;
mod config;
mod wss;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Loading Config...");
    let config = match Config::load_config() {
        Ok(c) => {
            println!("Config succesufully loaded");
            c
        },
        Err(e) => {
            println!("[CONFIG ERROR]: {}", e);
            std::process::exit(1);
        }
    };

    wss_loop(config).await?;

    Ok(())
}