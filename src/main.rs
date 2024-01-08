use inquire::{DateSelect, Select, Text};
use std::process::exit;

fn main() {
    // 1. show a welcome message
    println!("Welcome to Remembrall 🧙");
    println!(
        "Let's start by logging something you finished watching, reading, or playing recently."
    );

    // 2. Prompt to select a type of media from a list (enum)
    let media_types = vec!["Movie", "Game", "Series", "Book"];

    let media_type = Select::new("What type of Media have you completed?", media_types)
        .prompt()
        .unwrap();

    // 3. Text prompt for the name of the media
    let title = Text::new("What is the title of the media you have finished?")
        .prompt()
        .unwrap();

    // 3.1 Optional description text prompt.
    let description = Text::new("(Optional) Add a description of the media you have finished?")
        .prompt_skippable()
        .unwrap();

    // 4. Prompt for date of completion (default to now)
    let date = DateSelect::new("When did you finish the media?")
        .prompt()
        .unwrap();

    // 5. Rating from 1 to 10, could probably do this as Text prompt
    // let rating_options: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let rating: Option<u8> = Select::new("", rating_options).prompt_skippable().unwrap();

    // 6. Show media in a data list and prompt for confirmation
    println!("Title: {}", title);
    println!("Type: {}", media_type);
    match description {
        Some(d) => Some({
            println!("Description: {}", d);
        }),
        None => None,
    };
    println!("Completed Date: {}", date);

    // 7. On confirm, insert into database.
    let confirm_options = vec!["Cancel", "Save"];
    let confirmation = Select::new("Would you like to save this media?", confirm_options).prompt();

    match confirmation {
        Ok("Cancel") => {
            println!("Cancelled");
            exit(0)
        }
        Ok("Save") => save(),
        _ => println!("Something went wrong, please restart the program."),
    }
}

fn save() {
    println!("Please wait, saving in progress!");
}
