use rusql_alchemy::Error;
use rusql_alchemy::prelude::*;
use tokio::sync::OnceCell;

use crate::core::platform::get_config_path;

mod migrations;
pub mod models;

/// Shared connection setup. Query logic belongs in the repository modules.
pub(crate) async fn conn() -> Result<&'static Connection, Error> {
    static CONN: OnceCell<Connection> = OnceCell::const_new();
    CONN.get_or_try_init(|| async move {
        let path = get_config_path()?;
        let database = Database::new_local(path.to_str().unwrap()).await?;
        database.up().await?;
        migrations::apply(&database.conn).await?;
        Ok(database.conn)
    })
    .await
}
