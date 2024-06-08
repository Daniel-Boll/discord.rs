use std::sync::Arc;

use axum::extract::FromRef;
use leptos::{use_context, LeptosOptions, ServerFnError};
use leptos_router::RouteListing;
use scylla::Session;

#[derive(Debug, FromRef, Clone)]
pub struct AppState {
  pub leptos_options: LeptosOptions,
  pub session: Arc<Session>,
  pub routes: Vec<RouteListing>,
}

pub fn session() -> Result<Arc<Session>, ServerFnError> {
  use_context::<Arc<Session>>().ok_or_else(|| ServerFnError::ServerError("Session missing.".into()))
}
