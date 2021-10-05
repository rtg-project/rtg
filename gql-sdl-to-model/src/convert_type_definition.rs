use super::conversion_error::ConversionError;
use graphql_parser::schema::{Text, TypeDefinition};
use rtg_model::entity::Entity;
use rtg_model::field::Field;
use rtg_model::sql_type::Type;
// use rustc_hash::FxHashMap;
use std::rc::Rc;

pub fn convert_type_definition<'a, T: Text<'a>>(
  type_definition: &TypeDefinition<'a, T>,
  // relations_by_name: &FxHashMap<String, Rc<Entity>>,
) -> Result<Entity, ConversionError> {
  // relations_by_name.insert("s", "s");

  match type_definition {
    TypeDefinition::Scalar(_scalar_type) => {
      return Err(ConversionError::UnsupportedSyntax("Scalar".to_string()))
    }
    TypeDefinition::Object(obj) => {
      let fields = obj
        .fields
        .iter()
        .map(|field| {
          return Ok(Rc::new(Field::ScalarDatabaseColumn {
            name: field.name.as_ref().to_string(),
            sql_type: Type::Text,
            sql_column_name: field.name.as_ref().to_string(),
            graphql_field_name: field.name.as_ref().to_string(),
            graphql_type_name: field.name.as_ref().to_string(),
            graphql_order_by_asc: field.name.as_ref().to_string(),
            graphql_order_by_desc: field.name.as_ref().to_string(),
          }));
        })
        .collect::<Result<Vec<Rc<Field>>, ConversionError>>()
        .unwrap();

      return Ok(Entity::DatabaseTable {
        name: obj.name.as_ref().to_string(),
        sql_schema_name: obj.name.as_ref().to_string(),
        sql_table_name: obj.name.as_ref().to_string(),
        graphql_entity_type_name: obj.name.as_ref().to_string(),
        graphql_filter_type_name: obj.name.as_ref().to_string(),
        graphql_get_single_operation_name: obj.name.as_ref().to_string(),
        graphql_get_list_operation_name: obj.name.as_ref().to_string(),
        graphql_get_connection_operation_name: obj.name.as_ref().to_string(),
        graphql_default_order_by: obj.name.as_ref().to_string(),
        graphql_default_first: 0,
        graphql_default_offset: 10,
        fields: fields,
      });
    }
    TypeDefinition::Interface(_interface_type) => {
      return Err(ConversionError::UnsupportedSyntax("Interface".to_string()))
    }
    TypeDefinition::Union(_union_type) => {
      return Err(ConversionError::UnsupportedSyntax("Union".to_string()))
    }
    TypeDefinition::Enum(_enum_type) => {
      return Err(ConversionError::UnsupportedSyntax("Enum".to_string()))
    }
    TypeDefinition::InputObject(_input_object_type) => {
      return Err(ConversionError::UnsupportedSyntax(
        "InputObject".to_string(),
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
