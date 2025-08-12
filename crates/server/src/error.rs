use rocket::{Response, http::Status, response::Responder};

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Rocket error: {0}")]
  RocketError(#[from] rocket::Error),
  #[error("mail_send error: {0}")]
  MailSendError(#[from] mail_send::Error),
  #[error("sqlx error: {0}")]
  SqlxError(#[from] sqlx::Error),
  #[error("libsql error: {0}")]
  LibSqlError(#[from] libsql::Error),
  #[error("content index unsupported schema version: {0}")]
  ContentIndexUnsupportedSchemaVersion(i64),
}

impl<'r, 'o> Responder<'r, 'o> for Error
where
  'o: 'r,
{
  fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
    rocket::response::Result::Ok(
      Response::build()
        .status(Status::InternalServerError)
        .finalize(),
    )
  }
}
