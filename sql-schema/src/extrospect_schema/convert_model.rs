use super::conversion_error::ConversionError;
use super::convert_entity::convert_entity;
use rtg_model::explicit_model::ExplicitModel;

pub fn convert_model(model: &ExplicitModel) -> Result<String, ConversionError> {
  match model {
    ExplicitModel::V1 { entities, .. } => {
      let sql_table_sequence = match entities
        .iter()
        .map(|entity| convert_entity(entity))
        .collect::<Result<Vec<String>, ConversionError>>()
      {
        Ok(sql_entity_items) => sql_entity_items.join("\n\n"),
        Err(err) => return Err(err),
      };

      Ok(format!("{}", sql_table_sequence))
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
