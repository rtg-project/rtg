use super::conversion_error::ConversionError;
use super::convert_query_selection::convert_query_selection;
use graphql_parser::query::{SelectionSet, Text};
use rtg_model_cache::model_cache::ModelCache;

pub fn convert_query_selection_set<'a, T: Text<'a>>(
  query_selection_set: &SelectionSet<'a, T>,
  context: &ModelCache,
) -> Result<String, ConversionError> {
  let sql_alias_main_result = "__rtg_0__".to_string();

  let sql_field_sequence = match query_selection_set
    .items
    .iter()
    .map(|item| convert_query_selection(item, context))
    .collect::<Result<Vec<String>, ConversionError>>()
  {
    Ok(sql_field_items) => sql_field_items.join(","),
    Err(err) => return Err(err),
  };

  return Ok(format!(
    "select json_build_object({sql_field_sequence}) as {sql_alias_main_result}",
    sql_field_sequence = sql_field_sequence,
    sql_alias_main_result = sql_alias_main_result
  ));
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
