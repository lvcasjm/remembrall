use std::env;

use inquire::Select;
use remembrall::config::RemembrallConfig;
use remembrall::{create, database, list};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    println!("Welcome to Remembrall 🧙");

    let conf: RemembrallConfig = confy::load("remembrall", None)?;

    if conf.connection_url.is_empty() {
        create::request_connection_string()
    }

    let available_actions = vec!["List", "Create", "Setup"];

    let action: String;

    // TODO: improve this to be a match statement with all above opts
    if args.iter().any(|a| a.contains("-l")) {
        action = String::from("List");
    } else if args.iter().any(|a| a.contains("-c")) {
        action = String::from("Create");
    } else if args.iter().any(|a| a.contains("-s")) {
        action = String::from("Setup");
    } else {
        action = Select::new("What do you want to do?", available_actions)
            .prompt()
            .unwrap()
            .to_string();
    }

    if action == "Setup" {
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
