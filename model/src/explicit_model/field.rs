use super::sql_type;
use partial_struct::PartialStruct;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(PartialStruct, Serialize, Deserialize, Debug, JsonSchema)]
#[serde(tag = "type", rename_all = "camelCase")]
#[partial(name = "ImplicitField")]
#[partial_attribute(derive(Serialize, Deserialize, Debug, JsonSchema))]
#[partial_attribute(serde(tag = "type", rename_all = "camelCase"))]
pub enum ExplicitField {
  #[serde(rename_all = "camelCase")]
  ScalarDatabaseColumn {
    name: String,
    nullable: bool,
    // #[serde(flatten)]
    sql_type: sql_type::Type,
    sql_column_name: String,
    graphql_field_name: String,
    graphql_type_name: String,
    graphql_order_by_asc: String,
    graphql_order_by_desc: String,
  },
}

// Tests
#[cfg(test)]
mod tests {
  use super::*;
  use similar_asserts::assert_eq;

  #[test]
  fn serialize() {
    let value = ExplicitField::ScalarDatabaseColumn {
      name: "drone".to_owned(),
      nullable: false,
      sql_type: sql_type::Type::Text,
      sql_column_name: "drone_col".to_owned(),
      graphql_field_name: "drone".to_owned(),
      graphql_type_name: "String".to_owned(),
      graphql_order_by_asc: "drone_ASC".to_owned(),
      graphql_order_by_desc: "drone_DESC".to_owned(),
    };

    match serde_json::to_string_pretty(&value) {
      Ok(string) => {
        assert_eq!(
          string,
          r#"{
  "type": "scalarDatabaseColumn",
  "name": "drone",
  "nullable": false,
  "sqlType": "text",
  "sqlColumnName": "drone_col",
  "graphqlFieldName": "drone",
  "graphqlTypeName": "String",
  "graphqlOrderByAsc": "drone_ASC",
  "graphqlOrderByDesc": "drone_DESC"
}"#
        );
      }
      Err(e) => panic!("{}", e),
    }
  }

  #[test]
  fn serialize_other() {
    let value = ExplicitField::ScalarDatabaseColumn {
      name: "drone".to_owned(),
      nullable: false,
      sql_type: sql_type::Type::Other("Yoo".to_owned()),
      sql_column_name: "drone_col".to_owned(),
      graphql_field_name: "drone".to_owned(),
      graphql_type_name: "String".to_owned(),
      graphql_order_by_asc: "drone_ASC".to_owned(),
      graphql_order_by_desc: "drone_DESC".to_owned(),
    };

    match serde_json::to_string_pretty(&value) {
      Ok(string) => {
        assert_eq!(
          string,
          r#"{
  "type": "scalarDatabaseColumn",
  "name": "drone",
  "nullable": false,
  "sqlType": {
    "other": "Yoo"
  },
  "sqlColumnName": "drone_col",
  "graphqlFieldName": "drone",
  "graphqlTypeName": "String",
  "graphqlOrderByAsc": "drone_ASC",
  "graphqlOrderByDesc": "drone_DESC"
}"#
        );
      }
      Err(e) => panic!("{}", e),
    }
  }

  #[test]
  fn deserialize() {
    let data = r#"
      {
        "name": "id",
        "nullable": false,
        "type": "scalarDatabaseColumn",
        "sqlType": "text",
        "sqlColumnName": "id",
        "graphqlFieldName": "id",
        "graphqlTypeName": "String",
        "graphqlOrderByAsc": "idAsc",
        "graphqlOrderByDesc": "idDesc"
      }
      "#;

    match serde_json::from_str(data) {
      Ok(field) => match field {
        ExplicitField::ScalarDatabaseColumn {
          name,
          nullable,
          sql_type,
          sql_column_name,
          graphql_field_name: _,
          graphql_type_name: _,
          graphql_order_by_asc: _,
          graphql_order_by_desc: _,
        } => {
          assert_eq!(name, "id");
          assert_eq!(nullable, false);
          assert_eq!(sql_type, sql_type::Type::Text);
          assert_eq!(sql_column_name, "id");
        }
      },
      Err(e) => panic!("{}", e),
    }
  }

  #[test]
  fn deserialize_other() {
    let data = r#"
      {
        "name": "id",
        "type": "scalarDatabaseColumn",
        "nullable": true,
        "sqlType": {
          "other": "Hii"
        },
        "sqlColumnName": "id",
        "graphqlFieldName": "id",
        "graphqlTypeName": "String",
        "graphqlOrderByAsc": "idAsc",
        "graphqlOrderByDesc": "idDesc"
      }
      "#;

    match serde_json::from_str(data) {
      Ok(field) => match field {
        ExplicitField::ScalarDatabaseColumn {
          name,
          nullable,
          sql_type,
          sql_column_name,
          graphql_field_name: _,
          graphql_type_name: _,
          graphql_order_by_asc: _,
          graphql_order_by_desc: _,
        } => {
          assert_eq!(name, "id");
          assert_eq!(nullable, true);
          assert_eq!(sql_type, sql_type::Type::Other("Hii".to_owned()));
          assert_eq!(sql_column_name, "id");
        }
      },
      Err(e) => panic!("{}", e),
    }
  }
}
