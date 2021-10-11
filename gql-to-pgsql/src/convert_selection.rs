use super::conversion_error::ConversionError;
use graphql_parser::query::{Selection, Text};
use rtg_model::explicit_model::ExplicitField;
use rtg_model::model_cache::entity_cache::EntityCache;
use rtg_model::model_cache::field_cache::FieldCache;
use std::ops::Deref;

pub fn convert_selection<'a, T: Text<'a>>(
  selection: &Selection<'a, T>,
  context: &EntityCache,
  sql_parent_name: &str,
) -> Result<String, ConversionError> {
  match selection {
    Selection::Field(field) => {
      let graphql_aliased_name = match &field.alias {
        Some(alias) => alias,
        None => &field.name,
      };

      if field.selection_set.items.len() == 0 {
        let sql_column_name = match context {
          EntityCache::DatabaseTable {
            fields_by_graphql_field_name,
            ..
          } => match (*fields_by_graphql_field_name).get(field.name.as_ref()) {
            Some(field_cache) => match (*field_cache).deref() {
              FieldCache::ScalarDatabaseColumn { field } => match &*field.deref() {
                ExplicitField::ScalarDatabaseColumn {
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
        // // For root objects in Graphile it's done this way:
        // return Ok(
        //   format!("to_json({}.\"{}\")", sql_parent_name, sql_column_name)
        //     .as_(&format!("\"{}\"", graphql_aliased_name.as_ref())[..])
        //     .to_string(),
        // );
        // Generally it's done this way:
        return Ok(
          format!(
            "'{graphql_aliased_name}',{sql_parent_name}.\"{sql_column_name}\"",
            graphql_aliased_name = graphql_aliased_name.as_ref(),
            sql_parent_name = sql_parent_name,
            sql_column_name = sql_column_name
          )
          .to_string(),
        );
      } else {
        return Err(ConversionError::Generic(
          "Deeply nest fields are not supported yet".to_string(),
        ));
      }
    }
    Selection::FragmentSpread(_fragment_spread) => {
      return Err(ConversionError::UnsupportedSyntax(
        "FragmentSpread".to_string(),
      ))
    }
    Selection::InlineFragment(_inline_fragment) => {
      return Err(ConversionError::UnsupportedSyntax(
        "InlineFragment".to_string(),
      ))
    }
  }
}
