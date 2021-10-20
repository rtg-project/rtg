use super::conversion_error::ConversionError;
use super::convert_field::convert_field;
use rtg_model::explicit_model::ExplicitEntity;

pub fn convert_entity(entity: &ExplicitEntity) -> Result<String, ConversionError> {
  match entity {
    ExplicitEntity::DatabaseTable {
      sql_schema_name,
      sql_table_name,
      fields,
      ..
    } => {
      let sql_column_sequence = match fields
        .iter()
        .map(|field| convert_field(field))
        .collect::<Result<Vec<String>, ConversionError>>()
      {
        Ok(sql_column_items) => sql_column_items.join(",\n"),
        Err(err) => return Err(err),
      };

      Ok(format!(
        "create table \"{}\".\"{}\" (
{}
);",
        sql_schema_name, sql_table_name, sql_column_sequence
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
