use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
  #[error("Unknown conversion error")]
  Unknown,
}
