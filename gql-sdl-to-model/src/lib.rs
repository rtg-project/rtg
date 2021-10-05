pub mod conversion_error;
pub mod convert_document;
pub mod convert_graphql_string;

// Tests
#[cfg(test)]
mod tests {

  use similar_asserts::assert_eq;

  #[test]
  fn it_works() {
    assert_eq!(3 + 2, 5);
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
