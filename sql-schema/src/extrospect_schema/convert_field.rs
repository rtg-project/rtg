use super::conversion_error::ConversionError;
use rtg_model::explicit_model::ExplicitField;

pub fn convert_field(field: &ExplicitField) -> Result<String, ConversionError> {
  match field {
    ExplicitField::ScalarDatabaseColumn {
      sql_type,
      sql_column_name,
      nullable,
      ..
    } => Ok(format!(
      "  \"{}\" {}{}",
      sql_column_name,
      sql_type,
      if *nullable { "" } else { " not null" }
    )),
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
