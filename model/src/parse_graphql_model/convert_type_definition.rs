use super::conversion_error::ConversionError;
use super::convert_field::convert_field;
use graphql_parser::schema::{Text, TypeDefinition, Value};
// use rustc_hash::FxHashMap;
use crate::implicit_model::{ImplicitEntity, ImplicitField};
use std::rc::Rc;

pub fn convert_type_definition<'a, T: Text<'a>>(
  type_definition: &TypeDefinition<'a, T>,
  // relations_by_name: &FxHashMap<String, Rc<ImplicitEntity>>,
) -> Result<ImplicitEntity, ConversionError> {
  // relations_by_name.insert("s", "s");

  match type_definition {
    TypeDefinition::Scalar(_scalar_type) => {
      return Err(ConversionError::UnsupportedSyntax("Scalar".to_owned()))
    }
    TypeDefinition::Object(object) => {
      let name = Some(object.name.as_ref().to_owned());
      let mut sql_schema_name = None;
      let mut sql_table_name = None;
      let graphql_entity_type_name = None;
      let graphql_filter_type_name = None;
      let graphql_get_single_operation_name = None;
      let graphql_get_list_operation_name = None;
      let graphql_get_connection_operation_name = None;
      let graphql_default_order_by = None;
      let graphql_default_first = None;
      let graphql_default_offset = None;

      let fields = Some(
        object
          .fields
          .iter()
          .map(|field| convert_field(field, object))
          .collect::<Result<Vec<Rc<ImplicitField>>, ConversionError>>()
          .unwrap(),
      );

      for directive in object.directives.iter() {
        match directive.name.as_ref() {
          "sql" => {
            for argument in directive.arguments.iter() {
              match (*argument).0.as_ref() {
                "schema" => match &(*argument).1 {
                  Value::String(s) => sql_schema_name = Some(s.to_owned()),
                  _ => return Err(ConversionError::SqlDirectiveNameArgument),
                },
                "table" => match &(*argument).1 {
                  Value::String(s) => sql_table_name = Some(s.to_owned()),
                  _ => return Err(ConversionError::SqlDirectiveNameArgument),
                },
                argument_name => {
                  return Err(ConversionError::SqlDirectiveArgument(
                    argument_name.to_owned(),
                  ))
                }
              }
            }
          }
          _ => return Err(ConversionError::Unknown),
        }
      }

      return Ok(ImplicitEntity::DatabaseTable {
        name,
        sql_schema_name,
        sql_table_name,
        graphql_entity_type_name,
        graphql_filter_type_name,
        graphql_get_single_operation_name,
        graphql_get_list_operation_name,
        graphql_get_connection_operation_name,
        graphql_default_order_by,
        graphql_default_first,
        graphql_default_offset,
        fields,
      });
    }
    TypeDefinition::Interface(_interface_type) => {
      return Err(ConversionError::UnsupportedSyntax("Interface".to_owned()))
    }
    TypeDefinition::Union(_union_type) => {
      return Err(ConversionError::UnsupportedSyntax("Union".to_owned()))
    }
    TypeDefinition::Enum(_enum_type) => {
      return Err(ConversionError::UnsupportedSyntax("Enum".to_owned()))
    }
    TypeDefinition::InputObject(_input_object_type) => {
      return Err(ConversionError::UnsupportedSyntax("InputObject".to_owned()))
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
