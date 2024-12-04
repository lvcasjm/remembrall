use inquire::Select;
use remembrall::config::RemembrallConfig;
use remembrall::media::Media;
use remembrall::{create, database, list};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    println!("Welcome to Remembrall 🧙");

    let conf: RemembrallConfig = confy::load("remembrall", None)?;

    if conf.connection_url.is_empty() {
        create::request_connection_string()
    }

    let available_actions = vec!["List", "Create", "Config", "Metadata"];

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
    } else if action == "Metadata" {
        let media = database::list().await?;

        for item in media {
            Media::fetch_metadata(&item).await?;
        }
    }

    Ok(())
}
