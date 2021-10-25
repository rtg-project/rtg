use super::conversion_error::ConversionError;
use graphql_parser::schema::{Field, Type};
use graphql_parser::Pos;
use rtg_model::explicit_model::ExplicitField;

pub fn convert_field(field: &ExplicitField) -> Result<Field<String>, ConversionError> {
  match field {
    ExplicitField::ScalarDatabaseColumn {
      graphql_field_name,
      graphql_type_name,
      nullable,
      ..
    } => Ok(Field {
      position: Pos { line: 0, column: 0 },
      description: None,
      name: graphql_field_name.to_owned(),
      arguments: vec![],
      field_type: if *nullable {
        Type::NamedType(graphql_type_name.to_owned())
      } else {
        Type::NonNullType(Box::new(Type::NamedType(graphql_type_name.to_owned())))
      },
      directives: vec![],
    }),
  }
}

// Tests
#[cfg(test)]
mod tests {
  use similar_asserts::assert_eq;
  #[test]
  fn it_works() {
    assert_eq!(1, 1);
  }
}
