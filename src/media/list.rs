use crate::database;

pub async fn query() {
    let list = database::list().await.unwrap();

    println!("_________________________________________________________________________________________________________________________");
    println!(
        "|{0: <5} | {1: <50} | {2: <20} | {3: <20} | {4: <12}|",
        "id", "title", "description", "type", "completed"
    );

    for media in list {
        println!(
            "|{0: <5} | {1: <50} | {2: <20} | {3: <20} | {4: <12}|",
            media.id.unwrap(),
            truncate(media.title, 46),
            media.description,
            media.media_type,
            media.completed_at.format("%d/%m/%Y")
        )
    }

    println!("-------------------------------------------------------------------------------------------------------------------------");
}

fn truncate(s: String, max: usize) -> String {
    s.chars().take(max).collect()
}
