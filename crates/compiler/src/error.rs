#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("I/O error: {0}")]
  IoError(#[from] std::io::Error),
  #[error("Zip error: {0}")]
  ZipError(#[from] zip::result::ZipError),
}
