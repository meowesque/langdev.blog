use super::prelude::*;
use crate::content::index::ContentIndex;
use rocket::{
  fs::NamedFile,
  response::status::NotFound,
};
use std::path::PathBuf;

#[get("/<author>/<slug>")]
pub async fn get(
  index: &State<ContentIndex>,
  author: AuthorSlug<'_>,
  slug: &str,
) -> Result<NamedFile, NotFound<&'static str>> {
  let mut filepath = index
    .trx(async |t| t.get_post_filepath(author.0, slug).await)
    .await
    .map_err(|_| NotFound("Post not found"))?
    .ok_or_else(|| NotFound("Post not found"))?;

  filepath.push(format!("{}.html", slug));

  NamedFile::open(filepath)
    .await
    .map_err(|_| NotFound("Post not found"))
}

#[get("/<author>/<slug>/<file..>", rank = 2)]
pub async fn get_raw(
  index: &State<ContentIndex>,
  author: AuthorSlug<'_>,
  slug: &str,
  file: PathBuf,
) -> Result<NamedFile, NotFound<&'static str>> {
  let mut filepath = index
    .trx(async |t| t.get_post_filepath(author.0, &slug).await)
    .await
    .map_err(|e| {
      ::log::info!("{:?}", e);
      NotFound("Post not found")
    })?
    .ok_or_else(|| NotFound("Post not found"))?;

  filepath.push(file);

  NamedFile::open(filepath)
    .await
    .map_err(|_| NotFound("Post not found"))
}
