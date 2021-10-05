use super::conversion_error::ConversionError;
use super::convert_selection::convert_selection;
use graphql_parser::query::{SelectionSet, Text};
use rtg_model::entity::Entity;
use rtg_model_cache::entity_cache::EntityCache;
use std::ops::Deref;

pub fn convert_selection_set<'a, T: Text<'a>>(
  selection_set: &SelectionSet<'a, T>,
  context: &EntityCache,
  sql_parent_name: &str,
) -> Result<String, ConversionError> {
  let sql_alias_main_table = "__rtg_11__".to_string();
  let sql_limit = 10;

  let sql_from_table_name = match &*context {
    EntityCache::DatabaseTable { entity, .. } => match entity.deref() {
      Entity::DatabaseTable { sql_table_name, .. } => sql_table_name.to_string(),
    },
  };

  let sql_field_sequence = match selection_set
    .items
    .iter()
    .map(|item| convert_selection(item, context, &sql_alias_main_table))
    .collect::<Result<Vec<String>, ConversionError>>()
  {
    Ok(sql_field_items) => sql_field_items.join(","),
    Err(err) => return Err(err),
  };

  return Ok(
    format!(
      "select json_build_object({sql_field_sequence}) as {sql_parent_name} \
      from \"{sql_from_table_name}\" as {sql_alias_main_table} limit {sql_limit}",
      sql_field_sequence = sql_field_sequence,
      sql_parent_name = sql_parent_name,
      sql_from_table_name = sql_from_table_name,
      sql_alias_main_table = sql_alias_main_table,
      sql_limit = sql_limit
    )
    .to_string(),
  );
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
