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
      type toto {
        a: id
        b: int
      }
      "#,
    )
    .unwrap();
    assert_debug_snapshot!(model);
  }
}

// Test the code in the readme file
// See https://github.com/rust-lang/cargo/issues/383#issuecomment-720873790
#[cfg(doctest)]
mod test_readme {
  macro_rules! external_doc_test {
    ($x:expr) => {
      #[doc = $x]
      extern "C" {}
    };
  }

  external_doc_test!(include_str!("../README.md"));
}
