use super::field::{ExplicitField, ImplicitField};
use partial_struct::PartialStruct;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(PartialStruct, Serialize, Deserialize, Debug, JsonSchema)]
#[serde(tag = "type", rename_all = "camelCase")]
#[partial(name = "ImplicitEntity")]
#[partial_attribute(derive(Serialize, Deserialize, Debug, JsonSchema))]
#[partial_attribute(serde(tag = "type", rename_all = "camelCase"))]
pub enum ExplicitEntity {
  #[serde(rename_all = "camelCase")]
  DatabaseTable {
    name: String,
    sql_schema_name: String,
    sql_table_name: String,
    graphql_entity_type_name: String,
    graphql_filter_type_name: String,
    graphql_get_single_operation_name: String,
    graphql_get_list_operation_name: String,
    graphql_get_connection_operation_name: String,
    graphql_default_order_by: String,
    graphql_default_first: i16,
    graphql_default_offset: i16,
    #[partial(nested_type = "Vec<Rc<ImplicitField>>")]
    fields: Vec<Rc<ExplicitField>>,
  },
}

// Tests
#[cfg(test)]
mod tests {
  use super::*;
  use crate::explicit_model::sql_type;
  use similar_asserts::assert_eq;

  #[test]
  fn serialize() {
    let value = ExplicitEntity::DatabaseTable {
      name: "person".to_owned(),
      sql_schema_name: "public".to_owned(),
      sql_table_name: "person_table".to_owned(),
      graphql_entity_type_name: "Person".to_owned(),
      graphql_filter_type_name: "PersonWhereFilter".to_owned(),
      graphql_get_single_operation_name: "person".to_owned(),
      graphql_get_list_operation_name: "persons".to_owned(),
      graphql_get_connection_operation_name: "personConnection".to_owned(),
      graphql_default_order_by: "id_ASC".to_owned(),
      graphql_default_first: 10,
      graphql_default_offset: 0,
      fields: vec![
        Rc::new(ExplicitField::ScalarDatabaseColumn {
          name: "id".to_owned(),
          nullable: false,
          sql_type: sql_type::Type::Text,
          sql_column_name: "id_col".to_owned(),
          graphql_field_name: "id".to_owned(),
          graphql_type_name: "String".to_owned(),
          graphql_order_by_asc: "id_ASC".to_owned(),
          graphql_order_by_desc: "id_DESC".to_owned(),
        }),
        Rc::new(ExplicitField::ScalarDatabaseColumn {
          name: "drone".to_owned(),
          nullable: false,
          sql_type: sql_type::Type::Text,
          sql_column_name: "drone_col".to_owned(),
          graphql_field_name: "drone".to_owned(),
          graphql_type_name: "String".to_owned(),
          graphql_order_by_asc: "drone_ASC".to_owned(),
          graphql_order_by_desc: "drone_DESC".to_owned(),
        }),
      ],
    };

    match serde_json::to_string_pretty(&value) {
      Ok(string) => {
        assert_eq!(
          string,
          r#"{
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
      "nullable": false,
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
      "nullable": false,
      "sqlType": "text",
      "sqlColumnName": "drone_col",
      "graphqlFieldName": "drone",
      "graphqlTypeName": "String",
      "graphqlOrderByAsc": "drone_ASC",
      "graphqlOrderByDesc": "drone_DESC"
    }
  ]
}"#
        );
      }
      Err(e) => panic!("{}", e),
    }
  }
}
