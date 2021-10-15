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
  #[error("GraphQL directive `{0}` is not supported")]
  UnsupportedDirective(String),
  #[error("Field `{0}.{1}` is wrong, Nullable array field types are not supported, use [YourType!]! instead of [YourType] or [YourType]! or [YourType!]")]
  NullableArrayFieldType(String, String),
  #[error("Field `{0}.{1}` is wrong, Multiply non-nullable field types are not supported, use YourType! instead of YourType!! or YourType!!!")]
  MultipleNonNullFieldType(String, String),
  #[error("Field `{0}.{1}` is wrong, Array fields are not supported yet")]
  NonSupportedArray(String, String),
  #[error("Field with GraphQL field name `{0} not found in model")]
  FieldNotFound(String),
  #[error("The `sql` directive does not support the argument `{0}`.")]
  SqlDirectiveArgument(String),
  #[error("The `sql` directive `type` argument must be a string or an object")]
  SqlDirectiveTypeArgument,
  #[error(
    "The `sql` directive `type` argument must be a valid sql type. `{0}` is not a valid sql type. Error: {1}"
  )]
  SqlDirectiveTypeArgumentValue(String, String),
  #[error("The `sql` directive `name` argument must be a string")]
  SqlDirectiveNameArgument,
  #[error(
    "The `sql` directive `name` argument must be a valid sql identifier. `{0}` is not a valid sql identifier."
  )]
  SqlDirectiveNameArgumentValue(String),
  #[error("Generic error: `{0}`")]
  Generic(String),
  #[error("Unknown convert error")]
  Unknown,
}
