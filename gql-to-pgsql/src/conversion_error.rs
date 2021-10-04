use graphql_parser::query::ParseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
  #[error("GraphQL parse error")]
  GraphqlParse(#[from] ParseError),
  #[error("Document must contain one and only one definition")]
  DefinitionNumber,
  #[error("GraphQL syntax `{0}` is not supported yet")]
  Unsupported(String),
  #[error("Field with GraphQL field name `{0} not found in model")]
  FieldNotFound(String),
  #[error("Unknown convert error")]
  Unknown,
}
