#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Rocket error: {0}")]
  RocketError(#[from] rocket::Error),
  #[error("mail_send error: {0}")]
  MailSendError(#[from] mail_send::Error),
  #[error("sqlx error: {0}")]
  SqlxError(#[from] sqlx::Error),
}
