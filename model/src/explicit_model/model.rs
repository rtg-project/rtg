use super::entity::{ExplicitEntity, ImplicitEntity};
use partial_struct::PartialStruct;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(PartialStruct, Serialize, Deserialize, Debug, JsonSchema)]
#[serde(tag = "version", rename_all = "camelCase")]
#[partial(name = "ImplicitModel")]
#[partial_attribute(derive(Serialize, Deserialize, Debug, JsonSchema))]
#[partial_attribute(serde(tag = "version", rename_all = "camelCase"))]
pub enum ExplicitModel {
  #[serde(rename_all = "camelCase")]
  V1 {
    #[partial(nested_type = "Vec<Rc<ImplicitEntity>>")]
    entities: Vec<Rc<ExplicitEntity>>,
  },
}

// Tests
#[cfg(test)]
mod tests {
  use super::*;
  use crate::explicit_model::field::ExplicitField;
  use crate::explicit_model::sql_type;
  use insta::assert_json_snapshot;

  #[test]
  fn serialize() {
    let value = ExplicitModel::V1 {
      entities: vec![Rc::new(ExplicitEntity::DatabaseTable {
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
            graphql_enabled: true,
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
            graphql_enabled: true,
            graphql_field_name: "drone".to_owned(),
            graphql_type_name: "String".to_owned(),
            graphql_order_by_asc: "drone_ASC".to_owned(),
            graphql_order_by_desc: "drone_DESC".to_owned(),
          }),
        ],
      })],
    };

    assert_json_snapshot!(serde_json::to_string_pretty(&value).unwrap());
  }
}
