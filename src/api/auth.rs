use leptos::*;

#[server(RegisterUser, "/api", "Url", "register")]
pub async fn register_user(username: String, password: String) -> Result<(), ServerFnError> {
  use crate::entities::user::User;
  use crate::state::session;
  use bcrypt::{hash, DEFAULT_COST};
  use uuid::Uuid;

  let hashed_password = hash(password, DEFAULT_COST).unwrap();
  let user = User {
    id: Uuid::new_v4(),
    username,
    password_hash: hashed_password,
  };

  let session = session()?;
  session
    .query(
      "INSERT INTO users (id, username, password_hash) VALUES (?, ?, ?)",
      (user.id, user.username.clone(), user.password_hash.clone()),
    )
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

  Ok(())
}

#[server(LoginUser, "/api", "Url", "login")]
pub async fn login_user(username: String, password: String) -> Result<(), ServerFnError> {
  use crate::state::session;
  use bcrypt::verify;

  let session = session()?;
  let rows = session
    .query(
      "SELECT password_hash FROM users WHERE username = ?",
      (username,),
    )
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

  let (password_hash,) = rows
    .rows_typed::<(String,)>()?
    .next()
    .ok_or_else(|| ServerFnError::new("User not found".to_string()))?
    .map_err(|e| ServerFnError::new(e.to_string()))?;

  match verify(password, &password_hash) {
    Ok(true) => Ok(()),
    _ => Err(ServerFnError::new("Invalid password".to_string())),
  }
}
