use crate::database;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, *};

pub async fn query() {
    let mut table = Table::new();

    let list = database::list().await.unwrap();

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Title").add_attribute(Attribute::Bold),
            Cell::new("Description").add_attribute(Attribute::Bold),
            Cell::new("Type").add_attribute(Attribute::Bold),
            Cell::new("Completed").add_attribute(Attribute::Bold),
        ]);

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
