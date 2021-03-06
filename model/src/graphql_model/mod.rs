pub use graphql_parser::schema::{parse_schema, Document, ParseError};

// Tests
#[cfg(test)]
mod tests {

  use super::*;
  use insta::assert_debug_snapshot;

  #[test]
  fn it_works<'a>() {
    let model = parse_schema::<'a, &'a str>(
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
