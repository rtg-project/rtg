use graphql_parser::schema::ParseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
  #[error("GraphQL parse error")]
  GraphqlParse(#[from] ParseError),
  #[error("GraphQL schema must contain one and only one definition")]
  DefinitionNumber,
  #[error("GraphQL syntax `{0}` is not supported yet")]
  UnsupportedSyntax(String),
  #[error("Field with GraphQL field name `{0} not found in model")]
  FieldNotFound(String),
  #[error("Generic error: `{0}`")]
  Generic(String),
  #[error("Unknown convert error")]
  Unknown,
}
