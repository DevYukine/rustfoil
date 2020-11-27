use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TokenFile {
    pub tokens: Vec<TokenInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct TokenInfo {
    pub hash: i64,
    pub scopes: Vec<String>,
    pub token: Token,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in_timestamp: i32,
}

#[derive(Serialize, Deserialize)]
pub struct TinfoilToken {
    pub access_token: String,
    pub refresh_token: String,
}
