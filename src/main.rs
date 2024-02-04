pub mod media;
pub mod prompt;
pub mod save;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let media = prompt::prompt();

    println!("{:?}", media);

    save::save(&media).await.expect("TODO: panic message");
}
