use inquire::Select;
use serde::{Deserialize, Serialize};

pub mod create;
pub mod database;
mod list;
pub mod media;

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
        create::request_connection_string()
    }

    let available_actions = vec!["List", "Create", "Config"];

    let action = Select::new("What do you want to do?", available_actions)
        .prompt()
        .unwrap();

    if action == "Config" {
        create::request_connection_string()
    } else if action == "List" {
        list::query().await;
    } else if action == "Create" {
        let media = create::prompt();

        database::save(&media).await?;

        println!("Save successful, bye for now 🧙");
    }

    Ok(())
}