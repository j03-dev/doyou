use rusql_alchemy::Error;
use rusql_alchemy::prelude::Connection;

/// Columns added after the first release. `Database::up()` only creates missing
/// tables, so existing databases need these applied explicitly.
const TRACK_COLUMNS: &[(&str, &str)] = &[
    ("album", "varchar(255)"),
    ("duration", "float"),
    ("cover_path", "varchar(500)"),
    ("downloaded_at", "float"),
];

pub(super) async fn apply(conn: &Connection) -> Result<(), Error> {
    let mut rows = conn.query("PRAGMA table_info(Track)", ()).await?;
    let mut existing = Vec::new();
    while let Some(row) = rows.next().await? {
        existing.push(row.get::<String>(1)?);
    }

    for (name, definition) in TRACK_COLUMNS {
        if !existing
            .iter()
            .any(|column| column.eq_ignore_ascii_case(name))
        {
            let statement = format!("alter table Track add column {name} {definition}");
            conn.execute(&statement, ()).await?;
        }
    }

    Ok(())
}
