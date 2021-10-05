use graphql_parser::query::ParseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
  #[error("GraphQL parse error")]
  GraphqlParse(#[from] ParseError),
  #[error("Document must contain one and only one definition")]
  DefinitionNumber,
  #[error("GraphQL syntax `{0}` is not supported yet")]
  UnsupportedSyntax(String),
  #[error("GraphQL operation `{0}` of kind `{1}` is not supported yet")]
  UnsupportedOperation(String, String),
  #[error("Field with GraphQL field name `{0} not found in model")]
  FieldNotFound(String),
  #[error("Generic error: `{0}`")]
  Generic(String),
  #[error("Unknown convert error")]
  Unknown,
}
