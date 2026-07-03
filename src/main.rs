use inquire::Select;
use remembrall::config::RemembrallConfig;
use remembrall::{config, database, media};
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    println!("Welcome to Remembrall 🧙");

    let conf: RemembrallConfig = confy::load("remembrall", None)?;

    if conf.connection_url.is_empty() {
        config::request_connection_string()
    }

    const AVAILABLE_ACTIONS: &[&str] = &["List", "Create", "Setup"];

    let action = args
        .iter()
        .find_map(|a| {
            if a.contains("-l") {
                Some("List")
            } else if a.contains("-c") {
                Some("Create")
            } else if a.contains("-s") {
                Some("Setup")
            } else {
                None
            }
        })
        .map(String::from)
        .unwrap_or_else(|| {
            Select::new("What do you want to do?", AVAILABLE_ACTIONS.to_vec())
                .prompt()
                .unwrap()
                .to_string()
        });

    match action.as_str() {
        "Setup" => config::request_connection_string(),
        "List" => media::list::show_table().await,
        "Create" => {
            let media_fields = media::create::prompt().unwrap();

            match &media_fields {
                Some(fields) => {
                    let pool = database::connection().await.unwrap();

                    media::create::save(fields, pool).await?;

                    println!("Save successful, bye for now 🧙");
                }
                None => {
                    println!("No media selected, exiting.");
                    return Ok(());
                }
            }
        }
        _ => unreachable!("Invalid action selected"),
    }

    Ok(())
}
