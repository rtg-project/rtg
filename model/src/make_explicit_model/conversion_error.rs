use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
  #[error("Field name is missing")]
  FieldNameMissing,
  #[error("Entity name is missing")]
  EntityNameMissing,
  #[error("Entity `{0}` contains no field, Entities must have at least one field")]
  EntityHasNoField(String),
  #[error("Generic error: `{0}`")]
  Generic(String),
  #[error("Unknown convert error")]
  Unknown,
}
