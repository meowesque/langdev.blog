use super::Db;
use crate::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, sqlx::Type)]
#[sqlx(transparent, no_pg_array)]
pub struct UserId(pub i64);

#[derive(Debug, Serialize, Deserialize, Clone, Copy, sqlx::Type)]
#[sqlx(transparent, no_pg_array)]
pub struct InviteId(pub i64);

#[derive(sqlx::FromRow, Debug)]
pub struct User {
  pub id: UserId,
  pub username: String,
  pub email: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl User {
  pub async fn get_by_email(db: &Db, email: impl AsRef<str>) -> Result<Option<User>> {
    sqlx::query_as::<_, User>(
      "SELECT id, username, email, created_at, updated_at FROM users WHERE email = $1",
    )
    .bind(email.as_ref())
    .fetch_optional(&db.0)
    .await
    .map_err(Error::from)
  }

  pub async fn get_by_id(db: &Db, id: UserId) -> Result<Option<User>> {
    sqlx::query_as::<_, User>(
      "SELECT id, username, email, created_at, updated_at FROM users WHERE id = $1",
    )
    .bind(id.0)
    .fetch_optional(&db.0)
    .await
    .map_err(Error::from)
  }
}

#[derive(sqlx::FromRow, Debug)]
pub struct Invite {
  pub id: InviteId,
  pub inviter_id: UserId,
  pub token: String,
  pub email: String,
  pub created_at: DateTime<Utc>,
  pub expires_at: DateTime<Utc>,
  pub used_at: Option<DateTime<Utc>>,
}

impl Invite {
  pub async fn get_by_token(db: &Db, token: impl AsRef<str>) -> Result<Option<Invite>> {
    sqlx::query_as::<_, Invite>(
      "SELECT id, inviter_id, token, email, created_at, expires_at, used FROM invites WHERE token = $1",
    )
    .bind(token.as_ref())
    .fetch_optional(&db.0)
    .await
    .map_err(Error::from)
  }
}
