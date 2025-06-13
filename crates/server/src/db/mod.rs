pub mod model;

use crate::prelude::*;

#[derive(Clone)]
pub struct Db(pub(super) sqlx::Pool<sqlx::postgres::Postgres>);

impl Db {
  pub async fn new(connstr: impl AsRef<str>) -> Result<Self> {
    Ok(Self(
      sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(connstr.as_ref())
        .await?,
    ))
  }
}
