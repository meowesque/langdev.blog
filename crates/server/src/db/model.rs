use super::Db;
use crate::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, sqlx::Type)]
pub struct UserId(pub i64);

#[derive(sqlx::FromRow)]
pub struct User {
  pub id: UserId,
  pub username: String,
  pub email: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl User {
  pub async fn get_by_email(db: &Db, email: impl AsRef<str>) -> Result<Option<User>> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
      .bind(email.as_ref())
      .fetch_optional(&db.0)
      .await
      .map_err(Error::from)
  }
}
