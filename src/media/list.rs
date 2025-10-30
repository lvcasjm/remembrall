use crate::database;
use comfy_table::*;

pub async fn query() {
    let mut table = Table::new();

    let list = database::list().await.unwrap();

    table
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Title", "Description", "Type", "Completed"]);

    for row in list {
        table.add_row(vec![
            row.title,
            row.description,
            row.media_type,
            row.completed_at.format("%d/%m/%Y").to_string(),
        ]);
    }

    println!("{table}");
}
