use super::conversion_error::ConversionError;
use graphql_parser::schema::{Field, ObjectType, Text, Type};

// use rustc_hash::FxHashMap;
use crate::implicit_model::ImplicitField;
use std::rc::Rc;

pub fn convert_field<'a, T: Text<'a>>(
  field: &Field<'a, T>,
  object: &ObjectType<'a, T>,
  // relations_by_name: &FxHashMap<String, Rc<ImplicitEntity>>,
) -> Result<Rc<ImplicitField>, ConversionError> {
  // relations_by_name.insert("s", "s");

  let name = Some(field.name.as_ref().to_string());

  match &field.field_type {
    Type::NamedType(type_name) => {
      let graphql_type_name = Some(type_name.as_ref().to_string());
      let sql_type = None;
      let sql_column_name = None;
      let graphql_field_name = None;
      let graphql_order_by_asc = None;
      let graphql_order_by_desc = None;
      for directive in field.directives.iter() {
        match directive.name.as_ref() {
          "sql" => {
            for argument in directive.arguments.iter() {
              match (*argument).0.as_ref() {
                "type" => return Err(ConversionError::Unknown),
                &_ => return Err(ConversionError::Unknown),
              }
            }
          }
          &_ => {
            return Err(ConversionError::UnsupportedDirective(
              directive.name.as_ref().to_string(),
            ))
          }
        }
      }
      return Ok(Rc::new(ImplicitField::ScalarDatabaseColumn {
        name,
        nullable: Some(true),
        sql_type,
        sql_column_name,
        graphql_field_name,
        graphql_type_name,
        graphql_order_by_asc,
        graphql_order_by_desc,
      }));
    }
    Type::ListType(_item_type) => {
      return Err(ConversionError::NullableArrayFieldType(
        object.name.as_ref().to_string(),
        field.name.as_ref().to_string(),
      ));
    }
    Type::NonNullType(item_type) => match &**item_type {
      Type::NamedType(type_name) => {
        let graphql_type_name = Some(type_name.as_ref().to_string());
        let sql_type = None;
        let sql_column_name = None;
        let graphql_field_name = None;
        let graphql_order_by_asc = None;
        let graphql_order_by_desc = None;
        return Ok(Rc::new(ImplicitField::ScalarDatabaseColumn {
          name,
          nullable: Some(false),
          sql_type,
          sql_column_name,
          graphql_field_name,
          graphql_type_name,
          graphql_order_by_asc,
          graphql_order_by_desc,
        }));
      }
      Type::ListType(_item_type) => {
        return Err(ConversionError::NonSupportedArray(
          object.name.as_ref().to_string(),
          field.name.as_ref().to_string(),
        ));
      }
      Type::NonNullType(_item_type) => {
        return Err(ConversionError::MultipleNonNullFieldType(
          object.name.as_ref().to_string(),
          field.name.as_ref().to_string(),
        ));
      }
    },
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
