use super::conversion_error::ConversionError;
use super::convert_selection::convert_selection;
use graphql_parser::query::{parse_query, Definition, OperationDefinition, SelectionSet, Text};
use scooby::postgres::{select, Aliasable, Joinable, Orderable, Parameters};

pub fn convert_selection_set<'a, T: Text<'a>>(
  selection_set: SelectionSet<'a, T>,
) -> Result<String, ConversionError> {
  selection_set
    .items
    .iter()
    .map(|item| convert_selection_item(item))
    .collect::<Result<Vec<String>, ConversionError>>()
    .map(|items| items.join(" "));
  Ok(format!("Ok"))
}

// Tests
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    assert_eq!(1, 1);
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
