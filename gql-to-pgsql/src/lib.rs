use graphql_parser::query::{parse_query, Definition, OperationDefinition, ParseError};
use scooby::postgres::{select, Aliasable, Joinable, Orderable, Parameters};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConvertError {
  #[error("GraphQL parse error")]
  GraphqlParse(#[from] ParseError),
  #[error("Document must contain one and only one definition")]
  DefinitionNumber,
  #[error("GraphQL syntax `{0}` is not supported yet")]
  Unsupported(String),
  #[error("Unknown convert error")]
  Unknown,
}

pub fn convert_gql_to_sql(query: &str) -> Result<String, ConvertError> {
  let ast = parse_query::<&str>(query)?;

  if ast.definitions.len() != 1 {
    return Err(ConvertError::DefinitionNumber);
  }

  let graphql_query_name = match ast.definitions.first() {
    Some(def) => match *def {
      Definition::Operation(ref op) => match *op {
        OperationDefinition::SelectionSet(ref set) => {
          return Err(ConvertError::Unsupported("SelectionSet".to_string()))
        }
        OperationDefinition::Query(ref q) => match (*q).name {
          Some(ref name) => name.clone(),
          None => return Err(ConvertError::Unsupported("No".to_string())),
        },
        OperationDefinition::Mutation(ref m) => {
          return Err(ConvertError::Unsupported("Mutation".to_string()))
        }
        OperationDefinition::Subscription(ref s) => {
          return Err(ConvertError::Unsupported("Subscription".to_string()))
        }
      },
      Definition::Fragment(ref _frag) => {
        return Err(ConvertError::Unsupported("Fragment".to_string()))
      }
    },
    None => return Err(ConvertError::DefinitionNumber),
  };

  // write sql query
  let mut sql_params = Parameters::new();
  let sql_query = select(("country.name".as_("name"), "COUNT(*)".as_("count")))
    .from(
      "Country"
        .as_("country")
        .inner_join("City".as_("city"))
        .on("city.country_id = country.id"),
    )
    .where_(format!("city.population > {}", sql_params.next()))
    .group_by("country.name")
    .order_by("count".desc())
    .limit(10)
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
    assert_eq!(sql_query, "toto SELECT country.name AS name, COUNT(*) AS count FROM Country AS country INNER JOIN City AS city ON city.country_id = country.id WHERE city.population > $1 GROUP BY country.name ORDER BY count DESC LIMIT 10");
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
