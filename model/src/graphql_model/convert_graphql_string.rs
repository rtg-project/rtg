use super::conversion_error::ConversionError;
use super::convert_document::convert_document;
use crate::explicit_model::ExplicitModel;
use graphql_parser::schema::parse_schema;

pub fn convert_graphql_string<'a>(string: &'a str) -> Result<ExplicitModel, ConversionError> {
  let document = parse_schema::<'a, &'a str>(string)?;
  return convert_document(&document);
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
