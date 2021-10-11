use super::conversion_error::ConversionError;
use super::convert_selection_set::convert_selection_set;
use graphql_parser::query::{Selection, Text};
use rtg_model::model_cache::model_cache::{EntityOperationCache, ModelCache, OperationKind};

pub fn convert_query_selection<'a, T: Text<'a>>(
  selection: &Selection<'a, T>,
  context: &ModelCache,
) -> Result<String, ConversionError> {
  match selection {
    Selection::Field(field) => {
      let graphql_aliased_name = match &field.alias {
        Some(alias) => alias,
        None => &field.name,
      };

      let sql_sub_query = match context {
        ModelCache::V1 {
          entities_by_operation_name,
          ..
        } => match (*entities_by_operation_name).get(field.name.as_ref()) {
          Some(field_cache) => match &*field_cache {
            EntityOperationCache {
              entity_cache,
              operation_kind,
            } => match operation_kind {
              OperationKind::GetList => {
                let sql_local_result_name = "__rtg_9__".to_string();
                let sql_local_result_name_field = "__rtg_10__".to_string();
                match convert_selection_set(
                  &field.selection_set,
                  &entity_cache,
                  &sql_local_result_name_field,
                ) {
                  Ok(sql_selection_set_query) => format!(
                    "(select coalesce((select json_agg({sql_local_result_name}.\"{sql_local_result_name_field}\") from ({sql_selection_set_query}) as {sql_local_result_name}),'[]'::json))",
                    sql_local_result_name=sql_local_result_name,
                    sql_local_result_name_field=sql_local_result_name_field,
                    sql_selection_set_query=sql_selection_set_query,
                  )
                  .to_string(),
                  Err(error) => return Err(error),
                }
              }
              OperationKind::GetSingle => {
                return Err(ConversionError::UnsupportedOperation(
                  field.name.as_ref().to_string(),
                  "GetSingle".to_string(),
                ))
              }
              OperationKind::GetConnection => {
                return Err(ConversionError::UnsupportedOperation(
                  field.name.as_ref().to_string(),
                  "GetConnection".to_string(),
                ))
              }
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
      //   format!("to_json({}.\"{}\")", sql_parent_name, sql_sub_query)
      //     .as_(&format!("\"{}\"", graphql_aliased_name.as_ref())[..])
      //     .to_string(),
      // );
      // Generally it's done this way:
      return Ok(
        format!(
          "'{graphql_aliased_name}',{sql_sub_query}",
          graphql_aliased_name = graphql_aliased_name.as_ref(),
          sql_sub_query = sql_sub_query
        )
        .to_string(),
      );
    }
    Selection::FragmentSpread(_fragment_spread) => {
      return Err(ConversionError::UnsupportedSyntax(
        "QueryFragmentSpread".to_string(),
      ))
    }
    Selection::InlineFragment(_inline_fragment) => {
      return Err(ConversionError::UnsupportedSyntax(
        "QueryInlineFragment".to_string(),
      ))
    }
  }

  // match context {
  //   ModelCache::V1 {
  //     entities_by_operation_name,
  //     ..
  //   } => {
  //     let mut query_string = String::new();
  //     for operation in query.operations() {
  //       let entity = entities_by_operation_name
  //         .get(&operation.name.value)
  //         .ok_or_else(|| ConversionError::UnknownOperation(operation.name.value.to_string()))?;
  //       let selection = convert_selection(operation.selection, entity, context)?;
  //       query_string.push_str(&format!("{} {}", operation.name.value, selection));
  //     }
  //     Ok(query_string)
  //   }

  //   }
  // }

  // return Err(ConversionError::UnsupportedSyntax("Query".to_string()));
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
