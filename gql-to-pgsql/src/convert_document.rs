use super::conversion_error::ConversionError;
use super::convert_selection_set::convert_selection_set;
use graphql_parser::query::{parse_query, Definition, OperationDefinition};

pub fn convert_document(query: &str) -> Result<String, ConversionError> {
  let ast = parse_query::<&str>(query)?;

  if ast.definitions.len() != 1 {
    return Err(ConversionError::DefinitionNumber);
  }

  match ast.definitions.first() {
    Some(def) => match *def {
      Definition::Operation(ref op) => match *op {
        OperationDefinition::SelectionSet(ref set) => {
          return convert_selection_set(*set);
        }
        OperationDefinition::Query(ref q) => return convert_selection_set((*q).selection_set),
        OperationDefinition::Mutation(ref _mut) => {
          return Err(ConversionError::Unsupported("Mutation".to_string()))
        }
        OperationDefinition::Subscription(ref _sub) => {
          return Err(ConversionError::Unsupported("Subscription".to_string()))
        }
      },
      Definition::Fragment(ref _frag) => {
        return Err(ConversionError::Unsupported("Fragment".to_string()))
      }
    },
    None => return Err(ConversionError::DefinitionNumber),
  };
}

// Tests
#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn it_works() {
    let sql_query = convert_document(
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
