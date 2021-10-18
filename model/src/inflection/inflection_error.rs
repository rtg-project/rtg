use thiserror::Error;

#[derive(Error, Debug)]
pub enum InflectionError {
  #[error("Unsupported GraphQL scalar type: `{0}`")]
  UnsupportedGraphQLScalar(String),
  #[error("Unsupported SQL type: `{0}`")]
  UnsupportedSQLType(String),
  #[error("Inflection error: `{0}`")]
  Generic(String),
  #[error("Unknown convert error")]
  Unknown,
}
