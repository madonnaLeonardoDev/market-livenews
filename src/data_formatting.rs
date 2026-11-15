use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RawPayload {
    #[allow(non_snake_case)]
    pub T: String,
    pub id: String,
    pub headline: String,
    pub summary: String,
    pub created_at: String,
    pub updated_at: String,
    pub content: String,
    pub url: String,
    pub symbols: Vec<String>,
    pub source: String
}
pub trait ValueExt {
    fn parse<T: DeserializeOwned>(self) -> Result<T, String>;
}

impl ValueExt for Value {
    fn parse<T: DeserializeOwned>(self) -> Result<T, String> {
        let payload: T = serde_json::from_value(self)
            .map_err(|e| format!("Failed deserializng payload: {}", e))?;

        Ok(payload)
    }
}

pub struct NewsData {
    pub is_update: bool,
    pub headline: String,
    pub summary: String,
    pub content: String,
    pub symbols: Vec<String>,
    pub source: String,
}

impl NewsData {
        pub fn as_pretty_str(&self) -> String {
        let mut symbols_str = String::new();

        for (i,symbol) in self.symbols.iter().enumerate() {
            if i > 0 {
                symbols_str.push_str(", ");
            }
            symbols_str.push_str(symbol);
        };

        let new_str = if self.is_update {
            "[UPDATE]"
        } else {
            "[NEW]"
        };

        format!("HEADLINE: {}{}\n SYMBOLS: {} | SOURCE: {}\n Summary: '{}'\n Content: {}",
            new_str,self.headline, symbols_str, self.source, self.summary, self.content)
    }
}


impl RawPayload {
    pub fn into_news(self) -> Result<NewsData, String> {
        Ok(NewsData{
            is_update: self.created_at != self.updated_at,
            headline: self.headline,
            summary: self.summary,
            content: self.content,
            symbols: self.symbols,
            source: self.source
        })
    }
}

