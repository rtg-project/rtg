// Tests
#[cfg(test)]
mod tests {
  // use super::*;
  use glob::glob;
  // use rtg_model::graphql_model::parse_schema;
  use rtg_graphql_schema::generate_schema;
  use rtg_model::model_cache::model_cache::ModelCache;
  use rtg_model::{make_explicit_model, parse_graphql_model};
  use rtg_query::convert_graphql_string::convert_graphql_string;
  use rtg_sql_schema::extrospect_schema;
  use similar_asserts::assert_eq;
  use std::fs::{read_to_string, write};
  use std::rc::Rc;

  fn assert_matches_file(input_content: &str, file_path: &str) {
    let file_content = read_to_string(file_path);
    match file_content {
      Ok(file_content) => {
        assert_eq!(file_content, input_content);
        return;
      }
      Err(_e) => {
        let write_result = write(file_path, input_content);
        match write_result {
          Ok(_) => {
            return;
          }
          Err(e) => {
            panic!("Error writing file `{}`: {}", file_path, e);
          }
        }
      }
    }
  }

  #[test]
  fn run_integration_tests() {
    let mut entries = glob("./tests/only[0-9]*/model.graphql");

    entries = if entries.expect("Failed to read glob for test cases").count() > 0 {
      glob("./tests/only[0-9]*/model.graphql")
    } else {
      glob("./tests/[0-9]*/model.graphql")
    };
    for entry in entries.expect("Failed to read glob for test cases") {
      let entry_path = entry.unwrap();
      let dir_path = entry_path.parent().unwrap();

      // Read the graphql model file
      let graphql_model_string = read_to_string(dir_path.join("model.graphql")).unwrap();

      // Transforms into implicit model and check it
      let implicit_model = parse_graphql_model::convert_graphql_string::convert_graphql_string(
        &graphql_model_string[..],
      )
      .unwrap();
      let implicit_model_string = serde_json::to_string_pretty(&implicit_model).unwrap();
      assert_matches_file(
        implicit_model_string.as_str(),
        dir_path.join("implicit-model.json").to_str().unwrap(),
      );

      // Transforms into explicit model and check it
      let explicit_model =
        make_explicit_model::convert_model::convert_model(&implicit_model).unwrap();
      let explicit_model_string = serde_json::to_string_pretty(&implicit_model).unwrap();
      assert_matches_file(
        explicit_model_string.as_str(),
        dir_path.join("explicit-model.json").to_str().unwrap(),
      );

      // Transforms into sql schema and check it
      let sql_schema_string =
        extrospect_schema::convert_model::convert_model(&explicit_model).unwrap();
      assert_matches_file(
        sql_schema_string.as_str(),
        dir_path.join("schema.sql").to_str().unwrap(),
      );

      // Transforms into sql schema and check it
      let graphql_schema_string =
        generate_schema::convert_model::convert_model(&explicit_model).unwrap();
      assert_matches_file(
        format!("{}", graphql_schema_string).as_str(),
        dir_path.join("schema.graphql").to_str().unwrap(),
      );

      // Transforms into model cache and check it
      let model_rc = Rc::new(explicit_model);
      let model_cache = ModelCache::new(Rc::clone(&model_rc));
      let model_cache_string = serde_json::to_string_pretty(&model_cache).unwrap();
      assert_matches_file(
        model_cache_string.as_str(),
        dir_path.join("model-cache.json").to_str().unwrap(),
      );

      // Read and test all GraphQL queries on this model
      for entry in glob(dir_path.join("query-*.graphql").to_str().unwrap())
        .expect("Failed to read glob pattern for test queries")
      {
        // Read GraphQL query file
        let graphql_query_path = entry.unwrap();
        let graphql_query_string = read_to_string(graphql_query_path.clone()).unwrap();

        // Convert the GraphQL query to a SQL query, using the model cache
        let sql_query_string_inferred =
          convert_graphql_string(&graphql_query_string[..], &model_cache).unwrap();

        // Retrieve the query from file (if it exists, else write it)
        let sql_query_path = graphql_query_path.with_extension("sql");
        let sql_query_string_truth = match read_to_string(sql_query_path.clone()) {
          Result::Ok(sql_query_string_truth) => sql_query_string_truth,
          Result::Err(_e) => {
            // Write SQL query file result
            write(sql_query_path, sql_query_string_inferred.clone())
              .expect("Unable to write resulting sql file");
            sql_query_string_inferred.clone()
          }
        };

        // Compare the model cache (inferred) to the model cache (truth)
        assert_eq!(sql_query_string_truth, sql_query_string_inferred);
      }
    }
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
