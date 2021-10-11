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
  use rtg_model::model_cache::model_cache::ModelCache;
  use similar_asserts::assert_eq;
  use std::rc::Rc;

  #[test]
  fn it_works() {
    let data = r#"
    {
      "version": "v1",
      "entities": [
        {
          "type": "databaseTable",
          "name": "person",
          "sqlSchemaName": "public",
          "sqlTableName": "person_table",
          "graphqlEntityTypeName": "Person",
          "graphqlFilterTypeName": "PersonWhereFilter",
          "graphqlGetSingleOperationName": "person",
          "graphqlGetListOperationName": "persons",
          "graphqlGetConnectionOperationName": "personConnection",
          "graphqlDefaultOrderBy": "id_ASC",
          "graphqlDefaultFirst": 10,
          "graphqlDefaultOffset": 0,
          "fields": [
            {
              "type": "scalarDatabaseColumn",
              "name": "id",
              "sqlType": "text",
              "sqlColumnName": "id_col",
              "graphqlFieldName": "id",
              "graphqlTypeName": "String",
              "graphqlOrderByAsc": "id_ASC",
              "graphqlOrderByDesc": "id_DESC"
            },
            {
              "type": "scalarDatabaseColumn",
              "name": "drone",
              "sqlType": "text",
              "sqlColumnName": "drone_col",
              "graphqlFieldName": "drone",
              "graphqlTypeName": "String",
              "graphqlOrderByAsc": "drone_ASC",
              "graphqlOrderByDesc": "drone_DESC"
            }
          ]
        }
      ]
    }
    "#;

    let model = Rc::new(serde_json::from_str(&data).unwrap());
    let model_cache = ModelCache::new(Rc::clone(&model));
    let sql_query = convert_graphql_string::convert_graphql_string(
      r#"
      query toto {
        yoyi: persons {
          a: id
          b: drone
        }
      }
      "#,
      &model_cache,
    )
    .unwrap();
    assert_eq!(sql_query, "select json_build_object('yoyi',(select coalesce((select json_agg(__rtg_9__.\"__rtg_10__\") from (select json_build_object('a',__rtg_11__.\"id_col\",'b',__rtg_11__.\"drone_col\") as __rtg_10__ from \"person_table\" as __rtg_11__ limit 10) as __rtg_9__),'[]'::json))) as __rtg_0__");
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
