use super::conversion_error::ConversionError;
use super::convert_field::convert_field;
use graphql_parser::schema::{Definition, Field, ObjectType, TypeDefinition};
use graphql_parser::Pos;

use rtg_model::explicit_model::ExplicitEntity;

pub fn convert_entity(entity: &ExplicitEntity) -> Result<Definition<String>, ConversionError> {
  match entity {
    ExplicitEntity::DatabaseTable {
      graphql_entity_type_name,
      fields,
      ..
    } => {
      let graphql_fields = match fields
        .iter()
        .map(|field| convert_field(field))
        .collect::<Result<Vec<Field<String>>, ConversionError>>()
      {
        Ok(sql_column_items) => sql_column_items,
        Err(err) => return Err(err),
      };

      Ok(Definition::TypeDefinition(TypeDefinition::Object(
        ObjectType {
          position: Pos { line: 0, column: 0 },
          description: None,
          name: graphql_entity_type_name.to_owned(),
          implements_interfaces: vec![],
          directives: vec![],
          fields: graphql_fields,
        },
      )))
    }
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
