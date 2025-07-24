use crate::{db::model::UserId, prelude::*};
use std::{
  borrow::Cow,
  path::{Path, PathBuf},
};

const SCHEMA_VERSION: i64 = 1;
const SCHEMA_SQL: &str = include_str!("sql/schema.sql");

#[derive(Debug)]
pub struct PostMetadata<'a> {
  pub author_id: UserId,
  pub author_username: Cow<'a, str>,
  pub slug: Cow<'a, str>,
  pub filepath: Cow<'a, Path>,
}

#[derive(Clone)]
pub struct ContentIndex(libsql::Connection);

impl ContentIndex {
  pub async fn new(path: impl AsRef<Path>) -> Result<Self> {
    let conn = libsql::Builder::new_local(path).build().await?.connect()?;
    let index = ContentIndex(conn);

    index.trx(Trx::migrate).await?;

    Ok(index)
  }

  pub async fn trx<A>(&self, k: impl AsyncFnOnce(&Trx) -> Result<A>) -> Result<A> {
    let trx = Trx(self.0.transaction().await?);

    let a = k(&trx).await?;
    trx.commit().await?;

    Ok(a)
  }
}

pub struct Trx(libsql::Transaction);

impl Trx {
  async fn commit(self) -> Result<()> {
    self.0.commit().await?;
    Ok(())
  }

  pub async fn version(&self) -> Result<i64> {
    let mut row = self.0.query("PRAGMA user_version", ()).await?;

    let version = row
      .next()
      .await?
      .map(|row| row.get(0))
      .transpose()?
      .unwrap_or(0);

    Ok(version)
  }

  pub async fn migrate(&self) -> Result<()> {
    let version = self.version().await?;

    ::log::info!(
      "Attempting to migrate content index schema, current version: {}",
      version,
    );

    match version {
      0 => {
        let _ = self.0.execute_batch(SCHEMA_SQL).await?;
        ::log::info!("Database schema initialized (ver. {})", SCHEMA_VERSION);
      }
      _ if version == SCHEMA_VERSION => {
        // Do nothing.
        ::log::info!("Database schema already up-to-date (ver. {})", version);
      }
      unsupported => {
        ::log::error!("Database schema version {} is not supported", unsupported);
        return Err(Error::ContentIndexUnsupportedSchemaVersion(unsupported));
      }
    }

    Ok(())
  }

  pub async fn get_post_filepath(&self, author: &str, post_slug: &str) -> Result<Option<PathBuf>> {
    self
      .0
      .query(
        "SELECT filepath FROM post_metadata WHERE author_username = ? AND slug = ?",
        (author, post_slug),
      )
      .await?
      .next()
      .await?
      .map(|row| row.get::<String>(0).map(PathBuf::from))
      .transpose()
      .map_err(Error::from)
  }

  pub async fn insert_post_metadata(&self, post_metadata: &PostMetadata<'_>) -> Result<()> {
    self
      .0
      .query(
        "INSERT author_id, author_username, slug, filepath VALUES (?, ?, ?, ?) INTO post_metadata",
        (
          post_metadata.author_id.0,
          post_metadata.author_username.as_ref(),
          post_metadata.slug.as_ref(),
          post_metadata
            .filepath
            .as_ref()
            .to_str()
            .expect("PathBuf should be valid UTF-8"),
        ),
      )
      .await?;

    Ok(())
  }
}
