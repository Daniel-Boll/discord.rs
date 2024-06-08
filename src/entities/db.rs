use std::sync::Arc;

use leptos::logging;
use scylla::Session;

use super::user::User;

// TODO: Find a way to cache this session connection
pub async fn create_session() -> Result<Arc<Session>, Box<dyn std::error::Error>> {
  logging::log!("Creating session");
  let session = scylla::SessionBuilder::new()
    .known_node("127.0.0.1:9042")
    .build()
    .await?;

  // Change replication factor to 3 when running in production with 3 nodes
  session.query("CREATE KEYSPACE IF NOT EXISTS discord WITH REPLICATION = { 'class' : 'NetworkTopologyStrategy', 'replication_factor' : 1 }", &[]).await?;
  session.use_keyspace("discord", true).await?;

  User::register_entity(&session).await?;

  Ok(Arc::new(session))
}
