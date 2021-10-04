use super::conversion_error::ConversionError;
use graphql_parser::query::{Selection, Text};
use rtg_model::field::Field;
use rtg_model_cache::entity_cache::EntityCache;
use rtg_model_cache::field_cache::FieldCache;
use scooby::postgres::Aliasable;
use std::ops::Deref;

pub fn convert_selection<'a, T: Text<'a>>(
  selection: &Selection<'a, T>,
  sql_parent_name: &str,
  context: &EntityCache,
) -> Result<String, ConversionError> {
  match selection {
    Selection::Field(field) => {
      let graphql_aliased_name = match field.alias {
        Some(alias) => alias,
        None => field.name,
      };
      let sql_column_name = match context {
        EntityCache::DatabaseTable {
          fields_by_graphql_field_name,
          ..
        } => match (*fields_by_graphql_field_name).get(field.name.as_ref()) {
          Some(field_cache) => match (*field_cache).deref() {
            FieldCache::ScalarDatabaseColumn { field } => match *field.deref() {
              Field::ScalarDatabaseColumn {
                sql_column_name, ..
              } => sql_column_name,
            },
          },
          None => {
            return Err(ConversionError::FieldNotFound(
              field.name.as_ref().to_string(),
            ))
          }
        },
      };
      return Ok(
        format!("to_json({}.\"{}\")", sql_parent_name, sql_column_name)
          .as_(&format!("\"{}\"", graphql_aliased_name.as_ref())[..])
          .to_string(),
      );
    }
    Selection::FragmentSpread(fragment_spread) => {
      return Err(ConversionError::Unsupported("FragmentSpread".to_string()))
    }
    Selection::InlineFragment(inline_fragment) => {
      return Err(ConversionError::Unsupported("InlineFragment".to_string()))
    }
  }
}
