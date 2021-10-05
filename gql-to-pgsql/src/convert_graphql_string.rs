use super::conversion_error::ConversionError;
use super::convert_document::convert_document;
use graphql_parser::query::{parse_query, Text};
use rtg_model_cache::model_cache::ModelCache;

pub fn convert_graphql_string<'a, T: Text<'a>>(
  string: &'a str,
  context: &ModelCache,
) -> Result<String, ConversionError> {
  let document = parse_query::<'a, T>(string)?;
  return convert_document(&document, context);
}

// Tests
#[cfg(test)]
mod tests {
  // use super::*;
  #[test]
  fn it_works() {
    assert_eq!(1, 1);
  }
}
