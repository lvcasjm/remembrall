use serde::{Deserialize, Serialize};

pub mod media;
pub mod prompt;
pub mod save;

#[derive(Debug, Serialize, Deserialize)]
struct RemembrallConfig {
    connection_url: String,
}

impl Default for RemembrallConfig {
    fn default() -> Self {
        RemembrallConfig {
            connection_url: "".to_string(),
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    println!("Welcome to Remembrall 🧙");

    let config: RemembrallConfig = confy::load("remembrall", None)?;

    if config.connection_url.is_empty() {
        prompt::request_connection_string()
    }

    let media = prompt::prompt();

    save::save(&media).await?;

    println!("Save successful, bye for now 🧙");

    Ok(())
}
