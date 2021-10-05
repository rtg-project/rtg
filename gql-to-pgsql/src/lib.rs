pub mod conversion_error;
pub mod convert_document;
pub mod convert_graphql_string;
pub mod convert_query;
pub mod convert_query_selection;
pub mod convert_query_selection_set;
pub mod convert_selection;
pub mod convert_selection_set;

// Tests
#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn it_works() {
    let sql_query = convert_graphql_string::convert_graphql_string(
      r#"
      query toto {
        countries {
          name
          cities {
            name
            population
          }
        }
      }
      "#,
    )
    .unwrap();
    assert_eq!(sql_query, "toto SELECT to_json(__local0__.\"id\") AS \"the_id\" FROM SELECT __local0__.* FROM \"public\".\"User\" AS __local0__ WHERE true LIMIT 10 OFFSET 0");
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
