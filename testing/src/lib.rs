// Tests
#[cfg(test)]
mod tests {
  // use super::*;
  use glob::glob;
  use rtg_gql_to_pgsql::convert_graphql_string::convert_graphql_string;
  use rtg_model::model::Model;
  use rtg_model_cache::model_cache::ModelCache;
  use serde::Deserialize;
  use similar::{ChangeTag, TextDiff};
  use similar_asserts::assert_eq;
  use std::error::Error;
  use std::fs;
  use std::fs::{File, OpenOptions};
  use std::io::{BufReader, BufWriter};
  use std::path::Path;
  use std::rc::Rc;

  #[test]
  fn run_suites() {
    for entry in glob("../tests/*/model.json").expect("Failed to read glob for test cases") {
      let path = entry.unwrap();
      let dir_path = path.parent().unwrap();
      // println!("{:?}", dir_path.display());

      // Read the model from file
      let model_path = dir_path.join("model.json");
      let model_file = File::open(model_path).unwrap();
      let model_reader = BufReader::new(model_file);
      let model: Model = serde_json::from_reader(model_reader).unwrap();

      // Convert the model to a model cache
      let model_rc = Rc::new(model);
      let model_cache_inferred = ModelCache::new(Rc::clone(&model_rc));

      // Serialize the model cache (inferred)
      let model_cache_inferred_str = serde_json::to_string(&model_cache_inferred).unwrap();

      // Read existing model cache (truth) from disk
      let model_cache_truth_path = dir_path.join("model-cache.json");
      let model_cache_truth_file = File::open(model_cache_truth_path.clone());

      // Retrieve the model cache from file (if it exists, else write it)
      let model_cache_truth_str = match model_cache_truth_file {
        Result::Ok(model_cache_truth_file) => {
          // The model file exists already, read it for later comparison
          let model_cache_truth_reader = BufReader::new(model_cache_truth_file);
          let model_cache_truth: ModelCache =
            serde_json::from_reader(model_cache_truth_reader).unwrap();
          let model_cache_truth_str = serde_json::to_string(&model_cache_truth).unwrap();
          model_cache_truth_str
        }
        Result::Err(_e) => {
          // The model file does not exist, write it, the later comparison will be trivial
          let model_cache_inferred_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(model_cache_truth_path)
            .unwrap();
          let model_cache_inferred_writer = BufWriter::new(model_cache_inferred_file);
          serde_json::to_writer_pretty(model_cache_inferred_writer, &model_cache_inferred).unwrap();
          model_cache_inferred_str.clone()
        }
      };

      // Compare the model cache (inferred) to the model cache (truth)
      assert_eq!(model_cache_truth_str, model_cache_inferred_str);

      // Read and test all GraphQL queries on this model
      for entry in glob(dir_path.join("*.graphql").to_str().unwrap())
        .expect("Failed to read glob pattern for test queries")
      {
        // Read GraphQL query file
        let graphql_query_path = entry.unwrap();
        let graphql_query_string = fs::read_to_string(graphql_query_path.clone()).unwrap();

        // Convert the GraphQL query to a SQL query, using the model cache
        let sql_query_string =
          convert_graphql_string(&graphql_query_string[..], &model_cache_inferred).unwrap();

        // Wite SQL query file result
        let sql_query_path = graphql_query_path.with_extension("sql");
        fs::write(sql_query_path, sql_query_string).expect("Unable to write resulting sql file");
      }
    }
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
