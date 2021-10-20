use super::conversion_error::ConversionError;
use super::convert_entity::convert_entity;
use graphql_parser::schema::{Definition, Document};
use rtg_model::explicit_model::ExplicitModel;

pub fn convert_model(model: &ExplicitModel) -> Result<Document<String>, ConversionError> {
  match model {
    ExplicitModel::V1 { entities, .. } => {
      let graphql_types = match entities
        .iter()
        .map(|entity| convert_entity(entity))
        .collect::<Result<Vec<Definition<String>>, ConversionError>>()
      {
        Ok(graphql_types_items) => graphql_types_items,
        Err(err) => return Err(err),
      };

      Ok(Document {
        definitions: graphql_types,
      })
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
