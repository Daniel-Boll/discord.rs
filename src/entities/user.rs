#[cfg(feature = "ssr")]
use scylla::Session;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(scylla::FromRow))]
pub struct User {
  pub id: uuid::Uuid,
  pub username: String,
  pub password_hash: String,
}

#[cfg(feature = "ssr")]
impl User {
  pub async fn register_entity(session: &Session) -> Result<(), Box<dyn std::error::Error>> {
    session.query("CREATE TABLE IF NOT EXISTS users (id UUID, username TEXT, password_hash TEXT, primary key (id))", &[]).await?;
    session
      .query(
        "CREATE INDEX IF NOT EXISTS username_index ON users (username)",
        &[],
      )
      .await?;

    Ok(())
  }
}
