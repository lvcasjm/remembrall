use inquire::Text;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RemembrallConfig {
    pub connection_url: String,
}

impl Default for RemembrallConfig {
    fn default() -> Self {
        RemembrallConfig {
            connection_url: "".to_string(),
        }
    }
}

pub fn request_connection_string() {
    let connection_url =
        Text::new("To get started using Remembrall, please start by providing a connection url, the url should be a valid connection string to a running mysql database.")
            .prompt()
            .unwrap();

    let updated_config: RemembrallConfig = RemembrallConfig { connection_url };

    confy::store("remembrall", None, &updated_config).unwrap();
}
