use super::conversion_error::ConversionError;
use super::convert_type_definition::convert_type_definition;
use crate::implicit_model::{ImplicitEntity, ImplicitModel};
use graphql_parser::schema::{Definition, Document, Text};

// use rustc_hash::FxHashMap;
use std::rc::Rc;

pub fn convert_document<'a, T: Text<'a>>(
  document: &Document<'a, T>,
) -> Result<ImplicitModel, ConversionError> {
  // let mut relations_by_name = FxHashMap::default();

  // relations_by_name.insert("s", "s");

  let entities = match document
    .definitions
    .iter()
    .map(|definition| match definition {
      Definition::DirectiveDefinition(ref _dd) => {
        return Err(ConversionError::UnsupportedSyntax(
          "DirectiveDefinition".to_string(),
        ))
      }
      Definition::SchemaDefinition(ref _sd) => {
        return Err(ConversionError::UnsupportedSyntax(
          "SchemaDefinition".to_string(),
        ))
      }
      Definition::TypeDefinition(ref type_definition) => {
        return Ok(Rc::new(convert_type_definition(type_definition).unwrap()));
        // return Err(ConversionError::UnsupportedSyntax(
        //   "TypeDefinition".to_string(),
        // ))
      }
      Definition::TypeExtension(ref _te) => {
        return Err(ConversionError::UnsupportedSyntax(
          "TypeExtension".to_string(),
        ))
      }
    })
    .collect::<Result<Vec<Rc<ImplicitEntity>>, ConversionError>>()
  {
    Ok(entities) => entities,
    Err(err) => return Err(err),
  };

  return Ok(ImplicitModel::V1 {
    entities: Some(entities),
  });
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
