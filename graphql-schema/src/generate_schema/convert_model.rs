use super::conversion_error::ConversionError;
use super::convert_entity::convert_entity;
use graphql_parser::schema::{Definition, Document, ObjectType, TypeDefinition};
use graphql_parser::Pos;
use rtg_model::explicit_model::ExplicitModel;

pub fn convert_model(model: &ExplicitModel) -> Result<Document<String>, ConversionError> {
  match model {
    ExplicitModel::V1 { entities, .. } => {
      let mut query_fields = vec![];
      let mut graphql_types = vec![];

      for entity in entities.iter() {
        match convert_entity(entity) {
          Ok(entity_definition) => {
            graphql_types.push(entity_definition.0);
            query_fields.push(entity_definition.1);
            query_fields.push(entity_definition.2);
            query_fields.push(entity_definition.3);
          }
          Err(err) => return Err(err),
        }
      }

      let query_type = Definition::TypeDefinition(TypeDefinition::Object(ObjectType {
        position: Pos { line: 0, column: 0 },
        description: None,
        name: "Query".to_owned(),
        implements_interfaces: vec![],
        directives: vec![],
        fields: query_fields,
      }));

      graphql_types.push(query_type);

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
