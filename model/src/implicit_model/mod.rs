pub use crate::explicit_model::entity::ImplicitEntity;
pub use crate::explicit_model::field::ImplicitField;
pub use crate::explicit_model::model::ImplicitModel;
pub use crate::explicit_model::sql_type::Type;

pub mod json_schema;

// Tests
#[cfg(test)]
mod tests {
  use super::*;
  use insta::assert_json_snapshot;
  use std::rc::Rc;

  #[test]
  fn serialize() {
    let value = ImplicitModel::V1 {
      entities: Some(vec![Rc::new(ImplicitEntity::DatabaseTable {
        name: Some("person".to_string()),
        sql_schema_name: Some("public".to_string()),
        sql_table_name: Some("person_table".to_string()),
        graphql_entity_type_name: Some("Person".to_string()),
        graphql_filter_type_name: Some("PersonWhereFilter".to_string()),
        graphql_get_single_operation_name: None,
        graphql_get_list_operation_name: None,
        graphql_get_connection_operation_name: Some("personConnection".to_string()),
        graphql_default_order_by: Some("id_ASC".to_string()),
        graphql_default_first: Some(10),
        graphql_default_offset: Some(0),
        fields: Some(vec![
          Rc::new(ImplicitField::ScalarDatabaseColumn {
            name: Some("id".to_string()),
            sql_type: Some(Type::Text),
            sql_column_name: Some("id_col".to_string()),
            graphql_field_name: Some("id".to_string()),
            graphql_type_name: Some("String".to_string()),
            graphql_order_by_asc: None,
            graphql_order_by_desc: None,
          }),
          Rc::new(ImplicitField::ScalarDatabaseColumn {
            name: Some("drone".to_string()),
            sql_type: Some(Type::Text),
            sql_column_name: Some("drone_col".to_string()),
            graphql_field_name: Some("drone".to_string()),
            graphql_type_name: Some("String".to_string()),
            graphql_order_by_asc: Some("drone_ASC".to_string()),
            graphql_order_by_desc: Some("drone_DESC".to_string()),
          }),
        ]),
      })]),
    };

    assert_json_snapshot!(serde_json::to_string_pretty(&value).unwrap());
  }
}
