pub mod conversion_error;
pub mod convert_document;
pub mod convert_selection;
pub mod convert_selection_set;
use conversion_error::ConversionError;
use graphql_parser::query::{parse_query, Definition, OperationDefinition};
use scooby::postgres::{select, Aliasable, Joinable, Orderable, Parameters};

pub fn convert_gql_to_sql(query: &str) -> Result<String, ConversionError> {
  let ast = parse_query::<&str>(query)?;

  if ast.definitions.len() != 1 {
    return Err(ConversionError::DefinitionNumber);
  }

  let graphql_query_name = match ast.definitions.first() {
    Some(def) => match *def {
      Definition::Operation(ref op) => match *op {
        OperationDefinition::SelectionSet(ref _set) => {
          return Err(ConversionError::Unsupported("SelectionSet".to_string()))
        }
        OperationDefinition::Query(ref q) => match (*q).name {
          Some(ref name) => name.clone(),
          None => return Err(ConversionError::Unsupported("No".to_string())),
        },
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

  // write sql query
  let mut sql_params = Parameters::new();

  let local0 = "__local0__".to_string();
  let schema_name = "public".to_string();
  let table_name = "User".to_string();
  let condition = "true".to_string();
  let field0_sql_column_name = "id".to_string();
  let field0_graphql_aliased_name = "the_id".to_string();

  let sql_query = select(
    format!("to_json({}.\"{}\")", local0, field0_sql_column_name)
      .as_(&format!("\"{}\"", field0_graphql_aliased_name)[..]),
  )
  .from(
    select(format!("{}.*", local0))
      .from(format!("\"{}\".\"{}\"", schema_name, table_name).as_(&format!("{}", local0)[..]))
      .where_(condition)
      .limit(10)
      .offset(0)
      .to_string(),
  )
  .to_string();

  Ok(format!("{} {}", graphql_query_name, sql_query))
}

// Tests
#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn it_works() {
    let sql_query = convert_gql_to_sql(
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
