use super::conversion_error::ConversionError;
use super::convert_document::convert_document;
use graphql_parser::query::parse_query;
use rtg_model::model_cache::model_cache::ModelCache;

pub fn convert_graphql_string<'a>(
  string: &'a str,
  context: &ModelCache,
) -> Result<String, ConversionError> {
  let document = parse_query::<'a, &'a str>(string)?;
  return convert_document(&document, context);
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
