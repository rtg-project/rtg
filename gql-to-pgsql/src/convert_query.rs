use super::conversion_error::ConversionError;
use super::convert_query_selection_set::convert_query_selection_set;

use graphql_parser::query::{Query, Text};
use rtg_model_cache::model_cache::ModelCache;

pub fn convert_query<'a, T: Text<'a>>(
  query: &Query<'a, T>,
  context: &ModelCache,
) -> Result<String, ConversionError> {
  let query_selection_set = &query.selection_set;
  return convert_query_selection_set(&query_selection_set, context);
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
