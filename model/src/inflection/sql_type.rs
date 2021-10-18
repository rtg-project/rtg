use super::inflection_error;
use crate::explicit_model::Type;

pub fn inflect_sql_type_from_graphql_type(
  graphql_type: &str,
  is_array: bool,
) -> Result<Type, inflection_error::InflectionError> {
  match is_array {
    false => match graphql_type {
      "String" => Ok(Type::Text),
      "Boolean" => Ok(Type::Bool),
      "SmallInt" => Ok(Type::Int2),
      "Int" => Ok(Type::Int4),
      "BigInt" => Ok(Type::Int8),
      "Float" => Ok(Type::Float4),
      "Double" => Ok(Type::Float8),
      "Numeric" => Ok(Type::Numeric),
      "DateTime" => Ok(Type::Timestamptz),
      "Json" => Ok(Type::Jsonb),
      "Bytes" => Ok(Type::Bytea),
      type_name => Err(inflection_error::InflectionError::UnsupportedGraphQLScalar(
        type_name.to_string(),
      )),
    },
    true => match graphql_type {
      "String" => Ok(Type::TextArray),
      "Boolean" => Ok(Type::BoolArray),
      "SmallInt" => Ok(Type::Int2Array),
      "Int" => Ok(Type::Int4Array),
      "BigInt" => Ok(Type::Int8Array),
      "Float" => Ok(Type::Float4Array),
      "Double" => Ok(Type::Float8Array),
      "Numeric" => Ok(Type::NumericArray),
      "DateTime" => Ok(Type::TimestamptzArray),
      "Json" => Ok(Type::JsonbArray),
      "Bytes" => Ok(Type::ByteaArray),
      type_name => Err(inflection_error::InflectionError::UnsupportedGraphQLScalar(
        type_name.to_string(),
      )),
    },
  }
}

pub fn inflect_graphql_type_from_sql_type(
  sql_type: &Type,
) -> Result<(String, bool), inflection_error::InflectionError> {
  match sql_type {
    Type::Text => Ok(("String".to_string(), false)),
    Type::Bool => Ok(("Boolean".to_string(), false)),
    Type::Int2 => Ok(("SmallInt".to_string(), false)),
    Type::Int4 => Ok(("Int".to_string(), false)),
    Type::Int8 => Ok(("BigInt".to_string(), false)),
    Type::Float4 => Ok(("Float".to_string(), false)),
    Type::Float8 => Ok(("Double".to_string(), false)),
    Type::Numeric => Ok(("Numeric".to_string(), false)),
    Type::Timestamptz => Ok(("DateTime".to_string(), false)),
    Type::Jsonb => Ok(("Json".to_string(), false)),
    Type::Json => Ok(("Json".to_string(), false)),
    Type::Bytea => Ok(("Bytes".to_string(), false)),
    Type::TextArray => Ok(("String".to_string(), true)),
    Type::BoolArray => Ok(("Boolean".to_string(), true)),
    Type::Int2Array => Ok(("SmallInt".to_string(), true)),
    Type::Int4Array => Ok(("Int".to_string(), true)),
    Type::Int8Array => Ok(("BigInt".to_string(), true)),
    Type::Float4Array => Ok(("Float".to_string(), true)),
    Type::Float8Array => Ok(("Double".to_string(), true)),
    Type::NumericArray => Ok(("Numeric".to_string(), true)),
    Type::TimestamptzArray => Ok(("DateTime".to_string(), true)),
    Type::JsonbArray => Ok(("Json".to_string(), true)),
    Type::JsonArray => Ok(("Json".to_string(), true)),
    Type::ByteaArray => Ok(("Bytes".to_string(), true)),
    type_name => Err(inflection_error::InflectionError::UnsupportedSQLType(
      format!("{:?}", type_name),
    )),
  }
}
