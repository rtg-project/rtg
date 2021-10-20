// Tests
#[cfg(test)]
mod tests {
  // use super::*;
  use glob::glob;
  // use rtg_model::graphql_model::parse_schema;
  use rtg_model::model_cache::model_cache::ModelCache;
  use rtg_model::{make_explicit_model, parse_graphql_model};
  use rtg_query::convert_graphql_string::convert_graphql_string;
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
    print!("===================Starting\n");
    for entry in entries.expect("Failed to read glob for test cases") {
      let entry_path = entry.unwrap();
      print!("Ok ------------- {:?}", entry_path);
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

      // Transforms into model cache and check it
      let model_rc = Rc::new(explicit_model);
      let model_cache = ModelCache::new(Rc::clone(&model_rc));
      let model_cache_string = serde_json::to_string_pretty(&model_cache).unwrap();
      assert_matches_file(
        model_cache_string.as_str(),
        dir_path.join("model-cache.json").to_str().unwrap(),
      );

      // Read the model from file
      // let implicit_model_path = dir_path.join("implicit-model.json");
      // let implicit_model_string = read_to_string(implicit_model_path).unwrap();
      // let implicit_model = serde_json::from_str(implicit_model_string.as_str());

      // let model: ExplicitModel = serde_json::from_reader(model_reader).unwrap();

      // // Convert the model to a model cache
      // let model_rc = Rc::new(model);
      // let model_cache = ModelCache::new(Rc::clone(&model_rc));

      // // Serialize the model cache (inferred)
      // let model_cache_inferred_string = serde_json::to_string(&model_cache_inferred).unwrap();

      // // Retrieve the model cache (truth) from file (if it exists, else write it)
      // let model_cache_truth_path = dir_path.join("model-cache.json");
      // let model_cache_truth_file = File::open(model_cache_truth_path.clone());
      // let model_cache_truth_string = match model_cache_truth_file {
      //   Result::Ok(model_cache_truth_file) => {
      //     // The model file exists already, read it for later comparison
      //     let model_cache_truth_reader = BufReader::new(model_cache_truth_file);
      //     let model_cache_truth: ModelCache =
      //       serde_json::from_reader(model_cache_truth_reader).unwrap();
      //     let model_cache_truth_string = serde_json::to_string(&model_cache_truth).unwrap();
      //     model_cache_truth_string
      //   }
      //   Result::Err(_e) => {
      //     // The model file does not exist, write it, the later comparison will be trivial
      //     let model_cache_inferred_file = OpenOptions::new()
      //       .append(true)
      //       .create(true)
      //       .open(model_cache_truth_path)
      //       .unwrap();
      //     let model_cache_inferred_writer = BufWriter::new(model_cache_inferred_file);
      //     serde_json::to_writer_pretty(model_cache_inferred_writer, &model_cache_inferred).unwrap();
      //     model_cache_inferred_string.clone()
      //   }
      // };

      // Compare the model cache (inferred) to the model cache (truth)
      // assert_eq!(model_cache_truth_string, model_cache_inferred_string);

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
