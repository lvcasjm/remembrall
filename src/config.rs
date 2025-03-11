use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RemembrallConfig {
    pub connection_url: String,
    pub sqlite_connection_url: String,
}

impl Default for RemembrallConfig {
    fn default() -> Self {
        RemembrallConfig {
            connection_url: "".to_string(),
            sqlite_connection_url: "".to_string(),
        }
    }
}
