#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("I/O error: {0}")]
  IoError(#[from] std::io::Error),
  #[error("Zip error: {0}")]
  ZipError(#[from] zip::result::ZipError),
  #[error("TOML Deserialization error: {0}")]
  TomlDeserializeError(#[from] toml::de::Error),
  #[error("Markdown error: {0}")]
  MarkdownError(markdown::message::Message),
  #[error("Missing meta file")]
  MissingMetaFile,
}
