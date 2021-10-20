use super::conversion_error::ConversionError;
use super::convert_field::convert_field;
use graphql_parser::schema::{Definition, Field, ObjectType, Type, TypeDefinition};
use graphql_parser::Pos;

use rtg_model::explicit_model::ExplicitEntity;

pub fn convert_entity(
  entity: &ExplicitEntity,
) -> Result<
  (
    Definition<String>,
    Field<String>,
    Field<String>,
    Field<String>,
  ),
  ConversionError,
> {
  match entity {
    ExplicitEntity::DatabaseTable {
      graphql_entity_type_name,
      graphql_get_single_operation_name,
      graphql_get_list_operation_name,
      graphql_get_connection_operation_name,
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

      Ok((
        Definition::TypeDefinition(TypeDefinition::Object(ObjectType {
          position: Pos { line: 0, column: 0 },
          description: None,
          name: graphql_entity_type_name.to_owned(),
          implements_interfaces: vec![],
          directives: vec![],
          fields: graphql_fields,
        })),
        Field {
          position: Pos { line: 0, column: 0 },
          description: None,
          name: graphql_get_single_operation_name.to_owned(),
          arguments: vec![],
          field_type: Type::NamedType(graphql_entity_type_name.to_owned()),
          directives: vec![],
        },
        Field {
          position: Pos { line: 0, column: 0 },
          description: None,
          name: graphql_get_list_operation_name.to_owned(),
          arguments: vec![],
          field_type: Type::NonNullType(Box::new(Type::ListType(Box::new(Type::NonNullType(
            Box::new(Type::NamedType(graphql_entity_type_name.to_owned())),
          ))))),
          directives: vec![],
        },
        Field {
          position: Pos { line: 0, column: 0 },
          description: None,
          name: graphql_get_connection_operation_name.to_owned(),
          arguments: vec![],
          field_type: Type::NonNullType(Box::new(Type::ListType(Box::new(Type::NonNullType(
            Box::new(Type::NamedType(graphql_entity_type_name.to_owned())),
          ))))),
          directives: vec![],
        },
      ))
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
