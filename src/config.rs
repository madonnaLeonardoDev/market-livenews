use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_key_id: String,
    pub api_secret: String
}
impl Config{
    pub fn load_config() -> Result<Self, String> {
        let path = Path::new("./").join("market_feed.conf");

        if path.exists() && path.is_file() {
            let read = fs::read(path)
                .map_err(|_| "Failed reading data from config")?;

            let conf: Config = toml::from_slice(&read)
                .map_err(|_| "Failed Deserializing Config, wrong format")?;

            return Ok(conf)
        } else {
            let empty_default = r#"
api_key_id = #Your Alpaca key ID
api_secret =  #Your Secret Alpaca API KEY
            "#;
                fs::write(&path, empty_default)
                .map_err(|e| format!("Failed initializing config template: {}", e))?;

            Err(format!("No existing config, fill up the config at {}", path.to_string_lossy()))   
        }
    }
}