use graphql_parser::schema::ParseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
  #[error("GraphQL Parse error")]
  ParseError(ParseError),
  #[error("Unknown conversion error")]
  Unknown,
}
