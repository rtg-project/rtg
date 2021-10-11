pub mod conversion_error;
pub mod convert_document;
pub mod convert_graphql_string;
pub mod convert_type_definition;

// Tests
#[cfg(test)]
mod tests {

  use super::*;
  use insta::assert_debug_snapshot;

  #[test]
  fn it_works() {
    let model = convert_graphql_string::convert_graphql_string(
      r#"
      type Person {
        id: ID @id
        age: Int
      }
      "#,
    )
    .unwrap();
    assert_debug_snapshot!(model);
  }
}
