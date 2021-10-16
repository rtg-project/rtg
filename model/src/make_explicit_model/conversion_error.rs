use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
  #[error("Field name is missing, a field must have either a name, a sql column name, or a graphql field name")]
  FieldNameMissing,
  #[error("Field type is missing on field `{0}`")]
  FieldTypeMissing(String),
  #[error("Entity name is missing")]
  EntityNameMissing,
  #[error("Entity `{0}` contains no field, Entities must have at least one field")]
  EntityHasNoField(String),
  #[error("Generic error: `{0}`")]
  Generic(String),
  #[error("Unknown convert error")]
  Unknown,
}
