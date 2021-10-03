#![allow(missing_docs)]
#![allow(unused_variables)]

use graphql_parser::query::{parse_query, Definition, OperationDefinition, ParseError};
use handlebars::Handlebars;

use scooby::postgres::{select, Aliasable, Joinable, Orderable, Parameters};
use serde::Serialize;
// use serde_json::json;
use std::env;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
  // Read graphql query
  let graphql_query = "query MyQuery { field1, field2 }";
  let ast = match parse_query::<&str>(graphql_query) {
    Ok(ast) => ast,
    Err(e) => {
      alert(&format!("{}", e));
      return;
    }
  };
  let graphql_query_name = match ast.definitions.first() {
    Some(def) => match *def {
      Definition::Operation(ref op) => match *op {
        OperationDefinition::SelectionSet(ref set) => return,
        OperationDefinition::Query(ref q) => match (*q).name {
          Some(ref name) => name.clone(),
          None => return,
        },
        OperationDefinition::Mutation(ref m) => return,
        OperationDefinition::Subscription(ref s) => return,
      },
      Definition::Fragment(ref frag) => return,
    },
    None => return,
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

  // alert to js
  alert(&format!("Hello, {} , {}", sql_query, graphql_query_name));
}
